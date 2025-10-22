use {
    crate::{
        controllers::webhook::{handle_cal_webhook, health_check},
        models::state::AppState,
    },
    axum::{
        Router,
        routing::{get, post},
    },
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/cal", post(handle_cal_webhook));
    Router::new().merge(public_routes).with_state(state)
}
