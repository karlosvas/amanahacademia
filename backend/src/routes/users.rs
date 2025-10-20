use {
    crate::{
        controllers::users::{
            delete_me, get_all_users, get_user_me, login_user, refresh_token, register_user,
            update_user,
        },
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
        .route("/register", post(register_user)) // POST /user/register
        .route("/login", post(login_user)); // GET /user/login

    let protected_routes = Router::new()
        .route("/update/me", put(update_user)) // PUT /user/
        .route("/del/me", delete(delete_me)) // DELETE /user/me
        .route("/all", get(get_all_users)) // GET /user/all
        .route("/refresh_token", put(refresh_token)) // PUT /user/refresh_token
        .route("/me", get(get_user_me)) // GET /user/me
        .layer(middleware::from_fn_with_state(
            state.clone(),
            firebase_auth_middleware,
        ));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}
