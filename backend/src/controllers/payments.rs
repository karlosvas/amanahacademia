use {
    crate::{
        models::stripe::{
            CurrencyMap, PayloadCreacteProduct, PaymentPayload, PaymentResponse, PricePayload,
            ProductPayload,
        },
        services::payments::insert_options_by_country,
        state::AppState,
    },
    axum::{
        Json, debug_handler,
        extract::{Path, State},
        http::{HeaderMap, StatusCode},
        response::IntoResponse,
    },
    serde_json::json,
    std::{collections::HashMap, str::FromStr, sync::Arc},
    stripe::{
        CreatePaymentIntent, CreateProduct, CreateProductDefaultPriceData,
        CreateProductDefaultPriceDataCurrencyOptions, Currency, Expandable, List, ListPrices,
        ListProducts, PaymentIntent, PaymentIntentStatus, Price, PriceId, Product, ProductId,
        StripeError, UpdatePrice, UpdateProduct,
    },
};

async fn show_data(headers: &HeaderMap) {
    // Log estructurado
    println!("=== Payment Request Info ===");

    // User aggent
    if let Some(user_agent) = headers.get("user-agent").and_then(|v| v.to_str().ok()) {
        println!("User-Agent: {}", user_agent);
    }

    // Content-Type
    if let Some(content_type) = headers.get("content-type").and_then(|v| v.to_str().ok()) {
        println!("Content-Type: {}", content_type);
    }

    // IP real
    if let Some(ip) = headers.get("cf-connecting-ip") {
        println!("IP real del visitante: {:?}", ip.to_str().unwrap_or(""));
    }

    // País
    if let Some(country) = headers.get("cf-ipcountry") {
        println!("País: {:?}", country.to_str().unwrap_or(""));
    }

    // Ciudad
    if let Some(city) = headers.get("cf-city") {
        println!("Ciudad: {:?}", city.to_str().unwrap_or(""));
    }

    // Continente
    if let Some(continent) = headers.get("cf-continent") {
        println!("Continente: {:?}", continent.to_str().unwrap_or(""));
    }

    // Latitud y longitud
    if let Some(lat) = headers.get("cf-latitude") {
        println!("Latitud: {:?}", lat.to_str().unwrap_or(""));
    }

    // Longitud
    if let Some(lon) = headers.get("cf-longitude") {
        println!("Longitud: {:?}", lon.to_str().unwrap_or(""));
    }
    println!("========================");
}

// Comprar precios genericos
#[debug_handler]
pub async fn generic_payment(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<PaymentPayload>,
) -> impl IntoResponse {
    show_data(&headers).await;

    // Validar el monto mínimo (ejemplo: $5.00 USD = 500 centavos)
    if payload.amount < 500 {
        return Json(json!({
            "succes": false,
            "message": Some("El monto mínimo es 5.00€".to_string()),
        }));
    }

    // Crear el PaymentIntent
    let payment_intent: CreatePaymentIntent = CreatePaymentIntent {
        amount: payload.amount,
        currency: Currency::from_str(&payload.currency.to_string()).unwrap_or(Currency::EUR),
        application_fee_amount: None,
        automatic_payment_methods: None,
        capture_method: None,
        confirm: Some(true),
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
        payment_method: Some(stripe::PaymentMethodId::from_str(&payload.payment_method).unwrap()),
        payment_method_configuration: None,
        payment_method_data: None,
        payment_method_options: None,
        payment_method_types: None,
        radar_options: None,
        receipt_email: None,
        return_url: Some("https://amanahacademia.com"),
        setup_future_usage: None,
        shipping: None,
        statement_descriptor: None,
        statement_descriptor_suffix: None,
        transfer_data: None,
        transfer_group: None,
        use_stripe_sdk: None,
    };

    match PaymentIntent::create(&state.stripe_client, payment_intent).await {
        Ok(payment_intent) => match payment_intent.status {
            PaymentIntentStatus::Succeeded => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "succes".to_string(),
                error: None,
            })),
            PaymentIntentStatus::RequiresAction => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "requires_action".to_string(),
                error: Some("Requires additional authentication".to_string()),
            })),
            PaymentIntentStatus::RequiresPaymentMethod => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "requires_payment_method".to_string(),
                error: Some("Invalid payment method".to_string()),
            })),
            PaymentIntentStatus::Canceled => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "canceled".to_string(),
                error: Some("Payment canceled".to_string()),
            })),
            PaymentIntentStatus::Processing => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "processing".to_string(),
                error: Some("Payment processing".to_string()),
            })),
            _ => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: payment_intent.status.to_string(),
                error: None,
            })),
        },
        Err(_) => Json(json!(PaymentResponse {
            client_secret: None,
            status: "error".to_string(),
            error: Some("Error processing payment".to_string()),
        })),
    }
}

// Comprar clase individual
pub async fn basic_class_payment(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PaymentPayload>,
) -> impl IntoResponse {
    // Validar el monto mínimo (ejemplo: $5.00 USD = 500 centavos)
    if payload.amount < 500 {
        return Json(json!({
            "succes": false,
            "message": Some("El monto mínimo es 5.00€".to_string()),
        }));
    }

    // Crear el PaymentIntent
    let payment_intent: CreatePaymentIntent = CreatePaymentIntent {
        amount: payload.amount,
        currency: Currency::from_str(&payload.currency.to_string()).unwrap_or(Currency::EUR),
        application_fee_amount: None,
        automatic_payment_methods: None,
        capture_method: None,
        confirm: Some(true),
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
        payment_method: Some(stripe::PaymentMethodId::from_str(&payload.payment_method).unwrap()),
        payment_method_configuration: None,
        payment_method_data: None,
        payment_method_options: None,
        payment_method_types: None,
        radar_options: None,
        receipt_email: None,
        return_url: Some("https://amanahacademia.com"),
        setup_future_usage: None,
        shipping: None,
        statement_descriptor: None,
        statement_descriptor_suffix: None,
        transfer_data: None,
        transfer_group: None,
        use_stripe_sdk: None,
    };

    match PaymentIntent::create(&state.stripe_client, payment_intent).await {
        Ok(payment_intent) => match payment_intent.status {
            PaymentIntentStatus::Succeeded => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "succes".to_string(),
                error: Some("Payment successful".to_string()),
            })),
            PaymentIntentStatus::RequiresAction => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "requires_action".to_string(),
                error: Some("Requires additional authentication".to_string()),
            })),
            PaymentIntentStatus::RequiresPaymentMethod => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "requires_payment_method".to_string(),
                error: Some("Invalid payment method".to_string()),
            })),
            PaymentIntentStatus::Canceled => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "canceled".to_string(),
                error: Some("Payment canceled".to_string()),
            })),
            PaymentIntentStatus::Processing => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "processing".to_string(),
                error: Some("Payment processing".to_string()),
            })),
            _ => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: payment_intent.status.to_string(),
                error: None,
            })),
        },
        Err(_) => Json(json!(PaymentResponse {
            client_secret: None,
            status: "error".to_string(),
            error: Some("Error processing payment".to_string()),
        })),
    }
}

// Consultar un payment para saber su estado actual
pub async fn get_payment_status(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

// Cancelar un payment
pub async fn cancel_payment(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

// Devolver el pago
pub async fn refund_payment(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

// Consultar el historial de pagos del usuario
pub async fn get_payment_history(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

// Webhook para recibir eventos de Stripe
pub async fn webhook_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

// Crear un producto
#[debug_handler]
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

// Convierte una cadena a una divisa
fn convert_string_to_currency(currency: &str) -> Currency {
    Currency::from_str(currency).unwrap_or(Currency::EUR)
}
