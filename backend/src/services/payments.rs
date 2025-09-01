use crate::models::stripe::{CurrencyMap, PricePayload};
use stripe::{Currency, CreateProductDefaultPriceDataCurrencyOptions};

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
    let amount_sar = ((amount_eur as f64) * 4.1).round() as i64;
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
