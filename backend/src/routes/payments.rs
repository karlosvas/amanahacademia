use {
    crate::{
        controllers::payments::{
            archive_product, create_product, delete_price, get_all_paid_reservations,
            get_all_prices, get_all_products, get_payment_history, payment_intent,
        },
        middleware::auth::{admin_middleware, firebase_auth_middleware},
        models::state::AppState,
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
        .layer(middleware::from_fn_with_state(
            state.clone(),
            firebase_auth_middleware,
        ));

    let admin_routes: Router<Arc<AppState>> = Router::new()
        .route("/cal/connection/all", get(get_all_paid_reservations))
        .route("/product/all", get(get_all_products))
        .route("/price/all", get(get_all_prices))
        .route("/del/product/:id", delete(archive_product))
        .route("/del/price/:id", delete(delete_price))
        .route("/product", post(create_product))
        .route("/history", get(get_payment_history))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            admin_middleware,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            firebase_auth_middleware,
        ));

    Router::new()
        .merge(protected_routes)
        .merge(admin_routes)
        .with_state(state)
}
