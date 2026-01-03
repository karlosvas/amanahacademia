import { describe, it, expect, beforeEach, afterEach, vi, type Mock } from "vitest";
import { initCalendar, updatePricing, getPrice } from "@/services/calendar";
import { Class } from "@/enums/enums";
import type { PricingApiResponse } from "@/types/types";
import { getPricingByCountry } from "@/utils/auth";

// Mocks
vi.mock("@/utils/auth", () => ({
  getPricingByCountry: vi.fn(),
}));

describe("calendar.ts", () => {
  let consoleErrorSpy: any;

  beforeEach(() => {
    vi.clearAllMocks();
    consoleErrorSpy = vi.spyOn(console, "error").mockImplementation(() => {});

    // Reset DOM
    document.head.innerHTML = "";
    document.body.innerHTML = "";

    // Delete any existing Cal object
    delete (globalThis as any).Cal;
  });

  afterEach(() => {
    consoleErrorSpy.mockRestore();
    vi.restoreAllMocks();
    delete (globalThis as any).Cal;
  });

  //////////////////// initCalendar ////////////////////
  describe("initCalendar", () => {
    it("should initialize calendar and return true", () => {
      const result = initCalendar(Class.Standard);

      expect(result).toBe(true);
    });

    it("should create Cal global object if not exists", () => {
      expect((globalThis as any).Cal).toBeUndefined();

      initCalendar(Class.Standard);

      expect((globalThis as any).Cal).toBeDefined();
      expect(typeof (globalThis as any).Cal).toBe("function");
    });

    it("should load Cal.com embed script", () => {
      initCalendar(Class.Standard);

      const scripts = document.head.querySelectorAll('script[src="https://app.cal.com/embed/embed.js"]');
      expect(scripts.length).toBeGreaterThan(0);
    });

    it("should set Cal.loaded to true after initialization", () => {
      initCalendar(Class.Standard);

      expect((globalThis as any).Cal.loaded).toBe(true);
    });

    it("should create namespace for the calendar", () => {
      const namespace = Class.Standard;

      initCalendar(namespace);

      expect((globalThis as any).Cal.ns[namespace]).toBeDefined();
    });

    it("should work with different namespace types", () => {
      initCalendar(Class.Standard);
      expect((globalThis as any).Cal.ns[Class.Standard]).toBeDefined();

      initCalendar(Class.Conversacion);
      expect((globalThis as any).Cal.ns[Class.Conversacion]).toBeDefined();

      initCalendar(Class.Grupales);
      expect((globalThis as any).Cal.ns[Class.Grupales]).toBeDefined();

      initCalendar(Class.Free);
      expect((globalThis as any).Cal.ns[Class.Free]).toBeDefined();
    });

    it("should not load script multiple times", () => {
      // First initialization
      initCalendar(Class.Standard);
      const scriptCount1 = document.head.querySelectorAll('script[src="https://app.cal.com/embed/embed.js"]').length;

      // Second initialization
      initCalendar(Class.Conversacion);
      const scriptCount2 = document.head.querySelectorAll('script[src="https://app.cal.com/embed/embed.js"]').length;

      // Script should only be loaded once
      expect(scriptCount1).toBeGreaterThan(0);
      expect(scriptCount2).toBe(scriptCount1);
    });

    it("should initialize Cal with command queue", () => {
      initCalendar(Class.Standard);

      expect((globalThis as any).Cal.q).toBeDefined();
      expect(Array.isArray((globalThis as any).Cal.q)).toBe(true);
    });

    it("should create namespace with its own queue", () => {
      const namespace = Class.Standard;

      initCalendar(namespace);

      const namespaceInstance = (globalThis as any).Cal.ns[namespace];
      expect(namespaceInstance).toBeDefined();
      expect(namespaceInstance.q).toBeDefined();
    });
  });

  //////////////////// getPrice ////////////////////
  describe("getPrice", () => {
    const mockPricingData: PricingApiResponse = {
      country: "ES",
      currency: "EUR",
      symbol: "€",
      level: "high" as const,
      countryGroup: "EU",
      isDevelopment: false,
      prices: {
        individual_standard: 25,
        individual_conversation: 30,
        group: 15,
      },
    };

    it("should return standard price for standard-class tier", () => {
      const price = getPrice("standard-class", mockPricingData);

      expect(price).toBe(25);
    });

    it("should return conversation price for conversation-class tier", () => {
      const price = getPrice("conversation-class", mockPricingData);

      expect(price).toBe(30);
    });

    it("should return group price for group-class tier", () => {
      const price = getPrice("group-class", mockPricingData);

      expect(price).toBe(15);
    });

    it("should return standard price for unknown tier", () => {
      const price = getPrice("unknown-tier", mockPricingData);

      expect(price).toBe(25);
    });

    it("should return standard price for empty tier", () => {
      const price = getPrice("", mockPricingData);

      expect(price).toBe(25);
    });

    it("should handle different pricing values", () => {
      const customPricingData: PricingApiResponse = {
        country: "US",
        currency: "USD",
        symbol: "$",
        level: "high" as const,
        countryGroup: "NA",
        isDevelopment: false,
        prices: {
          individual_standard: 50,
          individual_conversation: 60,
          group: 35,
        },
      };

      expect(getPrice("standard-class", customPricingData)).toBe(50);
      expect(getPrice("conversation-class", customPricingData)).toBe(60);
      expect(getPrice("group-class", customPricingData)).toBe(35);
    });

    it("should handle zero prices", () => {
      const zeroPricingData: PricingApiResponse = {
        country: "TEST",
        currency: "EUR",
        symbol: "€",
        level: "low" as const,
        countryGroup: "TEST",
        isDevelopment: true,
        prices: {
          individual_standard: 0,
          individual_conversation: 0,
          group: 0,
        },
      };

      expect(getPrice("standard-class", zeroPricingData)).toBe(0);
      expect(getPrice("conversation-class", zeroPricingData)).toBe(0);
      expect(getPrice("group-class", zeroPricingData)).toBe(0);
    });
  });

  //////////////////// updatePricing ////////////////////
  describe("updatePricing", () => {
    const mockPricingData: PricingApiResponse = {
      country: "ES",
      currency: "EUR",
      symbol: "€",
      level: "high" as const,
      countryGroup: "EU",
      isDevelopment: false,
      prices: {
        individual_standard: 25,
        individual_conversation: 30,
        group: 15,
      },
    };

    beforeEach(() => {
      vi.mocked(getPricingByCountry).mockResolvedValue(mockPricingData);
    });

    it("should update pricing for standard-class card", async () => {
      const card = document.createElement("div");
      card.setAttribute("card-pricing-tier", "standard-class");

      const symbolElement = document.createElement("span");
      symbolElement.className = "currency-symbol";
      card.appendChild(symbolElement);

      const amountElement = document.createElement("span");
      amountElement.className = "price-amount";
      card.appendChild(amountElement);

      document.body.appendChild(card);

      await updatePricing();

      expect(symbolElement.textContent).toBe("€");
      expect(amountElement.textContent).toBe("25");
    });

    it("should update pricing for conversation-class card", async () => {
      const card = document.createElement("div");
      card.setAttribute("card-pricing-tier", "conversation-class");

      const symbolElement = document.createElement("span");
      symbolElement.className = "currency-symbol";
      card.appendChild(symbolElement);

      const amountElement = document.createElement("span");
      amountElement.className = "price-amount";
      card.appendChild(amountElement);

      document.body.appendChild(card);

      await updatePricing();

      expect(symbolElement.textContent).toBe("€");
      expect(amountElement.textContent).toBe("30");
    });

    it("should update pricing for group-class card", async () => {
      const card = document.createElement("div");
      card.setAttribute("card-pricing-tier", "group-class");

      const symbolElement = document.createElement("span");
      symbolElement.className = "currency-symbol";
      card.appendChild(symbolElement);

      const amountElement = document.createElement("span");
      amountElement.className = "price-amount";
      card.appendChild(amountElement);

      document.body.appendChild(card);

      await updatePricing();

      expect(symbolElement.textContent).toBe("€");
      expect(amountElement.textContent).toBe("15");
    });

    it("should update multiple cards", async () => {
      // Create multiple cards
      const cards = [
        { tier: "standard-class", expectedPrice: "25" },
        { tier: "conversation-class", expectedPrice: "30" },
        { tier: "group-class", expectedPrice: "15" },
      ];

      cards.forEach(({ tier }) => {
        const card = document.createElement("div");
        card.setAttribute("card-pricing-tier", tier);

        const symbolElement = document.createElement("span");
        symbolElement.className = "currency-symbol";
        card.appendChild(symbolElement);

        const amountElement = document.createElement("span");
        amountElement.className = "price-amount";
        card.appendChild(amountElement);

        document.body.appendChild(card);
      });

      await updatePricing();

      const allCards = document.querySelectorAll("[card-pricing-tier]");
      expect(allCards).toHaveLength(3);

      cards.forEach(({ tier, expectedPrice }, index) => {
        const card = allCards[index];
        const symbolElement = card.querySelector(".currency-symbol");
        const amountElement = card.querySelector(".price-amount");

        expect(symbolElement?.textContent).toBe("€");
        expect(amountElement?.textContent).toBe(expectedPrice);
      });
    });

    it("should update multiple currency symbols in a single card", async () => {
      const card = document.createElement("div");
      card.setAttribute("card-pricing-tier", "standard-class");

      const symbolElement1 = document.createElement("span");
      symbolElement1.className = "currency-symbol";
      card.appendChild(symbolElement1);

      const symbolElement2 = document.createElement("span");
      symbolElement2.className = "currency-symbol";
      card.appendChild(symbolElement2);

      const amountElement = document.createElement("span");
      amountElement.className = "price-amount";
      card.appendChild(amountElement);

      document.body.appendChild(card);

      await updatePricing();

      expect(symbolElement1.textContent).toBe("€");
      expect(symbolElement2.textContent).toBe("€");
      expect(amountElement.textContent).toBe("25");
    });

    it("should update multiple price amounts in a single card", async () => {
      const card = document.createElement("div");
      card.setAttribute("card-pricing-tier", "group-class");

      const symbolElement = document.createElement("span");
      symbolElement.className = "currency-symbol";
      card.appendChild(symbolElement);

      const amountElement1 = document.createElement("span");
      amountElement1.className = "price-amount";
      card.appendChild(amountElement1);

      const amountElement2 = document.createElement("span");
      amountElement2.className = "price-amount";
      card.appendChild(amountElement2);

      document.body.appendChild(card);

      await updatePricing();

      expect(symbolElement.textContent).toBe("€");
      expect(amountElement1.textContent).toBe("15");
      expect(amountElement2.textContent).toBe("15");
    });

    it("should handle card without tier attribute", async () => {
      const card = document.createElement("div");
      card.setAttribute("card-pricing-tier", "");

      const symbolElement = document.createElement("span");
      symbolElement.className = "currency-symbol";
      card.appendChild(symbolElement);

      const amountElement = document.createElement("span");
      amountElement.className = "price-amount";
      card.appendChild(amountElement);

      document.body.appendChild(card);

      await updatePricing();

      expect(consoleErrorSpy).toHaveBeenCalledWith("Dont find card-pracing-tier");
    });

    it("should handle card without null tier attribute", async () => {
      const card = document.createElement("div");
      // Don't set any tier attribute

      const symbolElement = document.createElement("span");
      symbolElement.className = "currency-symbol";
      card.appendChild(symbolElement);

      const amountElement = document.createElement("span");
      amountElement.className = "price-amount";
      card.appendChild(amountElement);

      document.body.appendChild(card);

      await updatePricing();

      // Should not throw error
      expect(consoleErrorSpy).not.toHaveBeenCalled();
    });

    it("should handle card without symbol element", async () => {
      const card = document.createElement("div");
      card.setAttribute("card-pricing-tier", "standard-class");

      const amountElement = document.createElement("span");
      amountElement.className = "price-amount";
      card.appendChild(amountElement);

      document.body.appendChild(card);

      await updatePricing();

      expect(amountElement.textContent).toBe("25");
      expect(consoleErrorSpy).not.toHaveBeenCalled();
    });

    it("should handle card without amount element", async () => {
      const card = document.createElement("div");
      card.setAttribute("card-pricing-tier", "standard-class");

      const symbolElement = document.createElement("span");
      symbolElement.className = "currency-symbol";
      card.appendChild(symbolElement);

      document.body.appendChild(card);

      await updatePricing();

      expect(symbolElement.textContent).toBe("€");
      expect(consoleErrorSpy).not.toHaveBeenCalled();
    });

    it("should handle card without symbol and amount elements", async () => {
      const card = document.createElement("div");
      card.setAttribute("card-pricing-tier", "standard-class");

      document.body.appendChild(card);

      await updatePricing();

      expect(consoleErrorSpy).not.toHaveBeenCalled();
    });

    it("should log error when getPricingByCountry fails", async () => {
      vi.mocked(getPricingByCountry).mockRejectedValue(new Error("Network error"));

      const card = document.createElement("div");
      card.setAttribute("card-pricing-tier", "standard-class");

      const symbolElement = document.createElement("span");
      symbolElement.className = "currency-symbol";
      card.appendChild(symbolElement);

      const amountElement = document.createElement("span");
      amountElement.className = "price-amount";
      card.appendChild(amountElement);

      document.body.appendChild(card);

      await updatePricing();

      expect(consoleErrorSpy).toHaveBeenCalledWith("Error loading pricing:", expect.any(Error));
    });

    it("should handle different currency symbols", async () => {
      const customPricingData: PricingApiResponse = {
        country: "US",
        currency: "USD",
        symbol: "$",
        level: "high" as const,
        countryGroup: "NA",
        isDevelopment: false,
        prices: {
          individual_standard: 50,
          individual_conversation: 60,
          group: 35,
        },
      };

      vi.mocked(getPricingByCountry).mockResolvedValue(customPricingData);

      const card = document.createElement("div");
      card.setAttribute("card-pricing-tier", "standard-class");

      const symbolElement = document.createElement("span");
      symbolElement.className = "currency-symbol";
      card.appendChild(symbolElement);

      const amountElement = document.createElement("span");
      amountElement.className = "price-amount";
      card.appendChild(amountElement);

      document.body.appendChild(card);

      await updatePricing();

      expect(symbolElement.textContent).toBe("$");
      expect(amountElement.textContent).toBe("50");
    });

    it("should handle unknown tier by using default price", async () => {
      const card = document.createElement("div");
      card.setAttribute("card-pricing-tier", "unknown-tier");

      const symbolElement = document.createElement("span");
      symbolElement.className = "currency-symbol";
      card.appendChild(symbolElement);

      const amountElement = document.createElement("span");
      amountElement.className = "price-amount";
      card.appendChild(amountElement);

      document.body.appendChild(card);

      await updatePricing();

      expect(symbolElement.textContent).toBe("€");
      expect(amountElement.textContent).toBe("25"); // Default to standard price
    });

    it("should call getPricingByCountry once", async () => {
      const card1 = document.createElement("div");
      card1.setAttribute("card-pricing-tier", "standard-class");

      const symbolElement1 = document.createElement("span");
      symbolElement1.className = "currency-symbol";
      card1.appendChild(symbolElement1);

      const amountElement1 = document.createElement("span");
      amountElement1.className = "price-amount";
      card1.appendChild(amountElement1);

      const card2 = document.createElement("div");
      card2.setAttribute("card-pricing-tier", "conversation-class");

      const symbolElement2 = document.createElement("span");
      symbolElement2.className = "currency-symbol";
      card2.appendChild(symbolElement2);

      const amountElement2 = document.createElement("span");
      amountElement2.className = "price-amount";
      card2.appendChild(amountElement2);

      document.body.appendChild(card1);
      document.body.appendChild(card2);

      await updatePricing();

      expect(getPricingByCountry).toHaveBeenCalledTimes(1);
    });

    it("should handle no cards present", async () => {
      await updatePricing();

      expect(getPricingByCountry).toHaveBeenCalled();
      expect(consoleErrorSpy).not.toHaveBeenCalled();
    });
  });
});
