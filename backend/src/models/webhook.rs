use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct CalWebhookEvent {
    #[serde(rename = "triggerEvent")]
    pub trigger_event: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub payload: CalBookingPayload,
}

#[derive(Deserialize, Debug)]
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
    pub status: String,
    #[serde(rename = "cancellationReason")]
    pub cancellation_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Attendee {
    pub name: String,
    pub email: String,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
}
