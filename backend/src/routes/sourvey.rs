use {
    crate::{
        controllers::sourvey::{create_survey, get_all_survey_results, get_survey_results},
        middleware::auth::firebase_auth_middleware,
        models::state::AppState,
    },
    axum::{
        Router, middleware,
        routing::{get, post},
    },
    std::sync::Arc,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_survey))
        .route("/:survey_id/results", get(get_survey_results))
        .route("/results", get(get_all_survey_results))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            firebase_auth_middleware,
        ))
        .with_state(state)
}
