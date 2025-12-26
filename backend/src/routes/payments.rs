use {
    crate::{
        controllers::payments::{
            archive_cal_connection, archive_product, create_product, delete_price,
            get_all_paid_reservations, get_all_prices, get_all_products, get_payment_history,
            payment_intent,
        },
        middleware::auth::firebase_auth_middleware,
        models::state::AppState,
    },
    axum::{
        Router, middleware,
        routing::{delete, get, post},
    },
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new().merge(
        Router::new()
            .route("/intent", post(payment_intent))
            .route("/cal/connection", post(archive_cal_connection))
            .route("/cal/connection/all", get(get_all_paid_reservations))
            .route("/product/all", get(get_all_products))
            .route("/price/all", get(get_all_prices))
            .route("/del/product/:id", delete(archive_product))
            .route("/del/price/:id", delete(delete_price))
            .route("/product", post(create_product))
            .route("/history", get(get_payment_history))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                firebase_auth_middleware,
            ))
            .with_state(state),
    )
}
