use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub app_state: String,
    pub firebase: CustomFirebase,
}

#[derive(Clone)]
pub struct CustomFirebase {
    pub firebase_keys: serde_json::Value,
    pub firebase_project_id: String,
    pub firebase_api_key: String,
}

pub type SharedState = Arc<AppState>;
