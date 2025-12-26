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
