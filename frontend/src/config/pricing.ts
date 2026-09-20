import type { PriceStructure } from "@/types/types";

// Nivel de precio según el grupo de países (ver highIncomeCountries en /api/pricing)
export type PriceLevel = "high" | "low";

/**
 * FLAG DE OFERTAS
 * Controla si se muestran (y se cobran) las ofertas en las tarjetas de precios.
 *  - false -> se muestra y se cobra el precio base (BASE_PRICES), sin precio tachado ni badge de ahorro.
 *  - true  -> se muestra el precio de oferta (OFFER_PRICES) con el precio base tachado,
 *             y ese precio de oferta es el que se envía a Stripe.
 *
 * Se puede sobrescribir sin tocar código con la variable de entorno PUBLIC_OFFERS_ENABLED
 * ("true" / "false"). Si no está definida se usa OFFERS_ENABLED_DEFAULT.
 */
const OFFERS_ENABLED_DEFAULT = false;

function parseBooleanFlag(value: unknown, fallback: boolean): boolean {
  if (typeof value === "boolean") return value;
  if (typeof value !== "string" || value.trim() === "") return fallback;
  return ["true", "1", "on", "yes"].includes(value.trim().toLowerCase());
}

export const OFFERS_ENABLED: boolean = parseBooleanFlag(
  import.meta.env.PUBLIC_OFFERS_ENABLED,
  OFFERS_ENABLED_DEFAULT,
);

/**
 * Precios base en EUR por clase (sin oferta).
 * Es la ÚNICA fuente de verdad: lo que se pinta en las tarjetas y lo que se cobra en Stripe
 * (el importe del PaymentIntent se calcula en el frontend a partir de /api/pricing).
 *
 * high -> primer grupo de países (mayor nivel de vida)
 * low  -> segundo grupo de países (menor nivel de vida)
 */
export const BASE_PRICES: Record<PriceLevel, PriceStructure> = {
  high: {
    individual_standard: 30,
    individual_conversation: 25,
    group: 10,
  },
  low: {
    individual_standard: 15,
    individual_conversation: 10,
    group: 3,
  },
};

/**
 * Precios de oferta en EUR por clase. Solo se aplican cuando OFFERS_ENABLED = true.
 * Si para un tipo de clase el precio de oferta es mayor o igual que el base,
 * esa tarjeta se pinta como precio normal (sin badge ni tachado).
 */
export const OFFER_PRICES: Record<PriceLevel, PriceStructure> = {
  high: {
    individual_standard: 20,
    individual_conversation: 20,
    group: 8,
  },
  low: {
    individual_standard: 10,
    individual_conversation: 10,
    group: 3,
  },
};

// Precios efectivos (los que se muestran y se cobran) según el flag de ofertas
export function getActivePrices(level: PriceLevel): PriceStructure {
  return OFFERS_ENABLED ? OFFER_PRICES[level] : BASE_PRICES[level];
}

// Precios tachados: solo existen si las ofertas están activas
export function getOldPrices(level: PriceLevel): PriceStructure | undefined {
  return OFFERS_ENABLED ? BASE_PRICES[level] : undefined;
}

// Países con mayor nivel de vida
export const HIGH_INCOME_COUNTRIES: readonly string[] = [
  // Europa
  "AT", // Austria
  "BE", // Bélgica
  "BG", // Bulgaria
  "HR", // Croacia
  "CY", // Chipre
  "CZ", // Chequia
  "DK", // Dinamarca
  "EE", // Estonia
  "FI", // Finlandia
  "FR", // Francia
  "DE", // Alemania
  "GR", // Grecia
  "HU", // Hungría
  "IE", // Irlanda
  "IT", // Italia
  "LV", // Letonia
  "LT", // Lituania
  "LU", // Luxemburgo
  "MT", // Malta
  "NL", // Países Bajos
  "PL", // Polonia
  "PT", // Portugal
  "RO", // Rumania
  "SK", // Eslovaquia
  "SI", // Eslovenia
  "ES", // España
  "SE", // Suecia
  "GB", // Reino Unido
  "NO", // Noruega
  "CH", // Suiza
  "IS", // Islandia
  "LI", // Liechtenstein
  "MC", // Mónaco
  "SM", // San Marino
  "VA", // Ciudad del Vaticano
  "AD", // Andorra

  // Otros países desarrollados
  "RU", // Rusia
  "JP", // Japón
  "CN", // China
  "AE", // Emiratos Árabes Unidos
  "KW", // Kuwait
  "BH", // Bahréin
  "QA", // Catar
  "SA", // Arabia Saudita
  "OM", // Omán
  "US", // Estados Unidos
  "CA", // Canadá
  "AU", // Australia
  "NZ", // Nueva Zelanda
  "SG", // Singapur
  "AG", // Antigua y Barbuda
  "AW", // Aruba
  "BB", // Barbados
  "BN", // Brunei
  "CW", // Curazao
  "IL", // Israel
  "SC", // Seychelles
];

export const COUNTRY_GROUP_LABELS: Record<PriceLevel, string> = {
  high: "Mayor nivel de vida",
  low: "Menor nivel de vida",
};

export const DEFAULT_COUNTRY = "ES";

export const CURRENCY = "EUR";
export const CURRENCY_SYMBOL = "€";

export function getPriceLevel(country: string): PriceLevel {
  return HIGH_INCOME_COUNTRIES.includes(country) ? "high" : "low";
}
