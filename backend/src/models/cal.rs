use {
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
    std::fmt,
};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BookingStatus {
    Accepted,
    Pending,
    Cancelled,
    Rejected,
}
impl fmt::Display for BookingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BookingStatus::Accepted => write!(f, "ACCEPTED"),
            BookingStatus::Pending => write!(f, "PENDING"),
            BookingStatus::Cancelled => write!(f, "CANCELLED"),
            BookingStatus::Rejected => write!(f, "REJECTED"),
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
