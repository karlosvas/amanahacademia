use {
    crate::{
        controllers::cookies::{add_session, delete_session, get_session},
        state::AppState,
    },
    axum::{
        Router,
        routing::{delete, get, post},
    },
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let public_routes = Router::new()
        .route("/session", post(add_session)) // POST /cookies/session
        .route("/session", get(get_session)) // GET /cookies/session
        .route("/session", delete(delete_session)) // DELETE /cookies/session
        .layer(tower_cookies::CookieManagerLayer::new()); // Añadir el middleware de cookies

    Router::new().merge(public_routes).with_state(state)
}
