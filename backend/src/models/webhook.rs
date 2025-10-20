use {
    crate::models::cal::BookingStatus,
    chrono::{DateTime, Utc},
    core::fmt,
    serde::{Deserialize, Serialize},
    serde_json::Value,
};

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

#[derive(Deserialize, Debug)]
pub struct CalWebhookEvent {
    #[serde(rename = "triggerEvent")]
    pub trigger_event: WebhookTrigger,
    #[allow(dead_code)]
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub payload: CalBookingPayload,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct CalBookingPayload {
    /// Identificador único de la reserva en Cal.com
    pub uid: String,

    /// ID numérico de la reserva en Cal.com (opcional, puede no estar presente en algunos eventos)
    #[serde(rename = "bookingId")]
    pub booking_id: Option<i64>,

    /// ID del tipo de evento (ej: clase individual, clase grupal, consultoría)
    #[serde(rename = "eventTypeId")]
    pub event_type_id: i64,

    /// Slug del tipo de evento (ej: "free-class", "standard-class", "conversation-class")
    #[serde(rename = "type")]
    pub event_type_slug: String, // ✅ Agregado

    /// Título descriptivo de la reserva (ej: "30min Meeting between John and Teacher")
    pub title: String,

    /// Descripción del evento (opcional)
    pub description: Option<String>, // ✅ Agregado (útil)

    /// Fecha y hora de inicio de la reserva en formato ISO 8601
    #[serde(rename = "startTime")]
    pub start_time: String,

    /// Fecha y hora de finalización de la reserva en formato ISO 8601
    #[serde(rename = "endTime")]
    pub end_time: String,

    /// Lista de asistentes a la reserva (estudiantes, profesores)
    pub attendees: Vec<Attendee>,

    /// Información del organizador (profesor)
    pub organizer: Organizer, // ✅ Agregado (importante)

    /// Ubicación/URL de la videollamada
    pub location: Option<String>, // ✅ Agregado (útil para obtener link de Cal Video)

    /// Metadatos adicionales de la reserva (puede incluir información personalizada del formulario)
    pub metadata: Option<Value>,

    /// Estado actual de la reserva (ACCEPTED, CANCELLED, PENDING, REJECTED)
    pub status: BookingStatus,

    /// Razón de cancelación si la reserva fue cancelada (opcional)
    #[serde(rename = "cancellationReason")]
    pub cancellation_reason: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Attendee {
    pub email: String,
    pub name: String,

    #[serde(rename = "timeZone")]
    pub time_zone: String,

    pub language: Option<Language>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Language {
    pub locale: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Organizer {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub username: String,

    #[serde(rename = "timeZone")]
    pub time_zone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundResponse {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub status: Option<String>,
    pub created: i64,
}

#[derive(Debug, Clone)]
pub struct BookingChange {
    pub uid: String,
    pub old_status: BookingStatus,
    pub new_status: BookingStatus,
    pub detected_at: DateTime<Utc>,
}
