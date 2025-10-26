use {
    crate::{
        controllers::metrics::{get_article_metrics, get_user_metrics},
        middleware::auth::public_ga_auth_middleware,
        models::state::AppState,
    },
    axum::{Router, middleware, routing::get},
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let public_routes: Router<Arc<AppState>> = Router::new()
        .route("/users", get(get_user_metrics))
        .route("/articles", get(get_article_metrics))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            public_ga_auth_middleware,
        ))
        .with_state(state.clone());

    Router::new().merge(public_routes).with_state(state)
}
