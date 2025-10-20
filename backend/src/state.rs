use {
    crate::models::{
        mailchimp::MailchimpClient,
        webhook::{BookingChange, CalBookingPayload as Booking},
    },
    reqwest::Client as HttpClient,
    resend_rs::Resend,
    std::{collections::HashMap, sync::Arc},
    stripe::Client as StripeClient,
    tokio::sync::RwLock,
};

pub struct AppState {
    pub firebase_options: CustomFirebase,
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
    pub booking_cache: Arc<RwLock<HashMap<String, Booking>>>,
    pub recent_changes: Arc<RwLock<Vec<BookingChange>>>,
}

#[derive(Clone)]
pub struct CustomFirebase {
    pub firebase_keys: serde_json::Value,
    pub firebase_project_id: String,
    pub firebase_api_key: String,
    pub firebase_database_url: String,
    pub firebase_database_secret: String,
    pub firebase_client: HttpClient,
}
