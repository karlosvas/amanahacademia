import { describe, it, expect, beforeEach, vi, afterEach } from "vitest";
import {
  syncTokenWithServer,
  syncLogoutWithServer,
  getPricingByCountry,
} from "@/utils/auth";
import * as firebase from "@/services/firebase";
import type { PricingApiResponse } from "@/types/types";

// Mock del servicio de Firebase
vi.mock("@/services/firebase", () => ({
  getCurrentUserToken: vi.fn(),
}));

describe("Auth Utilities", () => {
  let fetchMock: ReturnType<typeof vi.fn<typeof fetch>>;

  beforeEach(() => {
    // Mock de fetch global
    fetchMock = vi.fn<typeof fetch>();
    globalThis.fetch = fetchMock as typeof fetch;

    // Reset de mocks
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe("syncTokenWithServer", () => {
    it("should sync token with server when token is available", async () => {
      const mockToken = "test-firebase-token-12345";
      vi.mocked(firebase.getCurrentUserToken).mockResolvedValue(mockToken);

      fetchMock.mockResolvedValue(
        new Response(JSON.stringify({ success: true }), { status: 200 }),
      );

      await syncTokenWithServer();

      expect(firebase.getCurrentUserToken).toHaveBeenCalledTimes(1);
      expect(fetchMock).toHaveBeenCalledWith("/api/auth", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ token: mockToken }),
      });
    });

    it("should not call fetch when token is null", async () => {
      vi.mocked(firebase.getCurrentUserToken).mockResolvedValue(null);

      await syncTokenWithServer();

      expect(firebase.getCurrentUserToken).toHaveBeenCalledTimes(1);
      expect(fetchMock).not.toHaveBeenCalled();
    });

    it("should not call fetch when token is undefined", async () => {
      vi.mocked(firebase.getCurrentUserToken).mockResolvedValue(null);

      await syncTokenWithServer();

      expect(firebase.getCurrentUserToken).toHaveBeenCalledTimes(1);
      expect(fetchMock).not.toHaveBeenCalled();
    });

    it("should handle fetch errors gracefully", async () => {
      const mockToken = "test-token";
      vi.mocked(firebase.getCurrentUserToken).mockResolvedValue(mockToken);

      fetchMock.mockRejectedValue(new Error("Network error"));

      await expect(syncTokenWithServer()).rejects.toThrow("Network error");
      expect(firebase.getCurrentUserToken).toHaveBeenCalledTimes(1);
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });

    it("should handle server errors (non-ok response)", async () => {
      const mockToken = "test-token";
      vi.mocked(firebase.getCurrentUserToken).mockResolvedValue(mockToken);

      fetchMock.mockResolvedValue(
        new Response(null, {
          status: 500,
          statusText: "Internal Server Error",
        }),
      );

      // La función no lanza error pero el fetch se completa
      await syncTokenWithServer();

      expect(fetchMock).toHaveBeenCalledWith("/api/auth", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ token: mockToken }),
      });
    });

    it("should handle empty token string", async () => {
      vi.mocked(firebase.getCurrentUserToken).mockResolvedValue("");

      await syncTokenWithServer();

      expect(firebase.getCurrentUserToken).toHaveBeenCalledTimes(1);
      expect(fetchMock).not.toHaveBeenCalled();
    });
  });

  describe("syncLogoutWithServer", () => {
    it("should call DELETE on /api/auth endpoint", async () => {
      await syncLogoutWithServer();

      expect(fetchMock).toHaveBeenCalledWith("/api/auth", {
        method: "DELETE",
      });
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });

    it("should handle fetch errors during logout", async () => {
      fetchMock.mockRejectedValue(new Error("Network error"));

      await expect(syncLogoutWithServer()).rejects.toThrow("Network error");
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });

    it("should handle server errors during logout", async () => {
      fetchMock.mockResolvedValue(
        new Response(null, {
          status: 500,
          statusText: "Internal Server Error",
        }),
      );

      // La función no lanza error pero el fetch se completa
      await syncLogoutWithServer();

      expect(fetchMock).toHaveBeenCalledWith("/api/auth", {
        method: "DELETE",
      });
    });

    it("should complete even with timeout errors", async () => {
      fetchMock.mockRejectedValue(new Error("Request timeout"));

      await expect(syncLogoutWithServer()).rejects.toThrow("Request timeout");
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });
  });

  describe("getPricingByCountry", () => {
    const mockPricingResponse: PricingApiResponse = {
      country: "ES",
      currency: "EUR",
      symbol: "€",
      level: "high",
      countryGroup: "Europe",
      isDevelopment: false,
      prices: {
        individual_standard: 25,
        individual_conversation: 30,
        group: 15,
      },
    };

    beforeEach(() => {
      // Mock de globalThis.location
      Object.defineProperty(globalThis, "location", {
        value: {
          search: "",
          href: "http://localhost",
        },
        writable: true,
        configurable: true,
      });
    });

    it("should fetch pricing data without test_country parameter", async () => {
      fetchMock.mockResolvedValue(
        new Response(JSON.stringify(mockPricingResponse), { status: 200 }),
      );

      const result = await getPricingByCountry();

      expect(fetchMock).toHaveBeenCalledWith("/api/pricing");
      expect(result).toEqual(mockPricingResponse);
    });

    it("should fetch pricing data with test_country parameter from URL", async () => {
      Object.defineProperty(globalThis, "location", {
        value: {
          search: "?test_country=US",
          href: "http://localhost?test_country=US",
        },
        writable: true,
        configurable: true,
      });

      fetchMock.mockResolvedValue(
        new Response(
          JSON.stringify({
            ...mockPricingResponse,
            country: "US",
            currency: "USD",
            symbol: "$",
          }),
          { status: 200 },
        ),
      );

      const result = await getPricingByCountry();

      expect(fetchMock).toHaveBeenCalledWith("/api/pricing?test_country=US");
      expect(result.country).toBe("US");
      expect(result.currency).toBe("USD");
    });

    it("should handle multiple URL parameters correctly", async () => {
      Object.defineProperty(globalThis, "location", {
        value: {
          search: "?lang=en&test_country=FR&theme=dark",
          href: "http://localhost?lang=en&test_country=FR&theme=dark",
        },
        writable: true,
        configurable: true,
      });

      fetchMock.mockResolvedValue(
        new Response(JSON.stringify(mockPricingResponse), { status: 200 }),
      );

      await getPricingByCountry();

      expect(fetchMock).toHaveBeenCalledWith("/api/pricing?test_country=FR");
    });

    it("should throw error when response is not ok", async () => {
      fetchMock.mockResolvedValue(
        new Response(null, { status: 404, statusText: "Not Found" }),
      );

      await expect(getPricingByCountry()).rejects.toThrow(
        "HTTP error! status: 404",
      );
    });

    it("should throw error on network failure", async () => {
      fetchMock.mockRejectedValue(new Error("Network request failed"));

      await expect(getPricingByCountry()).rejects.toThrow(
        "Network request failed",
      );
    });

    it("should handle 500 server errors", async () => {
      fetchMock.mockResolvedValue(
        new Response(null, {
          status: 500,
          statusText: "Internal Server Error",
        }),
      );

      await expect(getPricingByCountry()).rejects.toThrow(
        "HTTP error! status: 500",
      );
    });

    it("should parse JSON response correctly", async () => {
      const customPricing: PricingApiResponse = {
        country: "MX",
        currency: "MXN",
        symbol: "$",
        level: "low",
        countryGroup: "Latin America",
        isDevelopment: false,
        prices: {
          individual_standard: 400,
          individual_conversation: 500,
          group: 250,
        },
      };

      fetchMock.mockResolvedValue(
        new Response(JSON.stringify(customPricing), { status: 200 }),
      );

      const result = await getPricingByCountry();

      expect(result).toEqual(customPricing);
      expect(result.prices.individual_standard).toBe(400);
      expect(result.level).toBe("low");
    });

    it("should handle empty test_country parameter", async () => {
      Object.defineProperty(globalThis, "location", {
        value: {
          search: "?test_country=",
          href: "http://localhost?test_country=",
        },
        writable: true,
        configurable: true,
      });

      fetchMock.mockResolvedValue(
        new Response(JSON.stringify(mockPricingResponse), { status: 200 }),
      );

      await getPricingByCountry();

      // Empty string is falsy, so it should not add the parameter
      expect(fetchMock).toHaveBeenCalledWith("/api/pricing");
    });

    it("should handle invalid JSON response", async () => {
      const invalidResponse = new Response("invalid json", { status: 200 });
      // Override json() to throw an error
      invalidResponse.json = async () => {
        throw new Error("Invalid JSON");
      };
      fetchMock.mockResolvedValue(invalidResponse);

      await expect(getPricingByCountry()).rejects.toThrow("Invalid JSON");
    });

    it("should handle pricing data with isDevelopment flag", async () => {
      const devPricing: PricingApiResponse = {
        ...mockPricingResponse,
        isDevelopment: true,
        country: "TEST",
      };

      fetchMock.mockResolvedValue(
        new Response(JSON.stringify(devPricing), { status: 200 }),
      );

      const result = await getPricingByCountry();

      expect(result.isDevelopment).toBe(true);
      expect(result.country).toBe("TEST");
    });

    it("should preserve all pricing response properties", async () => {
      fetchMock.mockResolvedValue(
        new Response(JSON.stringify(mockPricingResponse), { status: 200 }),
      );

      const result = await getPricingByCountry();

      expect(result).toHaveProperty("country");
      expect(result).toHaveProperty("currency");
      expect(result).toHaveProperty("symbol");
      expect(result).toHaveProperty("level");
      expect(result).toHaveProperty("countryGroup");
      expect(result).toHaveProperty("isDevelopment");
      expect(result).toHaveProperty("prices");
      expect(result.prices).toHaveProperty("individual_standard");
      expect(result.prices).toHaveProperty("individual_conversation");
      expect(result.prices).toHaveProperty("group");
    });

    it("should handle URL encoding in test_country parameter", async () => {
      Object.defineProperty(globalThis, "location", {
        value: {
          search: "?test_country=GB%20UK",
          href: "http://localhost?test_country=GB%20UK",
        },
        writable: true,
        configurable: true,
      });

      fetchMock.mockResolvedValue(
        new Response(JSON.stringify(mockPricingResponse), { status: 200 }),
      );

      await getPricingByCountry();

      expect(fetchMock).toHaveBeenCalledWith("/api/pricing?test_country=GB UK");
    });
  });
});
