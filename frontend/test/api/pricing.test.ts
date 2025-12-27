import { describe, it, expect, beforeEach, vi } from "vitest";
import type { PricingApiResponse } from "@/types/types";

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
      expect(data.prices.individual_standard).toBe(30);
      expect(data.prices.individual_conversation).toBe(20);
      expect(data.prices.group).toBe(10);
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
      expect(data.prices.individual_standard).toBe(30);
      expect(data.prices.individual_conversation).toBe(20);
      expect(data.prices.group).toBe(10);
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
      expect(data.countryGroup).toBe("Menor nivel de vida");
      expect(data.prices.individual_standard).toBe(15);
      expect(data.prices.individual_conversation).toBe(10);
      expect(data.prices.group).toBe(4.5);
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

      expect(response.headers.get("Cache-Control")).toBe("public, max-age=3600");
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
      const euCountries = ["AT", "BE", "BG", "HR", "CY", "CZ", "DK", "EE", "FI", "FR", "DE", "GR"];

      for (const country of euCountries) {
        const request = createMockRequest({
          url: `http://localhost/api/pricing?test_country=${country}`,
        });

        const { GET } = await import("@/pages/api/pricing");
        const response = await GET({ request });
        const data: PricingApiResponse = await response.json();

        expect(data.level).toBe("high");
        expect(data.prices.individual_standard).toBe(30);
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
        expect(data.prices.individual_standard).toBe(30);
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
        expect(data.prices.individual_standard).toBe(30);
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
        expect(data.prices.individual_standard).toBe(15);
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

      const highIncomeData: PricingApiResponse = await highIncomeResponse.json();
      const lowIncomeData: PricingApiResponse = await lowIncomeResponse.json();

      expect(Object.keys(highIncomeData).sort()).toEqual(Object.keys(lowIncomeData).sort());
      expect(Object.keys(highIncomeData.prices).sort()).toEqual(Object.keys(lowIncomeData.prices).sort());
    });
  });
});
