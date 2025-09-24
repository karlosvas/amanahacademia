use {
    crate::{
        controllers::comments::{
            add_comment, add_reply, delete_comment, edit_comment, get_all_comments,
            get_comment_by_id, toggle_like,
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
    let public_routes: Router<Arc<AppState>> = Router::new().route("/all", get(get_all_comments));

    let protected_routes: Router<Arc<AppState>> = Router::new()
        .route("/add", post(add_comment))
        .route("/:id", get(get_comment_by_id))
        .route("/edit/:comment_id", put(edit_comment))
        .route("/like/:comment_id", put(toggle_like))
        .route("/reply/:comment_id", put(add_reply))
        .route("/del/:comment_id", delete(delete_comment))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            firebase_auth_middleware,
        ));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}
