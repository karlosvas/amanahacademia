use {
    crate::{controller::user::create_user, state::AppState},
    axum::{Router, routing::post},
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        // Rutas públicas
        .route("/register", post(create_user)) // POST /user/register
        .with_state(state)
    // Rutas protegidas
    // .route("/login", get(obtain_user)) // GET /user/login
    // .route_layer(middleware::from_fn_with_state(
    //     state.clone(),
    //     firebase_auth_middleware,
    // ))
}
