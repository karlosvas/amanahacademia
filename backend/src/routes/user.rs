use {
    crate::{
        controllers::user::{create_user, delete_me, get_all_users, login_user, update_user},
        middleware::auth::firebase_auth_middleware,
        state::AppState,
    },
    axum::{
        Router, middleware,
        routing::{delete, get, post, put},
    },
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let public_routes = Router::new()
        .route("/register", post(create_user)) // POST /user/register
        .route("/login", post(login_user)); // GET /user/login

    let protected_routes = Router::new()
        .route("/me", put(update_user)) // PUT /user/
        .route("/del/me", delete(delete_me)) // DELETE /user/me
        .route("/all", get(get_all_users)) // GET /user/all
        .layer(middleware::from_fn_with_state(
            state.clone(),
            firebase_auth_middleware,
        ));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}
