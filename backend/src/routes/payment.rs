use {
    crate::{
        controllers::payment::{
            basic_class, cancel_payment, get_payment_history, get_payment_status, refund_payment,
            webhook_handler,
        },
        middleware::auth::firebase_auth_middleware,
        state::AppState,
    },
    axum::{
        Router, middleware,
        routing::{get, post},
    },
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let protected_routes: Router<Arc<AppState>> = Router::new()
        .route("/basic-class", post(basic_class))
        .route("/:id", get(get_payment_status))
        .route("/:id/canceled", post(cancel_payment))
        .route("/:id/refund", post(refund_payment))
        .route("/payments", get(get_payment_history))
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
