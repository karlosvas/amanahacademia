use {
    crate::{
        controllers::payments::{
            archive_product, cancel_payment, create_product, delete_price, get_all_prices,
            get_all_products, get_payment_history, get_payment_status, payment_intent,
            refund_payment, webhook_handler,
        },
        middleware::auth::firebase_auth_middleware,
        state::AppState,
    },
    axum::{
        Router, middleware,
        routing::{delete, get, post},
    },
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let protected_routes: Router<Arc<AppState>> = Router::new()
        .route("/intent", post(payment_intent))
        .route("/product/all", get(get_all_products))
        .route("/price/all", get(get_all_prices))
        .route("/del/product/:id", delete(archive_product))
        .route("/del/price/:id", delete(delete_price))
        .route("/product", post(create_product))
        .route("/:id", get(get_payment_status))
        .route("/:id/canceled", post(cancel_payment))
        .route("/:id/refund", post(refund_payment))
        .route("/history", get(get_payment_history))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            firebase_auth_middleware,
        ));

    let webhook_routes: Router<Arc<AppState>> =
        Router::new().route("/webhooks", post(webhook_handler));

    Router::new()
        .merge(protected_routes)
        .merge(webhook_routes)
        .with_state(state)
}
