use {
    crate::models::{cal::CalBookingPayload, metrics::ServiceAccount, webhook::BookingChange},
    reqwest::Client as HttpClient,
    resend_rs::Resend,
    std::{collections::HashMap, sync::Arc},
    stripe::Client as StripeClient,
    tokio::sync::RwLock,
};

// Estado global de la aplicación
pub struct AppState {
    pub firebase_options: CustomFirebase,
    pub stripe_client: StripeClient,
    pub resend_client: Resend,
    pub mailchimp_client: MailchimpOptions,
    pub cal_options: CalOptions,
    pub ga_options: GAOptions,
}

/// Configuración para interactuar con la API de Google Analytics
pub struct GAOptions {
    pub client: HttpClient,
    pub service_account: ServiceAccount,
    pub property_id: String,
}

/// Configuración para interactuar con la API de Cal.com
pub struct CalOptions {
    pub client: HttpClient,
    pub api_version: String,
    pub base_url: String,
    pub api_key: String,
    pub booking_cache: Arc<RwLock<HashMap<String, CalBookingPayload>>>,
    pub recent_changes: Arc<RwLock<Vec<BookingChange>>>,
}

/// Configuración personalizada para Firebase
#[derive(Clone)]
pub struct CustomFirebase {
    pub firebase_keys: serde_json::Value,
    pub firebase_project_id: String,
    pub firebase_api_key: String,
    pub firebase_database_url: String,
    pub firebase_database_secret: String,
    pub firebase_client: HttpClient,
}

/// Configuración para Mailchimp
#[derive(Debug)]
pub struct MailchimpOptions {
    pub api_key: String,
    pub list_id: String,
    /// Prefijo del datacenter (ej: "us1", "us19") extraído del API key
    pub datacenter: String,
    pub client: reqwest::Client,
}
impl MailchimpOptions {
    pub fn new(api_key: String, datacenter: String, list_id: String) -> Self {
        Self {
            api_key,
            datacenter,
            list_id,
            client: reqwest::Client::new(),
        }
    }

    pub fn get_base_url(&self) -> String {
        format!("https://{}.api.mailchimp.com/3.0", self.datacenter)
    }
}
