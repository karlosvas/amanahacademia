import {
  COUNTRY_GROUP_LABELS,
  CURRENCY,
  CURRENCY_SYMBOL,
  getActivePrices,
  getOldPrices,
  getPriceLevel,
  OFFERS_ENABLED,
  resolveCountry,
} from "@/config/pricing";
import type { PricingApiResponse } from "@/types/types";

export async function GET({ request }: { request: Request }) {
  const url = new URL(request.url);

  const country = resolveCountry(request);

  const isDevelopment =
    url.hostname === "localhost" ||
    url.hostname === "127.0.0.1" ||
    url.hostname.includes("local");

  const level = getPriceLevel(country);

  const pricing: PricingApiResponse = {
    currency: CURRENCY,
    symbol: CURRENCY_SYMBOL,
    level,
    countryGroup: COUNTRY_GROUP_LABELS[level],
    isDevelopment,
    country,
    offers_enabled: OFFERS_ENABLED,
    old_prices: getOldPrices(level),
    prices: getActivePrices(level),
  };

  return new Response(JSON.stringify(pricing), {
    headers: {
      "Content-Type": "application/json",
      "Cache-Control": isDevelopment ? "no-cache" : "public, max-age=3600",
    },
  });
}
