use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::models::webhook::{Attendee, Organizer};

/// Estado de los bookings de cal
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
/// Cal.com API v2 envía valores en UPPERCASE o lowercase según el endpoint
/// Usamos alias para compatibilidad con ambos formatos
#[serde(rename_all = "lowercase")]
pub enum BookingStatus {
    #[serde(alias = "ACCEPTED")]
    Accepted,
    #[default]
    #[serde(alias = "PENDING")]
    Pending,
    #[serde(alias = "CANCELLED")]
    Cancelled,
    #[serde(alias = "REJECTED")]
    Rejected,
}

/// Obtener el booking por id
/// Wrapper para respuestas de Cal.com API v2
#[derive(Deserialize, Debug)]
pub struct CalApiResponse<T> {
    pub status: String,
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Availability {
    pub days: Vec<String>,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub id: u64,
    pub owner_id: u64,
    pub name: String,
    pub time_zone: String,
    pub availability: Vec<Availability>,
    pub is_default: bool,
    pub overrides: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct BookingsQueryParams {
    #[serde(rename = "eventTypeId")]
    pub event_type_id: Option<String>,

    #[serde(rename = "eventTypeIds")]
    pub event_type_ids: Option<String>,

    #[serde(rename = "attendeeEmail")]
    pub attendee_email: Option<String>,

    #[serde(rename = "attendeeName")]
    pub attendee_name: Option<String>,

    #[serde(rename = "teamId")]
    pub team_id: Option<String>,

    #[serde(rename = "afterStart")]
    pub after_start: Option<String>,

    #[serde(rename = "beforeEnd")]
    pub before_end: Option<String>,

    pub status: Option<String>,

    #[serde(rename = "sortStart")]
    pub sort_start: Option<String>, // "asc" | "desc"
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserCal {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub timeZone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EventTypeCal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Estructura de los datos de la reserva incluidos en el payload del webhook de Cal.com
/// Compatible con Cal.com API v2 (webhooks y endpoints REST)
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct CalBookingPayload {
    /// ID numérico directo de Cal.com (cuando viene como "id" en respuestas)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Identificador único de la reserva en Cal.com
    /// Es opcional porque no viene al crear un booking, solo en respuestas posteriores
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// ID numérico de la reserva en Cal.com (opcional, puede no estar presente en algunos eventos)
    #[serde(rename = "bookingId", default, skip_serializing_if = "Option::is_none")]
    pub booking_id: Option<i64>,

    /// ID del tipo de evento (ej: clase individual, clase grupal, consultoría)
    #[serde(
        rename = "eventTypeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_type_id: Option<i64>,

    /// Slug del tipo de evento (ej: "free-class", "standard-class", "conversation-class")
    /// API v2 puede no incluir este campo en respuestas GET, pero sí en webhooks
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub event_type_slug: Option<String>,

    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EventTypeCal>,

    /// Username del usuario/organizador (requerido para crear bookings sin eventTypeId)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserCal>,

    /// Slug del equipo (opcional, para bookings de equipo)
    #[serde(rename = "teamSlug", default, skip_serializing_if = "Option::is_none")]
    pub team_slug: Option<String>,

    /// Slug de la organización (opcional, para bookings de organización)
    #[serde(
        rename = "organizationSlug",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_slug: Option<String>,

    /// Título descriptivo de la reserva (ej: "30min Meeting between John and Teacher")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Descripción del evento (opcional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Fecha y hora de inicio de la reserva en formato ISO 8601
    /// Puede venir como "startTime" o "start" dependiendo de la fuente (webhook vs API)
    #[serde(
        rename = "startTime",
        alias = "start",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_time: Option<String>,

    /// Fecha y hora de finalización de la reserva en formato ISO 8601
    /// Puede venir como "endTime" o "end" dependiendo de la fuente (webhook vs API)
    #[serde(
        rename = "endTime",
        alias = "end",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_time: Option<String>,

    /// Duración de la reserva en minutos (opcional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,

    /// Lista de asistentes a la reserva (estudiantes, profesores)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attendees: Vec<Attendee>,

    /// Información del organizador (profesor)
    /// Puede no estar presente en todas las respuestas de la API
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizer: Option<Organizer>,

    /// Ubicación/URL de la videollamada
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Metadatos adicionales de la reserva (puede incluir información personalizada del formulario)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,

    /// Estado actual de la reserva (ACCEPTED, CANCELLED, PENDING, REJECTED)
    #[serde(default)]
    pub status: BookingStatus,

    /// Razón de cancelación si la reserva fue cancelada (opcional)
    #[serde(
        rename = "cancellationReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancellation_reason: Option<String>,

    /// Información de la reunión (opcional)
    #[serde(
        rename = "meetingUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub meeting_url: Option<String>,

    /// Token de cancelación (usado para cancelar sin autenticación)
    #[serde(
        rename = "cancelToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancel_token: Option<String>,

    /// Token de reagendamiento (usado para reagendar sin autenticación)
    #[serde(
        rename = "rescheduleToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reschedule_token: Option<String>,

    /// Hosts/organizadores del evento (puede incluir varios en eventos de equipo)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<Value>>,

    /// Seat UID para bookings de eventos con asientos (group bookings)
    #[serde(rename = "seatUid", default, skip_serializing_if = "Option::is_none")]
    pub seat_uid: Option<String>,

    /// Host ausente
    #[serde(
        rename = "absentHost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub absent_host: Option<bool>,

    /// Fecha de creación
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Fecha de última actualización
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    /// Rating del evento
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rating: Option<Value>,

    /// ICS UID para integración con calendarios
    #[serde(rename = "icsUid", default, skip_serializing_if = "Option::is_none")]
    pub ics_uid: Option<String>,

    /// Email del usuario que reagendó
    #[serde(
        rename = "rescheduledByEmail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rescheduled_by_email: Option<String>,

    /// Lista de emails de invitados adicionales (para agregar al crear el booking)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guests: Option<Vec<String>>,
}
/// Estructura para agregar invitados a un booking
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddGuestsPayload {
    pub guests: Vec<GuestInput>,
}

/// Estructura de un invitado para agregar a un booking
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GuestInput {
    pub email: String,
    pub name: Option<String>,
    pub time_zone: Option<String>,
    pub phone_number: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Error)]
pub enum FetchCalErrors {
    #[error("Failed to communicate with Cal.com: {0}")]
    Network(reqwest::Error),

    #[error("Failed to parse Cal.com response: {0}")]
    ParseError(reqwest::Error),
}
