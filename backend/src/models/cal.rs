use {
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
};

/// Estado de los bookings de cal
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
/// Cal.com API v2 envía valores en lowercase, pero también acepta SCREAMING_SNAKE_CASE
/// Usamos alias para compatibilidad con ambos formatos
#[serde(rename_all = "lowercase")]
pub enum BookingStatus {
    Accepted,
    Pending,
    Cancelled,
    Rejected,
}

/// Booking de reserva de calse en cal

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booking {
    pub uid: String,
    pub bookingId: Option<String>,
    pub eventTypeId: Option<String>,
    #[serde(rename = "type")]
    pub booking_type: Option<String>,

    pub title: String,
    pub description: Option<String>,

    pub startTime: Option<DateTime<Utc>>,
    pub endTime: Option<DateTime<Utc>>,

    pub attendees: Vec<BookingAttendee>,
    pub organizer: Option<BookingOrganizer>,

    pub location: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub status: BookingStatus,
    pub cancellationReason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingAttendee {
    pub email: String,
    pub name: String,
    pub timeZone: String,
    pub language: BookingLanguage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingLanguage {
    pub locale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingOrganizer {
    pub email: Option<String>,
    pub name: Option<String>,
}

/// Obtener el booking por id
/// Wrapper para respuestas de Cal.com API v2
#[derive(Deserialize, Debug)]
pub struct CalApiResponse<T> {
    pub status: String,
    pub data: T,
}
