use {
    crate::{
        controllers::teachers::{create_teacher, delete_teacher, get_all_teachers, get_teacher},
        middleware::auth::firebase_auth_middleware,
        models::state::AppState,
    },
    axum::{
        Router, middleware,
        routing::{delete, get, post},
    },
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let public_routes: Router<Arc<AppState>> = Router::new().route("/all", get(get_all_teachers)); // GET /all

    let protected_routes: Router<Arc<AppState>> = Router::new()
        .route("/add", post(create_teacher)) // POST /add
        .route("/:id", get(get_teacher)) // GET /user/:id
        .route("/del/:id", delete(delete_teacher)) // GET /user/:id
        .layer(middleware::from_fn_with_state(
            state.clone(),
            firebase_auth_middleware,
        ));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}
