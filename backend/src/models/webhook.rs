use {
    crate::models::cal::BookingStatus,
    chrono::{DateTime, Utc},
    core::fmt,
    serde::{Deserialize, Serialize},
    serde_json::Value,
};

/// Tipos de eventos que pueden activar un webhook de Cal.com
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WebhookTrigger {
    BookingCreated,
    BookingCancelled,
    BookingRescheduled,
    BookingPaid,
}
impl fmt::Display for WebhookTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebhookTrigger::BookingCreated => write!(f, "BOOKING_CREATED"),
            WebhookTrigger::BookingCancelled => write!(f, "BOOKING_CANCELLED"),
            WebhookTrigger::BookingRescheduled => write!(f, "BOOKING_RESCHEDULED"),
            WebhookTrigger::BookingPaid => write!(f, "BOOKING_PAID"),
        }
    }
}

/// Estructura del evento recibido desde el webhook de Cal.com
#[derive(Deserialize, Debug)]
pub struct CalWebhookEvent {
    #[serde(rename = "triggerEvent")]
    pub trigger_event: WebhookTrigger,
    #[allow(dead_code)]
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub payload: CalBookingPayload,
}

/// Estructura de los datos de la reserva incluidos en el payload del webhook de Cal.com
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct CalBookingPayload {
    /// Identificador único de la reserva en Cal.com
    pub uid: String,

    /// ID numérico de la reserva en Cal.com (opcional, puede no estar presente en algunos eventos)
    #[serde(rename = "bookingId")]
    pub booking_id: Option<i64>,

    /// ID del tipo de evento (ej: clase individual, clase grupal, consultoría)
    #[serde(rename = "eventTypeId")]
    pub event_type_id: Option<i64>,

    /// Slug del tipo de evento (ej: "free-class", "standard-class", "conversation-class")
    /// API v2 puede no incluir este campo en respuestas GET, pero sí en webhooks
    #[serde(rename = "type", default)]
    pub event_type_slug: Option<String>,

    /// Título descriptivo de la reserva (ej: "30min Meeting between John and Teacher")
    pub title: String,

    /// Descripción del evento (opcional)
    pub description: Option<String>,

    /// Fecha y hora de inicio de la reserva en formato ISO 8601
    /// Puede venir como "startTime" o "start" dependiendo de la fuente (webhook vs API)
    #[serde(rename = "startTime", alias = "start", default)]
    pub start_time: Option<String>,

    /// Fecha y hora de finalización de la reserva en formato ISO 8601
    /// Puede venir como "endTime" o "end" dependiendo de la fuente (webhook vs API)
    #[serde(rename = "endTime", alias = "end", default)]
    pub end_time: Option<String>,

    /// Lista de asistentes a la reserva (estudiantes, profesores)
    #[serde(default)]
    pub attendees: Vec<Attendee>,

    /// Información del organizador (profesor)
    /// Puede no estar presente en todas las respuestas de la API
    #[serde(default)]
    pub organizer: Option<Organizer>,

    /// Ubicación/URL de la videollamada
    pub location: Option<String>,

    /// Metadatos adicionales de la reserva (puede incluir información personalizada del formulario)
    pub metadata: Option<Value>,

    /// Estado actual de la reserva (ACCEPTED, CANCELLED, PENDING, REJECTED)
    pub status: BookingStatus,

    /// Razón de cancelación si la reserva fue cancelada (opcional)
    #[serde(rename = "cancellationReason")]
    pub cancellation_reason: Option<String>,
}

/// Estructura que representa a un asistente en una reserva de Cal.com
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Attendee {
    pub email: String,
    pub name: String,

    #[serde(rename = "timeZone")]
    pub time_zone: String,

    #[serde(default, deserialize_with = "deserialize_language")]
    pub language: Option<Language>,
}

/// Estructura que representa el idioma preferido de un asistente
#[derive(Debug, Serialize, Clone)]
pub struct Language {
    pub locale: String,
}

/// Deserializador personalizado para manejar ambos formatos:
/// - String directa: "es"
/// - Objeto: {"locale": "es"}
fn deserialize_language<'de, D>(deserializer: D) -> Result<Option<Language>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum LanguageFormat {
        String(String),
        Object { locale: String },
    }

    match Option::<LanguageFormat>::deserialize(deserializer)? {
        Some(LanguageFormat::String(locale)) => Ok(Some(Language { locale })),
        Some(LanguageFormat::Object { locale }) => Ok(Some(Language { locale })),
        None => Ok(None),
    }
}

/// Estructura que representa al organizador (profesor) de una reserva de Cal.com
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Organizer {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub username: String,

    #[serde(rename = "timeZone")]
    pub time_zone: String,
}

/// Estructura de la respuesta al crear un reembolso en Cal.com
#[derive(Debug, Serialize, Deserialize)]
pub struct RefundResponse {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub status: Option<String>,
    pub created: i64,
}

/// Estructura que representa un cambio de estado en una reserva de Cal.com
#[derive(Debug, Clone)]
pub struct BookingChange {
    pub uid: String,
    pub old_status: BookingStatus,
    pub new_status: BookingStatus,
    pub detected_at: DateTime<Utc>,
}

/// Estructura de respuesta paginada de Cal.com API v2
#[derive(Deserialize, Debug)]
pub struct CalBookingsResponse {
    pub data: Vec<CalBookingPayload>,
}
