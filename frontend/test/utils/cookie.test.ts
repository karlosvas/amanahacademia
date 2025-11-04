import { describe, it, expect, beforeEach, vi } from 'vitest';
import {
  getThemeFromCookie,
  getLangFromCookie,
  acceptCookies,
  rejectCookies,
  initializeCookieConsent,
} from '@/utils/cookie';

describe('Cookie Utilities', () => {
  beforeEach(() => {
    // Reset document.cookie
    document.cookie = '';
    // Reset localStorage
    localStorage.clear();
    // Reset mocks
    vi.clearAllMocks();
    // Reset gtag
    delete (window as any).gtag;
    window.dataLayer = [];
  });

  describe('getThemeFromCookie', () => {
    it('should return "light" when no theme cookie exists', () => {
      const theme = getThemeFromCookie();
      expect(theme).toBe('light');
    });

    it('should return theme value from cookie', () => {
      document.cookie = 'theme=dark';
      const theme = getThemeFromCookie();
      expect(theme).toBe('dark');
    });

    it('should handle multiple cookies and extract theme', () => {
      document.cookie = 'lang=es; theme=dark; user=test';
      const theme = getThemeFromCookie();
      expect(theme).toBe('dark');
    });
  });

  describe('getLangFromCookie', () => {
    it('should return "es" as default language', () => {
      const lang = getLangFromCookie();
      expect(lang).toBe('es');
    });

    it('should return language value from cookie', () => {
      document.cookie = 'lang=en';
      const lang = getLangFromCookie();
      expect(lang).toBe('en');
    });

    it('should handle multiple cookies and extract language', () => {
      // Need to clear previous cookie first
      document.cookie = 'lang=en; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;';
      document.cookie = 'lang=fr';
      const lang = getLangFromCookie();
      expect(lang).toBe('fr');
    });
  });

  describe('acceptCookies', () => {
    beforeEach(() => {
      vi.useFakeTimers();
      // Mock getElementById for banner
      document.getElementById = vi.fn((id: string) => {
        if (id === 'cookie-banner') {
          return {
            classList: { add: vi.fn() },
            style: { animation: '' },
          } as any;
        }
        return null;
      });
    });

    afterEach(() => {
      vi.useRealTimers();
    });

    it('should set cookieConsent to accepted in localStorage', () => {
      acceptCookies();
      expect(localStorage.setItem).toHaveBeenCalledWith('cookieConsent', 'accepted');
    });

    it('should update gtag consent when gtag is available', () => {
      const mockGtag = vi.fn();
      (window as any).gtag = mockGtag;

      acceptCookies();

      expect(mockGtag).toHaveBeenCalledWith('consent', 'update', {
        ad_storage: 'granted',
        ad_user_data: 'granted',
        ad_personalization: 'granted',
        analytics_storage: 'granted',
      });

      expect(mockGtag).toHaveBeenCalledWith('event', 'cookie_consent_granted', {
        consent_type: 'accept',
      });

      expect(mockGtag).toHaveBeenCalledWith('event', 'page_view');
    });

    it('should hide cookie banner', () => {
      const mockBanner = {
        classList: { add: vi.fn() },
        style: { animation: '' },
      };
      document.getElementById = vi.fn(() => mockBanner as any);

      acceptCookies();

      // Wait for timeout
      vi.runAllTimers();
      expect(mockBanner.classList.add).toHaveBeenCalledWith('hidden');
    });
  });

  describe('rejectCookies', () => {
    beforeEach(() => {
      vi.useFakeTimers();
      document.getElementById = vi.fn((id: string) => {
        if (id === 'cookie-banner') {
          return {
            classList: { add: vi.fn() },
            style: { animation: '' },
          } as any;
        }
        return null;
      });
    });

    afterEach(() => {
      vi.useRealTimers();
    });

    it('should set cookieConsent to rejected in localStorage', () => {
      rejectCookies();
      expect(localStorage.setItem).toHaveBeenCalledWith('cookieConsent', 'rejected');
    });

    it('should hide cookie banner', () => {
      const mockBanner = {
        classList: { add: vi.fn() },
        style: { animation: '' },
      };
      document.getElementById = vi.fn(() => mockBanner as any);

      rejectCookies();

      vi.runAllTimers();
      expect(mockBanner.classList.add).toHaveBeenCalledWith('hidden');
    });
  });

  describe('initializeCookieConsent', () => {
    beforeEach(() => {
      vi.useFakeTimers();
    });

    afterEach(() => {
      vi.useRealTimers();
    });

    it('should show banner after 5 seconds when no consent exists', () => {
      const mockBanner = {
        classList: { remove: vi.fn() },
      };
      document.getElementById = vi.fn(() => mockBanner as any);

      initializeCookieConsent();

      vi.advanceTimersByTime(5000);
      expect(mockBanner.classList.remove).toHaveBeenCalledWith('hidden');
    });

    it('should not show banner when consent is accepted', () => {
      localStorage.getItem = vi.fn(() => 'accepted');
      const mockBanner = {
        classList: { remove: vi.fn() },
      };
      document.getElementById = vi.fn(() => mockBanner as any);

      initializeCookieConsent();

      vi.advanceTimersByTime(5000);
      expect(mockBanner.classList.remove).not.toHaveBeenCalled();
    });

    it('should update gtag consent when previously accepted', () => {
      localStorage.getItem = vi.fn(() => 'accepted');
      const mockGtag = vi.fn();
      (window as any).gtag = mockGtag;

      initializeCookieConsent();

      expect(mockGtag).toHaveBeenCalledWith('consent', 'update', {
        ad_storage: 'granted',
        ad_user_data: 'granted',
        ad_personalization: 'granted',
        analytics_storage: 'granted',
      });
    });
  });
});
