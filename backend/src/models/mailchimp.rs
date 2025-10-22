use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub email_address: String,
    /// Estado del contacto: "subscribed", "unsubscribed", "cleaned", "pending"
    pub status: String,
    /// Campos personalizados (nombre, apellido, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<MergeFields>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MergeFields {
    /// Nombre del contacto
    #[serde(rename = "FNAME")]
    pub fname: Option<String>,
    /// Apellido del contacto
    #[serde(rename = "LNAME")]
    pub lname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddContactResponse {
    /// ID único del contacto en Mailchimp
    pub id: String,
    pub email_address: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Campaign {
    pub id: String,
    pub settings: CampaignSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignSettings {
    /// Asunto del email que verán los destinatarios
    pub subject_line: String,
    /// Título interno de la campaña (no visible para suscriptores)
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMailchimp {
    /// Tipo de error (ej: "https://mailchimp.com/developer/marketing/docs/errors/")
    #[serde(rename = "type")]
    pub type_: String,
    pub title: String,
    /// Código HTTP del error
    pub status: i32,
    /// Descripción detallada del error
    pub detail: String,
    /// URI del recurso que causó el error
    pub instance: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MembersResponse {
    pub members: Vec<Member>,
    pub list_id: String,
    pub total_items: usize,
    /// Links HATEOAS para navegación de la API
    #[serde(default)]
    pub _links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    /// MD5 hash del email en lowercase
    pub id: String,
    pub email_address: String,
    /// ID único permanente del email (no cambia si se actualiza el email)
    pub unique_email_id: String,
    /// ID del contacto en el sistema de CRM de Mailchimp
    pub contact_id: String,
    pub full_name: String,
    /// ID numérico del contacto
    pub web_id: i64,
    /// Formato preferido: "html" o "text"
    pub email_type: Option<String>,
    /// Estado de suscripción: "subscribed", "unsubscribed", "cleaned", "pending"
    pub status: String,
    /// Razón de cancelación si status es "unsubscribed"
    pub unsubscribe_reason: Option<String>,
    /// Permite mensajería 1-a-1 (para GDPR)
    pub consents_to_one_to_one_messaging: Option<bool>,
    pub sms_phone_number: Option<String>,
    pub sms_subscription_status: Option<String>,
    pub sms_subscription_last_updated: Option<String>,
    /// Campos personalizados (FNAME, LNAME, etc.)
    pub merge_fields: Option<serde_json::Value>,
    /// Intereses/grupos a los que está suscrito
    pub interests: Option<serde_json::Value>,
    /// Estadísticas de engagement del suscriptor
    pub stats: Option<MemberStats>,
    /// IP desde donde se registró
    pub ip_signup: Option<String>,
    /// Timestamp de registro (ISO 8601)
    pub timestamp_signup: Option<String>,
    /// IP desde donde confirmó la suscripción (double opt-in)
    pub ip_opt: Option<String>,
    /// Timestamp de confirmación
    pub timestamp_opt: Option<String>,
    /// Rating de 1-5 basado en engagement (calculado por Mailchimp)
    pub member_rating: Option<i64>,
    /// Última modificación del registro
    pub last_changed: Option<String>,
    /// Código de idioma (ej: "en", "es")
    pub language: Option<String>,
    /// Marca si es un contacto VIP
    pub vip: Option<bool>,
    /// Cliente de email detectado (ej: "Gmail", "Outlook")
    pub email_client: Option<String>,
    /// Geolocalización inferida por Mailchimp
    pub location: Option<Location>,
    /// Permisos de marketing según GDPR
    pub marketing_permissions: Option<Vec<MarketingPermission>>,
    /// Última nota añadida al contacto
    pub last_note: Option<LastNote>,
    /// Origen del contacto (ej: "API", "Import", "Signup Form")
    pub source: Option<String>,
    pub tags_count: Option<i64>,
    pub tags: Option<Vec<Tag>>,
    pub list_id: String,
    #[serde(default)]
    pub _links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberStats {
    /// Tasa promedio de apertura de emails (0.0 - 1.0)
    pub avg_open_rate: Option<f64>,
    /// Tasa promedio de clicks (0.0 - 1.0)
    pub avg_click_rate: Option<f64>,
    /// Datos de compras si está integrado con ecommerce
    pub ecommerce_data: Option<EcommerceData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EcommerceData {
    /// Ingresos totales generados por este cliente
    pub total_revenue: Option<f64>,
    pub number_of_orders: Option<i64>,
    pub currency_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    /// GMT offset en segundos
    pub gmtoff: Option<i64>,
    /// Daylight Saving Time offset en segundos
    pub dstoff: Option<i64>,
    /// Código ISO del país
    pub country_code: Option<String>,
    pub timezone: Option<String>,
    /// Región/estado/provincia
    pub region: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingPermission {
    pub marketing_permission_id: String,
    /// Descripción del permiso (ej: "Email Marketing")
    pub text: String,
    /// Si el usuario dio consentimiento
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastNote {
    pub note_id: i64,
    pub created_at: String,
    pub created_by: String,
    pub note: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

/// Link HATEOAS para navegación de la API REST
#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    /// Relación del link (ej: "self", "parent", "update")
    pub rel: String,
    /// URL del recurso
    pub href: String,
    /// Método HTTP a usar (GET, POST, PUT, DELETE)
    pub method: String,
    /// JSON Schema del recurso (si está disponible)
    pub schema: Option<String>,
}
