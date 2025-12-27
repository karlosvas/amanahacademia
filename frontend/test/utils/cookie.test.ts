import { describe, it, expect, beforeEach, vi, afterEach } from "vitest";
import {
  getThemeFromCookie,
  getLangFromCookie,
  acceptCookies,
  rejectCookies,
  initializeCookieConsent,
  writeLangCookie,
  applyTheme,
} from "@/utils/cookie.ts";
import { Languages, Theme } from "@/enums/enums";
import * as modals from "@/utils/modals";

// Mock hideBanner
vi.mock("@/utils/modals", () => ({
  hideBanner: vi.fn(),
}));

describe("Cookie Utilities", () => {
  let cookieStore: { [key: string]: string };
  let localStorageStore: { [key: string]: string };

  beforeEach(() => {
    // Mock document.cookie con getter/setter
    cookieStore = {};
    Object.defineProperty(document, "cookie", {
      get: vi.fn(() => {
        return Object.entries(cookieStore)
          .map(([key, value]) => `${key}=${value}`)
          .join("; ");
      }),
      set: vi.fn((cookieString: string) => {
        const [pair] = cookieString.split(";");
        const [key, value] = pair.split("=");
        if (value) {
          cookieStore[key.trim()] = value.trim();
        }
      }),
      configurable: true,
    });

    // Mock localStorage
    localStorageStore = {};
    Object.defineProperty(globalThis, "localStorage", {
      value: {
        getItem: vi.fn((key: string) => localStorageStore[key] || null),
        setItem: vi.fn((key: string, value: string) => {
          localStorageStore[key] = value;
        }),
        clear: vi.fn(() => {
          localStorageStore = {};
        }),
      },
      configurable: true,
    });

    // Reset mocks
    vi.clearAllMocks();

    // Reset gtag
    delete (globalThis as any).gtag;
    globalThis.dataLayer = [];

    // Reset location
    Object.defineProperty(globalThis, "location", {
      value: { hostname: "test.com", search: "", protocol: "https:" },
      writable: true,
      configurable: true,
    });
  });

  // Getters
  describe("getThemeFromCookie", () => {
    it('should return "light" when no theme cookie exists', () => {
      const theme = getThemeFromCookie();
      expect(theme).toBe("light");
    });

    it("should return theme value from cookie", () => {
      cookieStore["theme"] = "dark";
      const theme = getThemeFromCookie();
      expect(theme).toBe("dark");
    });

    it("should handle multiple cookies and extract theme", () => {
      cookieStore["lang"] = "es";
      cookieStore["theme"] = "dark";
      cookieStore["user"] = "test";
      const theme = getThemeFromCookie();
      expect(theme).toBe("dark");
    });
  });

  describe("getLangFromCookie", () => {
    it('should return "es" as default language', () => {
      const lang = getLangFromCookie();
      expect(lang).toBe("es");
    });

    it("should return language value from cookie", () => {
      cookieStore["langCookie"] = "en";
      const lang = getLangFromCookie();
      expect(lang).toBe("en");
    });

    it("should handle multiple cookies and extract language", () => {
      cookieStore["other"] = "value";
      cookieStore["langCookie"] = "fr";
      const lang = getLangFromCookie();
      expect(lang).toBe("fr");
    });

    it("should return default language when cookie value is invalid", () => {
      const consoleWarnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});
      cookieStore["langCookie"] = "invalid_lang";
      const lang = getLangFromCookie();
      expect(lang).toBe("es");
      // Check that warn was called with the right message (accounting for adze logger formatting)
      expect(consoleWarnSpy).toHaveBeenCalled();
      const warnCall = consoleWarnSpy.mock.calls[0];
      const fullMessage = warnCall.join(" ");
      expect(fullMessage).toContain('Valor inválido detectado: "invalid_lang"');
      consoleWarnSpy.mockRestore();
    });

    it("should handle XSS attempts in cookie value", () => {
      const consoleWarnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});
      cookieStore["langCookie"] = "<script>alert('xss')</script>";
      const lang = getLangFromCookie();
      expect(lang).toBe("es");
      expect(consoleWarnSpy).toHaveBeenCalled();
      consoleWarnSpy.mockRestore();
    });

    it("should handle path traversal attempts in cookie value", () => {
      const consoleWarnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});
      cookieStore["langCookie"] = "../../../etc/passwd";
      const lang = getLangFromCookie();
      expect(lang).toBe("es");
      expect(consoleWarnSpy).toHaveBeenCalled();
      consoleWarnSpy.mockRestore();
    });

    it("should reset cookie to valid value when invalid", () => {
      cookieStore["langCookie"] = "hacked";
      getLangFromCookie();
      expect(cookieStore["langCookie"]).toBe("es");
    });

    it("should accept all valid language codes", () => {
      const validLangs = ["en", "es", "fr", "de", "it", "pt", "ar"];
      validLangs.forEach((lang) => {
        cookieStore["langCookie"] = lang;
        const result = getLangFromCookie();
        expect(result).toBe(lang);
      });
    });
  });

  // Setters
  describe("writeLangCookie", () => {
    it("should write valid language cookie", () => {
      writeLangCookie("en" as Languages);
      expect(cookieStore["langCookie"]).toBe("en");
    });

    it("should reject invalid values with slashes", () => {
      const consoleWarnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});
      writeLangCookie("en/test" as Languages);
      expect(consoleWarnSpy).toHaveBeenCalled();
      expect(cookieStore["langCookie"]).toBeUndefined();
      consoleWarnSpy.mockRestore();
    });

    it("should reject invalid values with dots", () => {
      const consoleWarnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});
      writeLangCookie("en.test" as Languages);
      expect(consoleWarnSpy).toHaveBeenCalled();
      consoleWarnSpy.mockRestore();
    });
  });

  describe("applyTheme", () => {
    it("should write dark theme cookie", () => {
      applyTheme(Theme.DARK);
      expect(cookieStore["theme"]).toBe(Theme.DARK);
    });

    it("should write light theme", () => {
      applyTheme(Theme.LIGHT);
      expect(cookieStore["theme"]).toBe(Theme.LIGHT);
    });
  });

  // Cookies
  describe("acceptCookies", () => {
    beforeEach(() => {
      vi.useFakeTimers();
    });

    afterEach(() => {
      vi.useRealTimers();
    });

    it("should set cookieConsent to accepted in localStorage", () => {
      acceptCookies();
      expect(localStorageStore["cookieConsent"]).toBe("accepted");
    });

    it("should update gtag consent when gtag is available", () => {
      const mockGtag = vi.fn();
      (globalThis as any).gtag = mockGtag;

      acceptCookies();

      expect(mockGtag).toHaveBeenCalledWith("consent", "update", {
        ad_storage: "granted",
        ad_user_data: "granted",
        ad_personalization: "granted",
        analytics_storage: "granted",
      });

      expect(mockGtag).toHaveBeenCalledWith("event", "cookie_consent_granted", {
        consent_type: "accept",
      });

      expect(mockGtag).toHaveBeenCalledWith("event", "page_view");
    });

    it("should call hideBanner", () => {
      acceptCookies();
      expect(modals.hideBanner).toHaveBeenCalled();
    });
  });

  describe("rejectCookies", () => {
    beforeEach(() => {
      vi.useFakeTimers();
    });

    afterEach(() => {
      vi.useRealTimers();
    });

    it("should set cookieConsent to rejected in localStorage", () => {
      rejectCookies();
      expect(localStorageStore["cookieConsent"]).toBe("rejected");
    });

    it("should call hideBanner", () => {
      rejectCookies();
      expect(modals.hideBanner).toHaveBeenCalled();
    });
  });

  describe("initializeCookieConsent", () => {
    beforeEach(() => {
      vi.useFakeTimers();
    });

    afterEach(() => {
      vi.useRealTimers();
    });

    it("should show banner after 5 seconds when no consent exists", () => {
      const mockBanner = {
        classList: { remove: vi.fn() },
      };
      document.getElementById = vi.fn(() => mockBanner as any);

      initializeCookieConsent();

      vi.advanceTimersByTime(5000);
      expect(mockBanner.classList.remove).toHaveBeenCalledWith("hidden");
    });

    it("should not show banner when consent is accepted", () => {
      localStorageStore["cookieConsent"] = "accepted";
      const mockBanner = {
        classList: { remove: vi.fn() },
      };
      document.getElementById = vi.fn(() => mockBanner as any);

      initializeCookieConsent();

      vi.advanceTimersByTime(5000);
      expect(mockBanner.classList.remove).not.toHaveBeenCalled();
    });

    it("should update gtag consent when previously accepted", () => {
      localStorageStore["cookieConsent"] = "accepted";
      const mockGtag = vi.fn();
      (globalThis as any).gtag = mockGtag;

      initializeCookieConsent();

      expect(mockGtag).toHaveBeenCalledWith("consent", "update", {
        ad_storage: "granted",
        ad_user_data: "granted",
        ad_personalization: "granted",
        analytics_storage: "granted",
      });
    });
  });
});
