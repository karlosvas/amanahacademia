use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub firebase: CustomFirebase,
    pub client: Client,
}

#[derive(Clone)]
pub struct CustomFirebase {
    pub firebase_keys: serde_json::Value,
    pub firebase_project_id: String,
    pub firebase_api_key: String,
}
