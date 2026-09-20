import { describe, it, expect, beforeEach, vi } from "vitest";
import {
  COUNTRY_GROUP_LABELS,
  getActivePrices,
  getOldPrices,
  OFFERS_ENABLED,
} from "@/config/pricing";
import type { PricingApiResponse } from "@/types/types";

const highPrices = getActivePrices("high");
const lowPrices = getActivePrices("low");

describe("API Routes - Pricing", () => {
  describe("GET /api/pricing", () => {
    const createMockRequest = (options: {
      url: string;
      headers?: Record<string, string>;
    }): Request => {
      const headers = new Headers(options.headers || {});
      return {
        url: options.url,
        headers,
      } as Request;
    };

    it("should return default pricing for ES when no parameters", async () => {
      const request = createMockRequest({
        url: "http://localhost/api/pricing",
      });

      const { GET } = await import("@/pages/api/pricing");
      const response = await GET({ request });

      const data: PricingApiResponse = await response.json();

      expect(data.country).toBe("ES");
      expect(data.currency).toBe("EUR");
      expect(data.symbol).toBe("€");
      expect(data.level).toBe("high");
      expect(data.isDevelopment).toBe(true);
      expect(data.prices).toEqual(highPrices);
    });

    it("should return pricing for US with test_country parameter", async () => {
      const request = createMockRequest({
        url: "http://localhost/api/pricing?test_country=US",
      });

      const { GET } = await import("@/pages/api/pricing");
      const response = await GET({ request });

      const data: PricingApiResponse = await response.json();

      expect(data.country).toBe("US");
      expect(data.level).toBe("high");
      expect(data.prices).toEqual(highPrices);
    });

    it("should return low income pricing for non-listed country", async () => {
      const request = createMockRequest({
        url: "http://localhost/api/pricing?test_country=MX",
      });

      const { GET } = await import("@/pages/api/pricing");
      const response = await GET({ request });

      const data: PricingApiResponse = await response.json();

      expect(data.country).toBe("MX");
      expect(data.level).toBe("low");
      expect(data.countryGroup).toBe(COUNTRY_GROUP_LABELS.low);
      expect(data.prices).toEqual(lowPrices);
    });

    it("should use CF-IPCountry header when available", async () => {
      const request = createMockRequest({
        url: "http://example.com/api/pricing",
        headers: {
          "CF-IPCountry": "FR",
        },
      });

      const { GET } = await import("@/pages/api/pricing");
      const response = await GET({ request });

      const data: PricingApiResponse = await response.json();

      expect(data.country).toBe("FR");
      expect(data.level).toBe("high");
    });

    it("should use x-vercel-ip-country header when CF-IPCountry is not available", async () => {
      const request = createMockRequest({
        url: "http://example.com/api/pricing",
        headers: {
          "x-vercel-ip-country": "DE",
        },
      });

      const { GET } = await import("@/pages/api/pricing");
      const response = await GET({ request });

      const data: PricingApiResponse = await response.json();

      expect(data.country).toBe("DE");
      expect(data.level).toBe("high");
    });

    it("should prioritize test_country over headers", async () => {
      const request = createMockRequest({
        url: "http://localhost/api/pricing?test_country=IT",
        headers: {
          "CF-IPCountry": "US",
          "x-vercel-ip-country": "FR",
        },
      });

      const { GET } = await import("@/pages/api/pricing");
      const response = await GET({ request });

      const data: PricingApiResponse = await response.json();

      expect(data.country).toBe("IT");
    });

    it("should detect development environment from localhost", async () => {
      const request = createMockRequest({
        url: "http://localhost/api/pricing",
      });

      const { GET } = await import("@/pages/api/pricing");
      const response = await GET({ request });

      const data: PricingApiResponse = await response.json();

      expect(data.isDevelopment).toBe(true);
    });

    it("should detect development environment from 127.0.0.1", async () => {
      const request = createMockRequest({
        url: "http://127.0.0.1/api/pricing",
      });

      const { GET } = await import("@/pages/api/pricing");
      const response = await GET({ request });

      const data: PricingApiResponse = await response.json();

      expect(data.isDevelopment).toBe(true);
    });

    it("should detect production environment", async () => {
      const request = createMockRequest({
        url: "https://example.com/api/pricing",
      });

      const { GET } = await import("@/pages/api/pricing");
      const response = await GET({ request });

      const data: PricingApiResponse = await response.json();

      expect(data.isDevelopment).toBe(false);
    });

    it("should return correct cache headers for development", async () => {
      const request = createMockRequest({
        url: "http://localhost/api/pricing",
      });

      const { GET } = await import("@/pages/api/pricing");
      const response = await GET({ request });

      expect(response.headers.get("Cache-Control")).toBe("no-cache");
    });

    it("should return correct cache headers for production", async () => {
      const request = createMockRequest({
        url: "https://example.com/api/pricing",
      });

      const { GET } = await import("@/pages/api/pricing");
      const response = await GET({ request });

      expect(response.headers.get("Cache-Control")).toBe(
        "public, max-age=3600",
      );
    });

    it("should return correct Content-Type header", async () => {
      const request = createMockRequest({
        url: "http://localhost/api/pricing",
      });

      const { GET } = await import("@/pages/api/pricing");
      const response = await GET({ request });

      expect(response.headers.get("Content-Type")).toBe("application/json");
    });

    it("should return high income pricing for all European Union countries", async () => {
      const euCountries = [
        "AT",
        "BE",
        "BG",
        "HR",
        "CY",
        "CZ",
        "DK",
        "EE",
        "FI",
        "FR",
        "DE",
        "GR",
      ];

      for (const country of euCountries) {
        const request = createMockRequest({
          url: `http://localhost/api/pricing?test_country=${country}`,
        });

        const { GET } = await import("@/pages/api/pricing");
        const response = await GET({ request });
        const data: PricingApiResponse = await response.json();

        expect(data.level).toBe("high");
        expect(data.prices).toEqual(highPrices);
      }
    });

    it("should return high income pricing for Middle Eastern countries", async () => {
      const middleEastCountries = ["AE", "KW", "BH", "QA", "SA"];

      for (const country of middleEastCountries) {
        const request = createMockRequest({
          url: `http://localhost/api/pricing?test_country=${country}`,
        });

        const { GET } = await import("@/pages/api/pricing");
        const response = await GET({ request });
        const data: PricingApiResponse = await response.json();

        expect(data.level).toBe("high");
        expect(data.prices).toEqual(highPrices);
      }
    });

    it("should return high income pricing for Asian developed countries", async () => {
      const asianCountries = ["JP", "SG", "BN"];

      for (const country of asianCountries) {
        const request = createMockRequest({
          url: `http://localhost/api/pricing?test_country=${country}`,
        });

        const { GET } = await import("@/pages/api/pricing");
        const response = await GET({ request });
        const data: PricingApiResponse = await response.json();

        expect(data.level).toBe("high");
        expect(data.prices).toEqual(highPrices);
      }
    });

    it("should return low income pricing for Latin American countries", async () => {
      const latinAmericanCountries = ["MX", "AR", "BR", "CL", "CO"];

      for (const country of latinAmericanCountries) {
        const request = createMockRequest({
          url: `http://localhost/api/pricing?test_country=${country}`,
        });

        const { GET } = await import("@/pages/api/pricing");
        const response = await GET({ request });
        const data: PricingApiResponse = await response.json();

        expect(data.level).toBe("low");
        expect(data.prices).toEqual(lowPrices);
      }
    });

    it("should handle empty test_country parameter", async () => {
      const request = createMockRequest({
        url: "http://localhost/api/pricing?test_country=",
      });

      const { GET } = await import("@/pages/api/pricing");
      const response = await GET({ request });
      const data: PricingApiResponse = await response.json();

      // Should fall back to default (ES)
      expect(data.country).toBe("ES");
    });

    it("should include all required pricing properties", async () => {
      const request = createMockRequest({
        url: "http://localhost/api/pricing",
      });

      const { GET } = await import("@/pages/api/pricing");
      const response = await GET({ request });
      const data: PricingApiResponse = await response.json();

      expect(data).toHaveProperty("country");
      expect(data).toHaveProperty("currency");
      expect(data).toHaveProperty("symbol");
      expect(data).toHaveProperty("level");
      expect(data).toHaveProperty("countryGroup");
      expect(data).toHaveProperty("isDevelopment");
      expect(data).toHaveProperty("prices");
      expect(data.prices).toHaveProperty("individual_standard");
      expect(data.prices).toHaveProperty("individual_conversation");
      expect(data.prices).toHaveProperty("group");
    });

    it("should handle URL with multiple parameters", async () => {
      const request = createMockRequest({
        url: "http://localhost/api/pricing?lang=es&test_country=GB&theme=dark",
      });

      const { GET } = await import("@/pages/api/pricing");
      const response = await GET({ request });
      const data: PricingApiResponse = await response.json();

      expect(data.country).toBe("GB");
      expect(data.level).toBe("high");
    });

    it("should return consistent data structure for both high and low income countries", async () => {
      const highIncomeRequest = createMockRequest({
        url: "http://localhost/api/pricing?test_country=US",
      });

      const lowIncomeRequest = createMockRequest({
        url: "http://localhost/api/pricing?test_country=MX",
      });

      const { GET } = await import("@/pages/api/pricing");
      const highIncomeResponse = await GET({ request: highIncomeRequest });
      const lowIncomeResponse = await GET({ request: lowIncomeRequest });

      const highIncomeData: PricingApiResponse =
        await highIncomeResponse.json();
      const lowIncomeData: PricingApiResponse = await lowIncomeResponse.json();

      expect(Object.keys(highIncomeData).sort()).toEqual(
        Object.keys(lowIncomeData).sort(),
      );
      expect(Object.keys(highIncomeData.prices).sort()).toEqual(
        Object.keys(lowIncomeData.prices).sort(),
      );
    });
  });
});

describe("Config - src/config/pricing.ts", () => {
  describe("Flag de ofertas", () => {
    it("expone en la respuesta si las ofertas están activas", async () => {
      const request = {
        url: "http://localhost/api/pricing",
        headers: new Headers(),
      } as Request;

      const { GET } = await import("@/pages/api/pricing");
      const data: PricingApiResponse = await (await GET({ request })).json();

      expect(data.offers_enabled).toBe(OFFERS_ENABLED);
    });

    it("solo envía old_prices (precio tachado) cuando hay oferta", async () => {
      const request = {
        url: "http://localhost/api/pricing",
        headers: new Headers(),
      } as Request;

      const { GET } = await import("@/pages/api/pricing");
      const data: PricingApiResponse = await (await GET({ request })).json();

      if (OFFERS_ENABLED) {
        expect(data.old_prices).toEqual(getOldPrices("high"));
      } else {
        expect(data.old_prices).toBeUndefined();
      }
    });

    it("con el flag activo cobra la oferta y tacha el precio base", async () => {
      vi.resetModules();
      vi.stubEnv("PUBLIC_OFFERS_ENABLED", "true");

      const {
        OFFERS_ENABLED: enabled,
        BASE_PRICES,
        OFFER_PRICES,
        getActivePrices: active,
        getOldPrices: old,
      } = await import("@/config/pricing");

      expect(enabled).toBe(true);
      expect(active("high")).toEqual(OFFER_PRICES.high);
      expect(old("high")).toEqual(BASE_PRICES.high);

      vi.unstubAllEnvs();
      vi.resetModules();
    });

    it("con el flag apagado cobra el precio base y no tacha nada", async () => {
      vi.resetModules();
      vi.stubEnv("PUBLIC_OFFERS_ENABLED", "false");

      const {
        OFFERS_ENABLED: enabled,
        BASE_PRICES,
        getActivePrices: active,
        getOldPrices: old,
      } = await import("@/config/pricing");

      expect(enabled).toBe(false);
      expect(active("low")).toEqual(BASE_PRICES.low);
      expect(old("low")).toBeUndefined();

      vi.unstubAllEnvs();
      vi.resetModules();
    });
  });

  describe("Coherencia de la tabla de precios", () => {
    it("los dos grupos definen las tres modalidades con importes positivos", async () => {
      const { BASE_PRICES, OFFER_PRICES } = await import("@/config/pricing");

      for (const table of [BASE_PRICES, OFFER_PRICES]) {
        for (const level of ["high", "low"] as const) {
          const prices = table[level];
          expect(Object.keys(prices).sort()).toEqual([
            "group",
            "individual_conversation",
            "individual_standard",
          ]);
          for (const amount of Object.values(prices)) {
            expect(amount).toBeGreaterThan(0);
          }
        }
      }
    });

    it("el primer grupo de países nunca paga menos que el segundo", async () => {
      const { BASE_PRICES } = await import("@/config/pricing");

      for (const key of Object.keys(BASE_PRICES.high) as Array<
        keyof typeof BASE_PRICES.high
      >) {
        expect(BASE_PRICES.high[key]).toBeGreaterThanOrEqual(
          BASE_PRICES.low[key],
        );
      }
    });
  });
});
