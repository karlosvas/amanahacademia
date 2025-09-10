use {
    crate::{controllers::webhook::health_check, state::AppState},
    axum::{Router, routing::get},
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let public_routes = Router::new().route("/healthcheck", get(health_check)); // GET /healthcheck
    Router::new().merge(public_routes).with_state(state)
}
