#[cfg(test)]
mod tests {
    use {
        crate::{models::stripe::PricePayload, services::payments::insert_options_by_country},
        std::collections::HashMap,
        stripe::{CreateProductDefaultPriceDataCurrencyOptions, Currency},
    };

    #[test]
    fn test_insert_options_by_country_with_base_price() {
        // Arrange: Precio base en EUR
        let base_price = PricePayload {
            currency: "eur".to_string(),
            unit_amount: 1000, // 10.00 EUR
            recurring: None,
        };

        let mut currency_opts: HashMap<Currency, CreateProductDefaultPriceDataCurrencyOptions> =
            HashMap::new();

        // Act: Insertar opciones de moneda
        insert_options_by_country(&mut currency_opts, &base_price);

        // Assert: Verificar que se agregaron USD y SAR
        assert_eq!(currency_opts.len(), 2);
        assert!(currency_opts.contains_key(&Currency::USD));
        assert!(currency_opts.contains_key(&Currency::SAR));

        // Verificar conversión USD (1 EUR = 1.1 USD)
        let usd_option = currency_opts.get(&Currency::USD).unwrap();
        assert_eq!(usd_option.unit_amount, Some(1100)); // 11.00 USD

        // Verificar conversión SAR (1 EUR = 4.1 SAR)
        let sar_option = currency_opts.get(&Currency::SAR).unwrap();
        assert_eq!(sar_option.unit_amount, Some(4100)); // 41.00 SAR
    }

    #[test]
    fn test_insert_options_by_country_with_zero_price() {
        // Arrange: Precio base de 0 EUR
        let base_price = PricePayload {
            currency: "eur".to_string(),
            unit_amount: 0,
            recurring: None,
        };

        let mut currency_opts: HashMap<Currency, CreateProductDefaultPriceDataCurrencyOptions> =
            HashMap::new();

        // Act
        insert_options_by_country(&mut currency_opts, &base_price);

        // Assert: Las conversiones deben resultar en 0
        let usd_option = currency_opts.get(&Currency::USD).unwrap();
        assert_eq!(usd_option.unit_amount, Some(0));

        let sar_option = currency_opts.get(&Currency::SAR).unwrap();
        assert_eq!(sar_option.unit_amount, Some(0));
    }

    #[test]
    fn test_insert_options_by_country_with_large_amount() {
        // Arrange: Precio alto
        let base_price = PricePayload {
            currency: "eur".to_string(),
            unit_amount: 100000, // 1000.00 EUR
            recurring: None,
        };

        let mut currency_opts: HashMap<Currency, CreateProductDefaultPriceDataCurrencyOptions> =
            HashMap::new();

        // Act
        insert_options_by_country(&mut currency_opts, &base_price);

        // Assert
        let usd_option = currency_opts.get(&Currency::USD).unwrap();
        assert_eq!(usd_option.unit_amount, Some(110000)); // 1100.00 USD

        let sar_option = currency_opts.get(&Currency::SAR).unwrap();
        assert_eq!(sar_option.unit_amount, Some(410000)); // 4100.00 SAR
    }

    #[test]
    fn test_insert_options_by_country_rounding() {
        // Arrange: Precio que requiere redondeo
        let base_price = PricePayload {
            currency: "eur".to_string(),
            unit_amount: 333, // 3.33 EUR
            recurring: None,
        };

        let mut currency_opts: HashMap<Currency, CreateProductDefaultPriceDataCurrencyOptions> =
            HashMap::new();

        // Act
        insert_options_by_country(&mut currency_opts, &base_price);

        // Assert: Verificar el redondeo correcto
        let usd_option = currency_opts.get(&Currency::USD).unwrap();
        assert_eq!(usd_option.unit_amount, Some(366)); // 3.33 * 1.1 = 3.663 → 366 (redondeado)

        let sar_option = currency_opts.get(&Currency::SAR).unwrap();
        assert_eq!(sar_option.unit_amount, Some(1365)); // 3.33 * 4.1 = 13.653 → 1365 (redondeado)
    }

    #[test]
    fn test_insert_options_by_country_preserves_existing_entries() {
        // Arrange: HashMap con entrada existente
        let base_price = PricePayload {
            currency: "eur".to_string(),
            unit_amount: 500,
            recurring: None,
        };

        let mut currency_opts: HashMap<Currency, CreateProductDefaultPriceDataCurrencyOptions> =
            HashMap::new();

        // Insertar una entrada existente
        currency_opts.insert(
            Currency::GBP,
            CreateProductDefaultPriceDataCurrencyOptions {
                unit_amount: Some(999),
                unit_amount_decimal: None,
                tax_behavior: None,
                custom_unit_amount: None,
                tiers: None,
            },
        );

        // Act
        insert_options_by_country(&mut currency_opts, &base_price);

        // Assert: Debe tener 3 entradas (GBP original + USD + SAR)
        assert_eq!(currency_opts.len(), 3);
        assert!(currency_opts.contains_key(&Currency::GBP));
        assert!(currency_opts.contains_key(&Currency::USD));
        assert!(currency_opts.contains_key(&Currency::SAR));

        // Verificar que GBP no fue modificado
        let gbp_option = currency_opts.get(&Currency::GBP).unwrap();
        assert_eq!(gbp_option.unit_amount, Some(999));
    }

    #[test]
    fn test_insert_options_by_country_overrides_existing_usd_sar() {
        // Arrange: HashMap con USD y SAR existentes
        let base_price = PricePayload {
            currency: "eur".to_string(),
            unit_amount: 2000,
            recurring: None,
        };

        let mut currency_opts: HashMap<Currency, CreateProductDefaultPriceDataCurrencyOptions> =
            HashMap::new();

        currency_opts.insert(
            Currency::USD,
            CreateProductDefaultPriceDataCurrencyOptions {
                unit_amount: Some(9999),
                unit_amount_decimal: None,
                tax_behavior: None,
                custom_unit_amount: None,
                tiers: None,
            },
        );

        // Act
        insert_options_by_country(&mut currency_opts, &base_price);

        // Assert: USD debe ser sobrescrito con el nuevo valor calculado
        let usd_option = currency_opts.get(&Currency::USD).unwrap();
        assert_eq!(usd_option.unit_amount, Some(2200)); // No 9999
    }

    #[test]
    fn test_insert_options_by_country_currency_options_structure() {
        // Arrange
        let base_price = PricePayload {
            currency: "eur".to_string(),
            unit_amount: 1500,
            recurring: None,
        };

        let mut currency_opts: HashMap<Currency, CreateProductDefaultPriceDataCurrencyOptions> =
            HashMap::new();

        // Act
        insert_options_by_country(&mut currency_opts, &base_price);

        // Assert: Verificar que todos los campos opcionales son None excepto unit_amount
        let usd_option = currency_opts.get(&Currency::USD).unwrap();
        assert!(usd_option.unit_amount.is_some());
        assert!(usd_option.unit_amount_decimal.is_none());
        assert!(usd_option.tax_behavior.is_none());
        assert!(usd_option.custom_unit_amount.is_none());
        assert!(usd_option.tiers.is_none());

        let sar_option = currency_opts.get(&Currency::SAR).unwrap();
        assert!(sar_option.unit_amount.is_some());
        assert!(sar_option.unit_amount_decimal.is_none());
        assert!(sar_option.tax_behavior.is_none());
        assert!(sar_option.custom_unit_amount.is_none());
        assert!(sar_option.tiers.is_none());
    }
}

#[cfg(test)]
mod checkout_tests {
    use {
        crate::{
            models::{
                cal::{BookingStatus, EventTypeCal},
                stripe::PriceTable,
                webhook::Attendee,
            },
            services::payments::{
                PaidClass, RefundDecision, booking_class_slug, booking_has_attendee,
                checkout_amount_cents, is_safe_id, payment_intent_ids_from_relation,
                refund_decision, relation_has_payment_from, secure_eq, verify_cal_signature,
                verify_stripe_signature,
            },
            test_fixtures::fixtures::create_test_booking,
        },
        chrono::{DateTime, Duration, Utc},
        hmac::{Hmac, Mac},
        serde_json::json,
        sha2::Sha256,
    };

    const PRICES: PriceTable = PriceTable {
        individual_standard: 30.0,
        individual_conversation: 25.0,
        group: 3.0,
    };

    fn sign(secret: &str, message: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(message.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    fn attendee(email: &str) -> Attendee {
        serde_json::from_value(json!({
            "email": email,
            "name": "Student",
            "timeZone": "Europe/Madrid",
        }))
        .unwrap()
    }

    #[test]
    fn test_amount_uses_server_price_per_class() {
        assert_eq!(
            checkout_amount_cents(PaidClass::Standard, &PRICES, None),
            Ok(3000)
        );
        assert_eq!(
            checkout_amount_cents(PaidClass::Conversation, &PRICES, None),
            Ok(2500)
        );
        assert_eq!(
            checkout_amount_cents(PaidClass::Group, &PRICES, None),
            Ok(300)
        );
    }

    #[test]
    fn test_amount_multiplies_weeks_by_classes_per_week() {
        assert_eq!(
            checkout_amount_cents(PaidClass::Group, &PRICES, Some(4)),
            Ok(2400)
        );
        assert_eq!(
            checkout_amount_cents(PaidClass::Group, &PRICES, Some(12)),
            Ok(7200)
        );
    }

    #[test]
    fn test_amount_rejects_invalid_weeks() {
        assert!(checkout_amount_cents(PaidClass::Group, &PRICES, Some(0)).is_err());
        assert!(checkout_amount_cents(PaidClass::Group, &PRICES, Some(13)).is_err());
    }

    #[test]
    fn test_amount_rejects_invalid_prices() {
        for price in [0.0, -10.0, f64::NAN, f64::INFINITY, 0.2, 5000.0] {
            let prices = PriceTable {
                individual_standard: price,
                ..PRICES
            };
            assert!(
                checkout_amount_cents(PaidClass::Standard, &prices, None).is_err(),
                "price {} should be rejected",
                price
            );
        }
    }

    #[test]
    fn test_paid_class_slugs() {
        for class in [
            PaidClass::Standard,
            PaidClass::Conversation,
            PaidClass::Group,
        ] {
            assert_eq!(PaidClass::from_slug(class.slug()), Some(class));
        }
        assert_eq!(PaidClass::from_slug("free-class"), None);
        assert_eq!(PaidClass::from_slug(""), None);
    }

    #[test]
    fn test_booking_class_slug_prefers_event_type() {
        let mut booking = create_test_booking("uid", BookingStatus::Pending);
        assert_eq!(booking_class_slug(&booking), None);

        booking.event_type_slug = Some("group-class".to_string());
        assert_eq!(booking_class_slug(&booking).as_deref(), Some("group-class"));

        booking.event_type = Some(EventTypeCal {
            id: Some(1),
            slug: Some("standard-class".to_string()),
            title: None,
        });
        assert_eq!(
            booking_class_slug(&booking).as_deref(),
            Some("standard-class")
        );
    }

    #[test]
    fn test_booking_has_attendee_checks_attendees_and_guests() {
        let mut booking = create_test_booking("uid", BookingStatus::Accepted);
        booking.attendees = vec![attendee("Student@Example.com")];
        booking.guests = Some(vec!["guest@example.com".to_string()]);

        assert!(booking_has_attendee(&booking, "student@example.com"));
        assert!(booking_has_attendee(&booking, "GUEST@example.com"));
        assert!(!booking_has_attendee(&booking, "other@example.com"));
    }

    #[test]
    fn test_is_safe_id() {
        assert!(is_safe_id("abc-123_XYZ"));
        assert!(!is_safe_id(""));
        assert!(!is_safe_id("abc/../x"));
        assert!(!is_safe_id("abc.json?auth=x"));
    }

    #[test]
    fn test_secure_eq() {
        assert!(secure_eq("secret", "secret"));
        assert!(!secure_eq("secret", "secreT"));
        assert!(!secure_eq("secret", "secret2"));
        assert!(!secure_eq("", "secret"));
    }

    #[test]
    fn test_verify_cal_signature() {
        let body = r#"{"triggerEvent":"BOOKING_CANCELLED"}"#;
        let signature = sign("cal_secret", body);

        assert!(verify_cal_signature(
            body.as_bytes(),
            &signature,
            "cal_secret"
        ));
        assert!(!verify_cal_signature(body.as_bytes(), &signature, "other"));
        assert!(!verify_cal_signature(b"{}", &signature, "cal_secret"));
        assert!(!verify_cal_signature(
            body.as_bytes(),
            "not-hex",
            "cal_secret"
        ));
        assert!(!verify_cal_signature(body.as_bytes(), "", "cal_secret"));
    }

    #[test]
    fn test_verify_stripe_signature() {
        let payload = r#"{"type":"payment_intent.succeeded"}"#;
        let now = 1_700_000_000;
        let signature = sign("whsec_test", &format!("{}.{}", now, payload));
        let header = format!("t={},v1={}", now, signature);

        assert!(verify_stripe_signature(
            payload.as_bytes(),
            &header,
            "whsec_test",
            now + 10
        ));
        assert!(!verify_stripe_signature(
            payload.as_bytes(),
            &header,
            "whsec_other",
            now
        ));
        assert!(!verify_stripe_signature(b"{}", &header, "whsec_test", now));
        assert!(!verify_stripe_signature(
            payload.as_bytes(),
            &header,
            "whsec_test",
            now + 301
        ));
        assert!(!verify_stripe_signature(
            payload.as_bytes(),
            &format!("v1={}", signature),
            "whsec_test",
            now
        ));
    }

    #[test]
    fn test_verify_stripe_signature_accepts_any_valid_v1() {
        let payload = "{}";
        let now = 1_700_000_000;
        let signature = sign("whsec_test", &format!("{}.{}", now, payload));
        let header = format!("t={}, v1=deadbeef, v1={}, v0=abc", now, signature);

        assert!(verify_stripe_signature(
            payload.as_bytes(),
            &header,
            "whsec_test",
            now
        ));
    }

    #[test]
    fn test_payment_intent_ids_from_relation() {
        assert!(payment_intent_ids_from_relation(&json!(null)).is_empty());

        assert_eq!(
            payment_intent_ids_from_relation(&json!({ "stripe_id": "pi_legacy" })),
            vec!["pi_legacy".to_string()]
        );

        let mut ids = payment_intent_ids_from_relation(&json!({
            "stripe_id": "pi_legacy",
            "pi_one": { "email": "a@example.com", "amount": 300 },
            "pi_two": { "email": "b@example.com", "amount": 300 },
            "other": "ignored",
        }));
        ids.sort();
        assert_eq!(ids, vec!["pi_legacy", "pi_one", "pi_two"]);
    }

    fn cancelled_booking(
        start_in_hours: i64,
        cancelled_by: Option<&str>,
    ) -> (crate::models::cal::CalBookingPayload, DateTime<Utc>) {
        let now: DateTime<Utc> = "2026-10-25T10:00:00Z".parse().unwrap();
        let mut booking = create_test_booking("uid", BookingStatus::Cancelled);
        booking.attendees = vec![attendee("student@example.com")];
        booking.start_time = Some((now + Duration::hours(start_in_hours)).to_rfc3339());
        booking.cancelled_by_email = cancelled_by.map(str::to_string);
        (booking, now)
    }

    #[test]
    fn test_refund_when_student_cancels_with_notice() {
        let (booking, now) = cancelled_booking(48, Some("student@example.com"));
        assert_eq!(refund_decision(&booking, now), RefundDecision::Refund);

        let (booking, now) = cancelled_booking(24, Some("student@example.com"));
        assert_eq!(refund_decision(&booking, now), RefundDecision::Refund);
    }

    #[test]
    fn test_no_refund_when_student_cancels_late() {
        let (booking, now) = cancelled_booking(23, Some("STUDENT@example.com"));
        assert_eq!(
            refund_decision(&booking, now),
            RefundDecision::LateCancellation
        );

        let (booking, now) = cancelled_booking(-1, Some("student@example.com"));
        assert_eq!(
            refund_decision(&booking, now),
            RefundDecision::LateCancellation
        );
    }

    #[test]
    fn test_unknown_canceller_follows_notice_rule() {
        let (booking, now) = cancelled_booking(48, None);
        assert_eq!(refund_decision(&booking, now), RefundDecision::Refund);

        let (booking, now) = cancelled_booking(2, None);
        assert_eq!(
            refund_decision(&booking, now),
            RefundDecision::LateCancellation
        );
    }

    #[test]
    fn test_empty_canceller_follows_notice_rule() {
        let (booking, now) = cancelled_booking(2, Some(""));
        assert_eq!(
            refund_decision(&booking, now),
            RefundDecision::LateCancellation
        );

        let (booking, now) = cancelled_booking(48, Some("  "));
        assert_eq!(refund_decision(&booking, now), RefundDecision::Refund);
    }

    #[test]
    fn test_refund_when_teacher_cancels_even_late() {
        let (booking, now) = cancelled_booking(1, Some("teacher@amanahacademia.com"));
        assert_eq!(refund_decision(&booking, now), RefundDecision::Refund);
    }

    #[test]
    fn test_reschedule_transfers_payment() {
        let (mut booking, now) = cancelled_booking(1, Some("student@example.com"));
        booking.rescheduled_to_uid = Some("new-uid".to_string());
        assert_eq!(
            refund_decision(&booking, now),
            RefundDecision::Transfer("new-uid".to_string())
        );
    }

    #[test]
    fn test_refund_when_booking_rejected() {
        let (mut booking, now) = cancelled_booking(1, None);
        booking.status = BookingStatus::Rejected;
        assert_eq!(refund_decision(&booking, now), RefundDecision::Refund);
    }

    #[test]
    fn test_no_refund_when_booking_not_cancelled() {
        for status in [
            BookingStatus::Accepted,
            BookingStatus::Pending,
            BookingStatus::AwaitingHost,
        ] {
            let (mut booking, now) = cancelled_booking(48, None);
            booking.status = status;
            assert_eq!(refund_decision(&booking, now), RefundDecision::NotCancelled);
        }
    }

    #[test]
    fn test_notice_uses_cancellation_time_when_available() {
        let (mut booking, now) = cancelled_booking(20, Some("student@example.com"));
        booking.updated_at = Some((now - Duration::hours(10)).to_rfc3339());
        assert_eq!(refund_decision(&booking, now), RefundDecision::Refund);

        booking.updated_at = Some((now + Duration::hours(10)).to_rfc3339());
        assert_eq!(
            refund_decision(&booking, now),
            RefundDecision::LateCancellation
        );
    }

    #[test]
    fn test_no_refund_without_start_time() {
        let (mut booking, now) = cancelled_booking(48, None);
        booking.start_time = None;
        assert_eq!(
            refund_decision(&booking, now),
            RefundDecision::LateCancellation
        );
    }

    #[test]
    fn test_parses_awaiting_host_status() {
        let status: BookingStatus = serde_json::from_value(json!("awaiting_host")).unwrap();
        assert_eq!(status, BookingStatus::AwaitingHost);
    }

    #[test]
    fn test_relation_has_payment_from() {
        let relation = json!({
            "stripe_id": "pi_legacy",
            "pi_one": { "email": "Student@Example.com", "amount": 300 },
        });

        assert!(relation_has_payment_from(&relation, "student@example.com"));
        assert!(!relation_has_payment_from(&relation, "other@example.com"));
        assert!(!relation_has_payment_from(
            &json!(null),
            "student@example.com"
        ));
    }
}
