use {
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
    std::fmt,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BookingStatus {
    ACCEPTED,
    CANCELLED,
    PENDING,
    REJECTED,
}

impl fmt::Display for BookingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BookingStatus::ACCEPTED => write!(f, "ACCEPTED"),
            BookingStatus::CANCELLED => write!(f, "CANCELLED"),
            BookingStatus::PENDING => write!(f, "PENDING"),
            BookingStatus::REJECTED => write!(f, "REJECTED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booking {
    uid: String,
    status: BookingStatus,
    title: String,
    #[serde(rename = "updatedAt")]
    updated_at: Option<DateTime<Utc>>,
}
