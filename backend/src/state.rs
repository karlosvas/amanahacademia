use {
    crate::models::mailchimp::MailchimpClient, reqwest::Client as HttpClient, resend_rs::Resend,
    stripe::Client as StripeClient,
};

pub struct AppState {
    pub firebase: CustomFirebase,
    pub firebase_client: HttpClient,
    pub stripe_client: StripeClient,
    pub resend_client: Resend,
    pub mailchimp_client: MailchimpClient,
    pub booking_client: HttpClient,
    pub cal_options: CalOptions,
}

pub struct CalOptions {
    pub client: HttpClient,
    pub api_version: String,
    pub base_url: String,
    pub api_key: String,
}

#[derive(Clone)]
pub struct CustomFirebase {
    pub firebase_keys: serde_json::Value,
    pub firebase_project_id: String,
    pub firebase_api_key: String,
    pub firebase_database_url: String,
}
