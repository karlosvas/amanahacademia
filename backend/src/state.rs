use {reqwest::Client as HttpClient, resend_rs::Resend, stripe::Client as StripeClient};

#[derive(Clone)]
pub struct AppState {
    pub firebase: CustomFirebase,
    pub firebase_client: HttpClient,
    pub stripe_client: StripeClient,
    pub resend_client: Resend,
}

#[derive(Clone)]
pub struct CustomFirebase {
    pub firebase_keys: serde_json::Value,
    pub firebase_project_id: String,
    pub firebase_api_key: String,
    pub firebase_database_url: String,
}
