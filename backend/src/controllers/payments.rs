use {
    crate::{
        controllers::cal::{add_seat_to_booking, confirm_booking_by_uid, fetch_booking},
        models::{
            cal::{BookingStatus, CalBookingPayload},
            firebase::UserAuthentication,
            response::ResponseAPI,
            state::AppState,
            stripe::{
                BookingPayment, CheckoutPayload, CheckoutResponse, CurrencyMap,
                PayloadCreacteProduct, PaymentIntentSimplified, PricePayload, ProductPayload,
            },
        },
        services::payments::{
            PaidClass, booking_class_slug, booking_has_attendee, checkout_amount_cents,
            insert_options_by_country, is_safe_id, payment_intent_ids_from_relation,
            relation_has_payment_from, secure_eq,
        },
    },
    axum::{
        Extension, Json, debug_handler,
        extract::{Path, State},
        http::{HeaderMap, StatusCode},
        response::{IntoResponse, Response},
    },
    serde_json::{Value, json},
    std::{collections::HashMap, str::FromStr, sync::Arc},
    stripe::{
        CreatePaymentIntent, CreatePaymentIntentAutomaticPaymentMethods,
        CreatePaymentIntentAutomaticPaymentMethodsAllowRedirects, CreateProduct,
        CreateProductDefaultPriceData, CreateProductDefaultPriceDataCurrencyOptions, Currency,
        Expandable, List, ListPaymentIntents, ListPrices, ListProducts, PaymentIntent,
        PaymentIntentId, PaymentIntentStatus, Price, PriceId, Product, ProductId, StripeError,
        UpdatePrice, UpdateProduct,
    },
    tracing::instrument,
};

fn error_response(status: StatusCode, message: &str) -> Response {
    (status, Json(ResponseAPI::<()>::error(message.to_string()))).into_response()
}

async fn resolve_checkout_class(
    state: &AppState,
    payload: &CheckoutPayload,
    email: &str,
) -> Result<(PaidClass, Option<String>), (StatusCode, &'static str)> {
    match (payload.booking_uid.as_deref(), payload.weeks) {
        (Some(uid), None) => {
            if !is_safe_id(uid) {
                return Err((StatusCode::BAD_REQUEST, "Invalid booking id"));
            }

            let booking: CalBookingPayload = fetch_booking(state, uid).await.map_err(|e| {
                tracing::error!("[payment_intent] Error fetching booking {}: {}", uid, e);
                (StatusCode::BAD_GATEWAY, "Could not retrieve the booking")
            })?;

            if !matches!(
                booking.status,
                BookingStatus::Pending | BookingStatus::Accepted
            ) {
                return Err((StatusCode::CONFLICT, "This booking can no longer be paid"));
            }

            let class: PaidClass = booking_class_slug(&booking)
                .as_deref()
                .and_then(PaidClass::from_slug)
                .ok_or_else(|| {
                    tracing::warn!("[payment_intent] Booking {} has no paid class type", uid);
                    (StatusCode::BAD_REQUEST, "This class type cannot be paid")
                })?;

            let relation: Value = fetch_booking_relation(state, uid).await.map_err(|e| {
                tracing::error!("[payment_intent] Error reading payments of {}: {}", uid, e);
                (StatusCode::BAD_GATEWAY, "Could not verify the booking")
            })?;

            let already_paid: bool = match class {
                PaidClass::Group => relation_has_payment_from(&relation, email),
                _ => !payment_intent_ids_from_relation(&relation).is_empty(),
            };
            if already_paid {
                return Err((StatusCode::CONFLICT, "This booking is already paid"));
            }

            Ok((class, Some(uid.to_string())))
        }
        (None, Some(_)) => payload
            .class_type
            .as_deref()
            .and_then(PaidClass::from_slug)
            .map(|class| (class, None))
            .ok_or((StatusCode::BAD_REQUEST, "Invalid class type")),
        _ => Err((
            StatusCode::BAD_REQUEST,
            "Either booking_uid or weeks must be provided",
        )),
    }
}

#[debug_handler]
#[instrument(
    skip(state, user_claims, headers, payload),
    fields(user_id = %user_claims.sub, operation = "payment_intent")
)]
pub async fn payment_intent(
    State(state): State<Arc<AppState>>,
    Extension(user_claims): Extension<UserAuthentication>,
    headers: HeaderMap,
    Json(payload): Json<CheckoutPayload>,
) -> impl IntoResponse {
    let Some(internal_secret) = state.payment_options.internal_api_secret.as_deref() else {
        tracing::error!("[payment_intent] INTERNAL_API_SECRET is not configured");
        return error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            "Payments are not configured",
        );
    };

    let provided_secret: &str = headers
        .get("x-internal-secret")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    if !secure_eq(provided_secret, internal_secret) {
        tracing::warn!("[payment_intent] Request without a valid internal secret");
        return error_response(StatusCode::FORBIDDEN, "Forbidden");
    }

    let Some(email) = user_claims.email.clone().filter(|email| !email.is_empty()) else {
        return error_response(StatusCode::BAD_REQUEST, "User email is required");
    };

    let (class, booking_uid) = match resolve_checkout_class(&state, &payload, &email).await {
        Ok(resolved) => resolved,
        Err((status, message)) => return error_response(status, message),
    };

    let amount: i64 = match checkout_amount_cents(class, &payload.prices, payload.weeks) {
        Ok(amount) => amount,
        Err(e) => {
            tracing::error!("[payment_intent] Invalid amount: {}", e);
            return error_response(StatusCode::BAD_REQUEST, "Invalid price");
        }
    };

    let mut metadata: HashMap<String, String> = HashMap::from([
        ("class_type".to_string(), class.slug().to_string()),
        ("user_uid".to_string(), user_claims.sub.clone()),
        ("user_email".to_string(), email.clone()),
    ]);
    if let Some(name) = user_claims
        .name
        .as_ref()
        .filter(|name| !name.trim().is_empty())
    {
        metadata.insert("user_name".to_string(), name.clone());
    }
    if let Some(uid) = &booking_uid {
        metadata.insert("booking_uid".to_string(), uid.clone());
    }
    if let Some(weeks) = payload.weeks {
        metadata.insert("weeks".to_string(), weeks.to_string());
    }

    let description: String = match (&booking_uid, payload.weeks) {
        (Some(uid), _) => format!("{} - booking {}", class.slug(), uid),
        (None, Some(weeks)) => format!("{} - {} weeks", class.slug(), weeks),
        (None, None) => class.slug().to_string(),
    };

    let mut params: CreatePaymentIntent = CreatePaymentIntent::new(amount, Currency::EUR);
    params.automatic_payment_methods = Some(CreatePaymentIntentAutomaticPaymentMethods {
        enabled: true,
        allow_redirects: Some(CreatePaymentIntentAutomaticPaymentMethodsAllowRedirects::Never),
    });
    params.metadata = Some(metadata);
    params.receipt_email = Some(&email);
    params.description = Some(&description);

    match PaymentIntent::create(&state.stripe_client, params).await {
        Ok(payment_intent) => {
            tracing::info!(
                "[payment_intent] PaymentIntent created: id={}, amount={}, class={}",
                payment_intent.id,
                amount,
                class.slug()
            );

            let Some(client_secret) = payment_intent.client_secret else {
                tracing::error!("[payment_intent] Missing client_secret");
                return error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal error");
            };

            (
                StatusCode::OK,
                Json(ResponseAPI::success(
                    "PaymentIntent created".to_string(),
                    CheckoutResponse {
                        client_secret,
                        amount,
                        currency: "eur".to_string(),
                    },
                )),
            )
                .into_response()
        }
        Err(stripe_error) => {
            tracing::error!(error = ?stripe_error, "Stripe API error");
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error processing payment",
            )
        }
    }
}

/// Obtener historial de pagos de un usuario
#[debug_handler]
#[instrument(skip(state), fields(operation = "get_payment_history"))]
pub async fn get_payment_history(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut params: ListPaymentIntents = ListPaymentIntents::new();
    params.limit = Some(100);

    match PaymentIntent::list(&state.stripe_client, &params).await {
        Ok(payment_intents) => {
            // Convertir a tipo simplificado
            let user_payments: Vec<PaymentIntentSimplified> = payment_intents
                .data
                .into_iter()
                .map(PaymentIntentSimplified::from)
                .collect();

            (
                StatusCode::OK,
                Json(ResponseAPI::success(
                    "Payment history retrieved successfully".to_string(),
                    user_payments,
                )),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("Stripe error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(format!(
                    "Failed to retrieve payment history: {}",
                    e
                ))),
            )
                .into_response()
        }
    }
}

/// Crear un producto
#[debug_handler]
#[instrument(
    skip(state, payload),
    fields(
        product_name = %payload.product.name,
        price_amount = %payload.price.unit_amount,
        price_currency = %payload.price.currency,
        operation = "create_product"
    )
)]
pub async fn create_product(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PayloadCreacteProduct>,
) -> impl IntoResponse {
    let product: ProductPayload = ProductPayload {
        name: payload.product.name.clone(),
        description: payload.product.description.clone(),
        images: payload.product.images.clone(),
        metadata: payload.product.metadata.clone(),
        active: payload.product.active,
    };

    let price: PricePayload = PricePayload {
        currency: payload.price.currency.clone(),
        unit_amount: payload.price.unit_amount,
        recurring: payload.price.recurring.clone(),
    };

    // Monedas diferentes a la original EURO
    let mut currency_opts: CurrencyMap<CreateProductDefaultPriceDataCurrencyOptions> =
        HashMap::new();
    insert_options_by_country(&mut currency_opts, &price);

    // Creeamos un precio
    let pricing: CreateProductDefaultPriceData = CreateProductDefaultPriceData {
        // Obligatorios
        currency: match Currency::from_str(&price.currency.to_string()) {
            Ok(currency) => {
                if currency != Currency::EUR {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            "error": "Invalid currency, must be EUR"
                        })),
                    );
                }
                currency
            }
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Invalid currency"
                    })),
                );
            }
        },
        unit_amount: Some(price.unit_amount),
        // Si es recurrente
        recurring: price.recurring,
        // Opcionales
        currency_options: Some(currency_opts),
        tax_behavior: None,
        unit_amount_decimal: None,
    };

    // Creamos un nuevo producto con todos los datos que sabemos tanto del producto como del nuevo precio
    let new_product: CreateProduct = CreateProduct {
        name: &product.name,                     // Nombre del producto (obligatorio)
        description: Some(&product.description), // Descripción del producto (opcional)
        images: None,   // Lista de URLs de imágenes del producto (opcional)
        metadata: None, // Metadatos personalizados (opcional, útil para datos internos)
        active: Some(product.active), // Si el producto está activo o no (opcional)
        shippable: Some(false), // Indica si el producto se puede enviar físicamente (solo para productos físicos)
        unit_label: None, // Etiqueta personalizada para la unidad (opcional, ej: "mes", "clase")
        tax_code: None,   // Código de impuestos para Stripe Tax (opcional)
        url: Some("https://amanahacademia.com/pricing"), // URL pública del producto (opcional)
        expand: &[],      // Campos adicionales para expandir en la respuesta de Stripe (opcional)
        default_price_data: Some(pricing), // Datos para crear el precio por defecto del producto (no se puede usar junto con type_)
        features: None, // Lista de características del producto (opcional, solo para productos físicos)
        id: None,       // ID personalizado del producto (opcional, normalmente lo genera Stripe)
        package_dimensions: None, // Dimensiones del paquete (opcional, solo para productos físicos)
        statement_descriptor: None, // Descripción personalizada para el extracto bancario (opcional)
        type_: None, // Tipo de producto: "good" (físico) o "service" (servicio). No se puede usar junto con default_price_data
    };

    match Product::create(&state.stripe_client, new_product).await {
        Ok(product) => (
            StatusCode::CREATED,
            Json(json!({ "success": true, "product": product })),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json! ({"success": false, "error": format!("Error creating product: {}", err) })),
        ),
    }
}

/// Obtener toda la lista de productos
#[debug_handler]
#[instrument(skip(state), fields(operation = "get_all_products"))]
pub async fn get_all_products(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let params = ListProducts::default();
    let products: Result<List<Product>, StripeError> =
        Product::list(&state.stripe_client, &params).await;
    match products {
        Ok(products) => (
            StatusCode::OK,
            Json(json!({ "success": true, "products": products })),
        ),
        Err(_err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"success": false, "error": "Error fetching products" })),
        ),
    }
}

/// Obtener todos los precios
#[debug_handler]
#[instrument(skip(state), fields(operation = "get_all_prices"))]
pub async fn get_all_prices(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let params: ListPrices<'_> = ListPrices::default();
    let prices: Result<List<Price>, StripeError> = Price::list(&state.stripe_client, &params).await;
    match prices {
        Ok(prices) => (
            StatusCode::OK,
            Json(json!({ "success": true, "prices": prices })),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"success": false, "error": format!("Error fetching prices: {}", err) })),
        ),
    }
}

/// Eliminar un producto
#[debug_handler]
#[instrument(
    skip(state),
    fields(
        product_id = %id,
        operation = "archive_product"
    )
)]
pub async fn archive_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // ID del producto
    let product_id: ProductId = match ProductId::from_str(&id) {
        Ok(pid) => pid,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"success": false, "error": "Invalid product ID" })),
            );
        }
    };

    // Recuperar el producto
    let product: Product = match Product::retrieve(&state.stripe_client, &product_id, &[]).await {
        Ok(p) => p,
        Err(err) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": format!("Product not found: {}", err)})),
            );
        }
    };

    // Si existe default_price, necesitas manejarlo primero
    // Nota: No puedes archivar el precio por defecto directamente si está asociado como default_price
    if let Some(price_id) = product
        .default_price
        .as_ref()
        .map(|p: &Expandable<Price>| p.id())
    {
        // Primero, elimina el default_price del producto actualizando el producto
        // Construye UpdateProduct y elimina el default_price
        let mut update_product = stripe::UpdateProduct::new();
        update_product.default_price = None;
        match Product::update(&state.stripe_client, &product_id, update_product).await {
            Ok(_) => {
                // Ahora que el precio ya no es el predeterminado, puedes archivarlo
                let update_price = UpdatePrice {
                    active: Some(false), // Archiva el precio
                    ..Default::default()
                };
                if let Err(e) = Price::update(&state.stripe_client, &price_id, update_price).await {
                    // Maneja el error adecuadamente: quizás loguearlo y continuar o abortar
                    println!("Failed to archive price {}: {}", price_id, e);
                }
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("Failed to remove default price: {}", e)})),
                );
            }
        };
    }

    // Archivar el producto itself
    let update_product: UpdateProduct = UpdateProduct {
        active: Some(false), // Esto archiva el producto
        ..Default::default()
    };
    match Product::update(&state.stripe_client, &product_id, update_product).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({"success": true, "message": "Product archived successfully"})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to archive product: {}", e)})),
        ),
    }
}

/// Archivar precio
#[debug_handler]
#[instrument(
    skip(state),
    fields(
        price_id = %id,
        operation = "archive_price"
    )
)]
pub async fn delete_price(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let price_id: PriceId = match PriceId::from_str(&id) {
        Ok(pid) => pid,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"success": false, "error": "Invalid price ID" })),
            );
        }
    };
    let mut params: UpdatePrice<'_> = UpdatePrice::new();
    params.active = Some(false);
    match Price::update(&state.stripe_client, &price_id, params).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "success": true }))),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"success": false, "error": format!("Error archiving price: {}", err) })),
        ),
    }
}

fn relation_url(state: &AppState, path: &str) -> String {
    format!(
        "{}/relation_cal_stripe/{}.json?auth={}",
        state.firebase_options.firebase_database_url,
        path,
        state.firebase_options.firebase_database_secret
    )
}

async fn fetch_booking_relation(state: &AppState, booking_uid: &str) -> Result<Value, String> {
    let response = state
        .firebase_options
        .firebase_client
        .get(relation_url(state, booking_uid))
        .send()
        .await
        .map_err(|e| format!("Firebase request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Firebase error: {}", response.status()));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Invalid Firebase response: {}", e))
}

pub async fn get_booking_payment_ids(
    state: &AppState,
    booking_uid: &str,
) -> Result<Vec<String>, String> {
    let relation: Value = fetch_booking_relation(state, booking_uid).await?;
    Ok(payment_intent_ids_from_relation(&relation))
}

async fn put_relation_entry(state: &AppState, path: &str, entry: &Value) -> Result<(), String> {
    let response = state
        .firebase_options
        .firebase_client
        .put(relation_url(state, path))
        .json(entry)
        .send()
        .await
        .map_err(|e| format!("Firebase request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text: String = response.text().await.unwrap_or_default();
        return Err(format!("Firebase error: {}", error_text));
    }

    Ok(())
}

#[instrument(skip(state))]
pub async fn transfer_booking_payments(
    state: &AppState,
    from_uid: &str,
    to_uid: &str,
) -> Result<usize, String> {
    if !is_safe_id(from_uid) || !is_safe_id(to_uid) {
        return Err(format!("Invalid booking ids: {} -> {}", from_uid, to_uid));
    }

    let relation: Value = fetch_booking_relation(state, from_uid).await?;
    let payment_ids: Vec<String> = payment_intent_ids_from_relation(&relation);

    for payment_id in &payment_ids {
        let mut entry: Value = match relation.get(payment_id) {
            Some(Value::Object(entry)) => Value::Object(entry.clone()),
            _ => json!({}),
        };
        entry["rescheduled_from"] = json!(from_uid);
        put_relation_entry(state, &format!("{}/{}", to_uid, payment_id), &entry).await?;
    }

    tracing::info!(
        "Moved {} payment(s) from booking {} to rescheduled booking {}",
        payment_ids.len(),
        from_uid,
        to_uid
    );
    Ok(payment_ids.len())
}

async fn save_booking_payment(
    state: &AppState,
    booking_uid: &str,
    payment_intent_id: &str,
    payment: &BookingPayment,
) -> Result<(), String> {
    let entry: Value =
        serde_json::to_value(payment).map_err(|e| format!("Invalid payment entry: {}", e))?;
    put_relation_entry(
        state,
        &format!("{}/{}", booking_uid, payment_intent_id),
        &entry,
    )
    .await
}

#[instrument(skip(state))]
pub async fn fulfill_payment(state: &AppState, payment_intent_id: &str) -> Result<(), String> {
    let id: PaymentIntentId = payment_intent_id
        .parse()
        .map_err(|e| format!("Invalid PaymentIntent id: {}", e))?;
    let payment_intent: PaymentIntent = PaymentIntent::retrieve(&state.stripe_client, &id, &[])
        .await
        .map_err(|e| format!("Error retrieving PaymentIntent: {}", e))?;

    if payment_intent.status != PaymentIntentStatus::Succeeded {
        tracing::warn!(
            "PaymentIntent {} is {:?}, skipping fulfillment",
            payment_intent.id,
            payment_intent.status
        );
        return Ok(());
    }

    let Some(booking_uid) = payment_intent.metadata.get("booking_uid") else {
        tracing::info!(
            "PaymentIntent {} has no booking to fulfill (metadata: {:?})",
            payment_intent.id,
            payment_intent.metadata
        );
        return Ok(());
    };

    let class: PaidClass = payment_intent
        .metadata
        .get("class_type")
        .and_then(|slug| PaidClass::from_slug(slug))
        .ok_or_else(|| format!("PaymentIntent {} without class type", payment_intent.id))?;
    let email: &String = payment_intent
        .metadata
        .get("user_email")
        .ok_or_else(|| format!("PaymentIntent {} without user email", payment_intent.id))?;

    save_booking_payment(
        state,
        booking_uid,
        payment_intent.id.as_str(),
        &BookingPayment {
            email: email.clone(),
            amount: payment_intent.amount,
            class_type: class.slug().to_string(),
            created: payment_intent.created,
        },
    )
    .await?;

    let booking: CalBookingPayload = fetch_booking(state, booking_uid).await?;

    if class == PaidClass::Group && !booking_has_attendee(&booking, email) {
        let name: String = payment_intent
            .metadata
            .get("user_name")
            .cloned()
            .unwrap_or_else(|| email.split('@').next().unwrap_or(email).to_string());
        add_seat_to_booking(state, &booking, email, &name).await?;
    }

    if booking.status == BookingStatus::Pending {
        confirm_booking_by_uid(state, booking_uid).await?;
    }

    tracing::info!(
        "Payment {} fulfilled for booking {}",
        payment_intent.id,
        booking_uid
    );
    Ok(())
}

/// Obtener todas las reservas pagadas vinculadas a Cal.com
#[debug_handler]
#[instrument(skip(state), fields(operation = "get_all_paid_reservations"))]
pub async fn get_all_paid_reservations(
    State(state): State<Arc<AppState>>,
    Extension(id_token): Extension<String>,
) -> impl IntoResponse {
    let url_firebase_db: String = format!(
        "{}/relation_cal_stripe.json?auth={}",
        state.firebase_options.firebase_database_url, id_token
    );

    match state
        .firebase_options
        .firebase_client
        .get(&url_firebase_db)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let data: Value = match response.json::<Value>().await {
                    Ok(Value::Object(relations)) => Value::Object(relations),
                    _ => json!({}),
                };
                (
                    StatusCode::OK,
                    Json(ResponseAPI::success(
                        "Relation retrieved successfully".to_string(),
                        data,
                    )),
                )
                    .into_response()
            } else {
                let error_text: String = response.text().await.unwrap_or_default();
                tracing::error!("Firebase error: {}", error_text);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseAPI::<()>::error(
                        "Failed to get relations".to_string(),
                    )),
                )
                    .into_response()
            }
        }
        Err(e) => {
            tracing::error!("Request failed: {}", e);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(
                    "Failed to send request".to_string(),
                )),
            )
                .into_response()
        }
    }
}

///////// POSIBLES FUTURAS FUNCIONES /////////
// Consultar un payment para saber su estado actual
// pub async fn get_payment_status(
//     Path(id): Path<String>,
//     State(state): State<Arc<AppState>>,
// ) -> impl IntoResponse {
//     StatusCode::NO_CONTENT.into_response()
// }

// Cancelar un payment
// pub async fn cancel_payment(
//     Path(id): Path<String>,
//     State(state): State<Arc<AppState>>,
// ) -> impl IntoResponse {
//     StatusCode::NO_CONTENT.into_response()
// }

// Devolver el pago
// pub async fn refund_payment(
//     Path(id): Path<String>,
//     State(state): State<Arc<AppState>>,
// ) -> impl IntoResponse {
//     StatusCode::NO_CONTENT.into_response()
// }

#[cfg(test)]
#[path = "../test/controllers/payments.rs"]
mod extended_tests;
