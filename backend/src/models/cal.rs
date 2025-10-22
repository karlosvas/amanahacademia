use {
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
};

/// Estado de los bookings de cal
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
/// Cal.com API v2 envía valores en lowercase, pero también acepta SCREAMING_SNAKE_CASE
/// Usamos alias para compatibilidad con ambos formatos
#[serde(rename_all = "lowercase")]
pub enum BookingStatus {
    #[serde(alias = "ACCEPTED")]
    Accepted,
    #[serde(alias = "PENDING")]
    Pending,
    #[serde(alias = "CANCELLED")]
    Cancelled,
    #[serde(alias = "REJECTED")]
    Rejected,
}

/// Booking de reserva de calse en cal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booking {
    uid: String,
    status: BookingStatus,
    title: String,
    #[serde(rename = "updatedAt")]
    updated_at: Option<DateTime<Utc>>,
}
