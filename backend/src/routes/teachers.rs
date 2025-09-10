use {
    crate::{
        controllers::teachers::{create_teacher, get_all_teachers, get_teacher},
        middleware::auth::firebase_auth_middleware,
        state::AppState,
    },
    axum::{
        Router, middleware,
        routing::{get, post},
    },
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let public_routes: Router<Arc<AppState>> = Router::new().route("/add", post(create_teacher)); // POST /add

    let protected_routes: Router<Arc<AppState>> = Router::new()
        .route("/all", get(get_all_teachers)) // GET /all
        .route("/:id", get(get_teacher)) // GET /user/:id
        .layer(middleware::from_fn_with_state(
            state.clone(),
            firebase_auth_middleware,
        ));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}
