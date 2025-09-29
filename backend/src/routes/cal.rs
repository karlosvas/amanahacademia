use {
    crate::{
        controllers::cal::confirm_booking, middleware::auth::firebase_auth_middleware,
        state::AppState,
    },
    axum::{Router, middleware, routing::post},
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let protected_routes: Router<Arc<AppState>> = Router::new()
        .route("/bookings/:id/confirm", post(confirm_booking))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            firebase_auth_middleware,
        ));

    Router::new().merge(protected_routes).with_state(state)
}
