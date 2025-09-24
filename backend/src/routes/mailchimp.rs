use {
    crate::{controllers::mailchimp::add_contact, state::AppState},
    axum::{Router, routing::post},
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let public_routes: Router<Arc<AppState>> =
        Router::new().route("/add_newsletter", post(add_contact));

    Router::new().merge(public_routes).with_state(state)
}
