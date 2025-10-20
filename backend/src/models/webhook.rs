use {
    crate::models::cal::BookingStatus,
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
    serde_json::Value,
};

#[derive(Deserialize, Debug)]
pub struct CalWebhookEvent {
    #[serde(rename = "triggerEvent")]
    pub trigger_event: String,
    #[allow(dead_code)]
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub payload: CalBookingPayload,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct CalBookingPayload {
    pub uid: String,
    #[serde(rename = "bookingId")]
    pub booking_id: Option<i64>,
    #[serde(rename = "eventTypeId")]
    pub event_type_id: i64,
    pub title: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    pub attendees: Vec<Attendee>,
    pub metadata: Option<Value>,
    pub status: BookingStatus,
    #[serde(rename = "cancellationReason")]
    pub cancellation_reason: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Attendee {
    pub name: String,
    pub email: String,
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
