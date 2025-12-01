use {
    crate::{
        controllers::cal::{
            add_booking, confirm_booking, get_all_bookings, get_booking, get_schedule,
            get_schedules,
        },
        middleware::auth::firebase_auth_middleware,
        models::state::AppState,
    },
    axum::{
        Router, middleware,
        routing::{get, post},
    },
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let protected_routes: Router<Arc<AppState>> = Router::new()
        .route("/bookings/:id/confirm", post(confirm_booking))
        .route("/bookings/:id", get(get_booking))
        .route("/bookings", get(get_all_bookings))
        .route("/bookings", post(add_booking))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            firebase_auth_middleware,
        ));

    let public_routes: Router<Arc<AppState>> = Router::new()
        .route("/schedules", get(get_schedules))
        .route("/schedule/:id", get(get_schedule));

    Router::new()
        .merge(protected_routes)
        .merge(public_routes)
        .with_state(state)
}
