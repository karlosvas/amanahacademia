use {
    crate::{controllers::email::send_contact_email, models::state::AppState},
    axum::{Router, routing::post},
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let public_routes: Router<Arc<AppState>> =
        Router::new().route("/contact", post(send_contact_email));

    Router::new().merge(public_routes).with_state(state)
}
