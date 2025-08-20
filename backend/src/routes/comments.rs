use {
    crate::{
        controllers::comments::{
            add_comment, add_reply, delete_comment, get_all_comments, toggle_like,
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
    Router::new()
        .route("/add", post(add_comment))
        .route("/all", get(get_all_comments))
        .route("/like/:comment_id", put(toggle_like))
        .route("/reply/:comment_id", put(add_reply))
        .route("/del/:comment_id", delete(delete_comment))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            firebase_auth_middleware,
        ))
        .with_state(state)
}
