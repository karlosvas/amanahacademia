use {
    crate::{
        controllers::mailchimp::{add_contact, get_all_contacts},
        state::AppState,
    },
    axum::{
        Router,
        routing::{get, post},
    },
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let public_routes: Router<Arc<AppState>> =
        Router::new().route("/add_contact", post(add_contact));

    let protected_routes: Router<Arc<AppState>> =
        Router::new().route("/get_all_contacts", get(get_all_contacts));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}
