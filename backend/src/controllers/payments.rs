use {
    crate::{
        models::{
            response::ResponseAPI,
            stripe::{
                CurrencyMap, PayloadCreacteProduct, PaymentPayload, PaymentResponse, PricePayload,
                ProductPayload, RelationalCalStripe,
            },
        },
        services::payments::insert_options_by_country,
        state::AppState,
    },
    axum::{
        Extension, Json, debug_handler,
        extract::{Path, State},
        http::StatusCode,
        response::IntoResponse,
    },
    serde_json::json,
    std::{collections::HashMap, str::FromStr, sync::Arc},
    stripe::{
        CreatePaymentIntent, CreatePaymentIntentAutomaticPaymentMethods, CreateProduct,
        CreateProductDefaultPriceData, CreateProductDefaultPriceDataCurrencyOptions, Currency,
        Expandable, List, ListPrices, ListProducts, PaymentIntent, Price, PriceId, Product,
        ProductId, StripeError, UpdatePrice, UpdateProduct,
    },
    tracing::instrument,
};

// Comprar precios genericos
#[debug_handler]
#[instrument(
    skip(state, payload),
    fields(
        amount = %payload.amount,
        currency = %payload.currency,
        operation = "payment_intent"
    )
)]
pub async fn payment_intent(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PaymentPayload>,
) -> impl IntoResponse {
    tracing::debug!(
        "[payment_intent] payload: amount={}, currency={}",
        payload.amount,
        payload.currency
    );

    // Validar el monto mínimo (ejemplo: $5.00 USD = 500 centavos)
    if payload.amount < 500 {
        tracing::warn!("[payment_intent] Monto menor al mínimo: {}", payload.amount);
        return (
            StatusCode::BAD_REQUEST,
            Json(ResponseAPI::<()>::error(
                "El monto mínimo es 5.00€".to_string(),
            )),
        )
            .into_response();
    }

    // Crear el PaymentIntent
    let currency: Currency =
        Currency::from_str(&payload.currency.to_string()).unwrap_or(Currency::EUR);
    tracing::debug!("[payment_intent] Usando currency: {:?}", currency);

    let payment_intent = CreatePaymentIntent {
        amount: payload.amount,
        currency,
        payment_method: None,
        payment_method_types: None,
        confirm: Some(false),
        return_url: None,
        automatic_payment_methods: Some(CreatePaymentIntentAutomaticPaymentMethods {
            enabled: true,
            allow_redirects: Some(
                stripe::CreatePaymentIntentAutomaticPaymentMethodsAllowRedirects::Never,
            ),
        }),
        capture_method: None,
        confirmation_method: None,
        customer: None,
        description: None,
        error_on_requires_action: None,
        expand: &[],
        mandate: None,
        mandate_data: None,
        metadata: None,
        off_session: None,
        on_behalf_of: None,
        payment_method_configuration: None,
        payment_method_data: None,
        payment_method_options: None,
        radar_options: None,
        receipt_email: None,
        setup_future_usage: None,
        shipping: None,
        statement_descriptor: None,
        statement_descriptor_suffix: None,
        transfer_data: None,
        transfer_group: None,
        use_stripe_sdk: None,
        application_fee_amount: None,
    };

    tracing::debug!("[payment_intent] Creando PaymentIntent con Stripe...");
    match PaymentIntent::create(&state.stripe_client, payment_intent).await {
        Ok(payment_intent) => {
            tracing::info!(
                "[payment_intent] PaymentIntent creado: id={}, status={:?}",
                payment_intent.id,
                payment_intent.status
            );

            // Extraer el client_secret
            let client_secret = match payment_intent.client_secret {
                Some(secret) => secret,
                None => {
                    tracing::error!("[payment_intent] No se recibió client_secret");
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ResponseAPI::<()>::error("Error interno".to_string())),
                    )
                        .into_response();
                }
            };

            // Con confirm=false, el status DEBE ser RequiresPaymentMethod
            // Esto es correcto y esperado
            (
                StatusCode::OK,
                Json(ResponseAPI::success(
                    "PaymentIntent creado".to_string(),
                    PaymentResponse {
                        client_secret: Some(client_secret),
                        status: format!("{:?}", payment_intent.status),
                        error: None,
                    },
                )),
            )
                .into_response()
        }
        Err(stripe_error) => {
            tracing::error!(
                error = ?stripe_error,
                "Stripe API error"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(
                    "Error processing payment".to_string(),
                )),
            )
                .into_response()
        }
    }
}

// Consultar un payment para saber su estado actual
pub async fn get_payment_status(// Path(id): Path<String>,
    // State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

// Cancelar un payment
pub async fn cancel_payment(// Path(id): Path<String>,
    // State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

// Devolver el pago
pub async fn refund_payment(// Path(id): Path<String>,
    // State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

// Consultar el historial de pagos del usuario
pub async fn get_payment_history() -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

// Crear un producto
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
        recurring: match price.recurring {
            Some(recurring) => Some(recurring),
            None => None, // Si no es recurrente no pasa nada
        },
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

// Obtener toda la lista de productos
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

// Obtener todos los precios
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

// Eliminar un producto
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
        .and_then(|p: &Expandable<Price>| Some(p.id()))
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

// Archivar precio
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

// Estructura para recibir la relacion entre cal y stripe
pub async fn archive_cal_connection(
    State(state): State<Arc<AppState>>,
    Extension(id_token): Extension<String>,
    Json(payload): Json<RelationalCalStripe>,
) -> impl IntoResponse {
    let url_firebase_db: String = format!(
        "{}/relation_cal_stripe/{}.json?auth={}",
        state.firebase_options.firebase_database_url, payload.cal_id, id_token
    );

    tracing::info!(
        "💾 Saving Cal-Stripe relation: {} -> {}",
        payload.cal_id,
        payload.stripe_id
    );

    match state
        .firebase_options
        .firebase_client
        .put(&url_firebase_db)
        .json(&json!({
            "stripe_id": payload.stripe_id
        }))
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                tracing::info!("Relation saved successfully");
                let mut data = HashMap::new();
                data.insert("cal_id".to_string(), payload.cal_id.clone());
                data.insert("stripe_id".to_string(), payload.stripe_id.clone());
                (
                    StatusCode::OK,
                    Json(ResponseAPI::<HashMap<String, String>>::success(
                        "Relation saved successfully".to_string(),
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
                        "Failed to save relation".to_string(),
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
