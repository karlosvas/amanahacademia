use crate::models::{
    cal::{BookingStatus, CalBookingPayload},
    stripe::{CurrencyMap, PricePayload, PriceTable},
};
use chrono::{DateTime, Duration, Utc};
use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::Sha256;
use stripe::{CreateProductDefaultPriceDataCurrencyOptions, Currency};

pub const CLASSES_PER_WEEK: i64 = 2;
pub const MAX_WEEKS: u32 = 12;
pub const MIN_AMOUNT_CENTS: i64 = 50;
pub const MAX_AMOUNT_CENTS: i64 = 100_000;
pub const STRIPE_SIGNATURE_TOLERANCE_SECS: i64 = 300;
pub const REFUND_MIN_NOTICE_HOURS: i64 = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaidClass {
    Standard,
    Conversation,
    Group,
}

impl PaidClass {
    pub fn from_slug(slug: &str) -> Option<Self> {
        match slug {
            "standard-class" => Some(Self::Standard),
            "conversation-class" => Some(Self::Conversation),
            "group-class" => Some(Self::Group),
            _ => None,
        }
    }

    pub fn slug(self) -> &'static str {
        match self {
            Self::Standard => "standard-class",
            Self::Conversation => "conversation-class",
            Self::Group => "group-class",
        }
    }

    pub fn unit_price(self, prices: &PriceTable) -> f64 {
        match self {
            Self::Standard => prices.individual_standard,
            Self::Conversation => prices.individual_conversation,
            Self::Group => prices.group,
        }
    }
}

pub fn checkout_amount_cents(
    class: PaidClass,
    prices: &PriceTable,
    weeks: Option<u32>,
) -> Result<i64, String> {
    let unit_price: f64 = class.unit_price(prices);
    if !unit_price.is_finite() || unit_price <= 0.0 {
        return Err(format!("Invalid price for {}", class.slug()));
    }

    let unit_cents: i64 = (unit_price * 100.0).round() as i64;
    let amount: i64 = match weeks {
        None => unit_cents,
        Some(weeks) if (1..=MAX_WEEKS).contains(&weeks) => {
            unit_cents * i64::from(weeks) * CLASSES_PER_WEEK
        }
        Some(weeks) => return Err(format!("Invalid number of weeks: {}", weeks)),
    };

    if !(MIN_AMOUNT_CENTS..=MAX_AMOUNT_CENTS).contains(&amount) {
        return Err(format!("Amount out of range: {}", amount));
    }

    Ok(amount)
}

pub fn booking_class_slug(booking: &CalBookingPayload) -> Option<String> {
    booking
        .event_type
        .as_ref()
        .and_then(|event_type| event_type.slug.clone())
        .or_else(|| booking.event_type_slug.clone())
}

pub fn booking_event_type_id(booking: &CalBookingPayload) -> Option<i64> {
    booking.event_type_id.or_else(|| {
        booking
            .event_type
            .as_ref()
            .and_then(|event_type| event_type.id)
    })
}

pub fn booking_has_attendee(booking: &CalBookingPayload, email: &str) -> bool {
    booking
        .attendees
        .iter()
        .any(|attendee| attendee.email.eq_ignore_ascii_case(email))
        || booking
            .guests
            .as_ref()
            .is_some_and(|guests| guests.iter().any(|guest| guest.eq_ignore_ascii_case(email)))
}

pub fn is_safe_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

pub fn secure_eq(a: &str, b: &str) -> bool {
    a.len() == b.len()
        && a.bytes()
            .zip(b.bytes())
            .fold(0u8, |acc, (x, y)| acc | (x ^ y))
            == 0
}

fn hmac_sha256_matches(secret: &str, message: &[&[u8]], signature_hex: &str) -> bool {
    let Ok(expected) = hex::decode(signature_hex.trim()) else {
        return false;
    };
    let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    for part in message {
        mac.update(part);
    }
    mac.verify_slice(&expected).is_ok()
}

pub fn verify_cal_signature(body: &[u8], signature_hex: &str, secret: &str) -> bool {
    hmac_sha256_matches(secret, &[body], signature_hex)
}

pub fn verify_stripe_signature(payload: &[u8], header: &str, secret: &str, now: i64) -> bool {
    let mut timestamp: Option<&str> = None;
    let mut signatures: Vec<&str> = Vec::new();

    for part in header.split(',') {
        match part.trim().split_once('=') {
            Some(("t", value)) => timestamp = Some(value),
            Some(("v1", value)) => signatures.push(value),
            _ => {}
        }
    }

    let Some(timestamp) = timestamp else {
        return false;
    };
    let Ok(signed_at) = timestamp.parse::<i64>() else {
        return false;
    };
    if (now - signed_at).abs() > STRIPE_SIGNATURE_TOLERANCE_SECS {
        return false;
    }

    signatures.iter().any(|signature| {
        hmac_sha256_matches(secret, &[timestamp.as_bytes(), b".", payload], signature)
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefundDecision {
    Refund,
    Transfer(String),
    LateCancellation,
    NotCancelled,
}

fn parse_datetime(value: Option<&str>) -> Option<DateTime<Utc>> {
    value
        .and_then(|value| DateTime::parse_from_rfc3339(value).ok())
        .map(|date| date.with_timezone(&Utc))
}

pub fn refund_decision(booking: &CalBookingPayload, now: DateTime<Utc>) -> RefundDecision {
    match booking.status {
        BookingStatus::Rejected => return RefundDecision::Refund,
        BookingStatus::Cancelled => {}
        _ => return RefundDecision::NotCancelled,
    }

    if let Some(new_uid) = booking
        .rescheduled_to_uid
        .as_ref()
        .filter(|uid| !uid.is_empty())
    {
        return RefundDecision::Transfer(new_uid.clone());
    }

    let cancelled_by_attendee: bool = booking
        .cancelled_by_email
        .as_deref()
        .map(str::trim)
        .filter(|email| !email.is_empty())
        .is_none_or(|email| booking_has_attendee(booking, email));
    if !cancelled_by_attendee {
        return RefundDecision::Refund;
    }

    let cancelled_at: DateTime<Utc> = parse_datetime(booking.updated_at.as_deref())
        .filter(|date| *date <= now)
        .unwrap_or(now);

    match parse_datetime(booking.start_time.as_deref()) {
        Some(start) if start - cancelled_at >= Duration::hours(REFUND_MIN_NOTICE_HOURS) => {
            RefundDecision::Refund
        }
        _ => RefundDecision::LateCancellation,
    }
}

pub fn relation_has_payment_from(relation: &Value, email: &str) -> bool {
    relation.as_object().is_some_and(|entries| {
        entries.iter().any(|(key, value)| {
            key.starts_with("pi_")
                && value
                    .get("email")
                    .and_then(Value::as_str)
                    .is_some_and(|paid_by| paid_by.eq_ignore_ascii_case(email))
        })
    })
}

pub fn payment_intent_ids_from_relation(relation: &Value) -> Vec<String> {
    let Some(entries) = relation.as_object() else {
        return Vec::new();
    };

    entries
        .iter()
        .filter_map(|(key, value)| match value {
            Value::String(id) if key == "stripe_id" => Some(id.clone()),
            Value::Object(_) if key.starts_with("pi_") => Some(key.clone()),
            _ => None,
        })
        .collect()
}

// Añadir otras monedas
pub fn insert_options_by_country(
    currency_opts: &mut CurrencyMap<CreateProductDefaultPriceDataCurrencyOptions>,
    base_price: &PricePayload,
) {
    let amount_eur: i64 = base_price.unit_amount;

    // USD (ejemplo: 1 EUR = 1.1 USD)
    let amount_usd: i64 = ((amount_eur as f64) * 1.1).round() as i64;
    currency_opts.insert(
        Currency::USD,
        CreateProductDefaultPriceDataCurrencyOptions {
            unit_amount: Some(amount_usd),
            unit_amount_decimal: None,
            tax_behavior: None,
            custom_unit_amount: None,
            tiers: None,
        },
    );

    // SAR (ejemplo: 1 EUR = 4.1 SAR)
    let amount_sar: i64 = ((amount_eur as f64) * 4.1).round() as i64;
    currency_opts.insert(
        Currency::SAR,
        CreateProductDefaultPriceDataCurrencyOptions {
            unit_amount: Some(amount_sar),
            unit_amount_decimal: None,
            tax_behavior: None,
            custom_unit_amount: None,
            tiers: None,
        },
    );
}

#[cfg(test)]
#[path = "../test/services/payments.rs"]
mod extended_tests;
