import { Languages } from "@/enums/enums";
import { hideBanner } from "./modals";

/// Cookies consent banner
export function acceptCookies() {
  const consentUpdate = {
    ad_storage: "granted",
    ad_user_data: "granted",
    ad_personalization: "granted",
    analytics_storage: "granted",
  };

  // Debug habilitado en desarrollo O si hay parámetro ?ga_debug=1
  const isDebug =
    window.location.hostname === "localhost" ||
    window.location.hostname === "127.0.0.1" ||
    new URLSearchParams(window.location.search).get("ga_debug") === "1";

  // Función que actualiza el consentimiento
  const updateConsent = () => {
    if (typeof window.gtag !== "undefined") {
      window.gtag("consent", "update", consentUpdate);

      //  Enviar un evento después de actualizar el consentimiento
      // para que GA4 registre el hit inmediatamente
      window.gtag("event", "cookie_consent_granted", {
        consent_type: "accept",
      });

      // Enviar page_view explícito para que la extensión lo detecte
      window.gtag("event", "page_view");

      if (isDebug) {
        console.log("[GA Debug] Consent updated (ACCEPTED):", consentUpdate);
        console.log("[GA Debug] Event sent: cookie_consent_granted");
        console.log("[GA Debug] Event sent: page_view");
        console.log("[GA Debug] DataLayer:", window.dataLayer);
      }
    } else if (isDebug) {
      console.log("[GA Debug] gtag not available when accepting cookies");
    }
  };

  // Esperar a que gtag esté disponible
  if (typeof window.gtag !== "undefined") {
    updateConsent();
  } else {
    if (isDebug) {
      console.log("[GA Debug] Waiting for gtag to be available...");
    }

    // Esperar hasta 5 segundos a que gtag esté disponible
    const checkGtag = setInterval(() => {
      if (typeof window.gtag !== "undefined") {
        clearInterval(checkGtag);
        updateConsent();
      }
    }, 100);

    setTimeout(() => clearInterval(checkGtag), 5000);
  }

  localStorage.setItem("cookieConsent", "accepted");
  hideBanner();
}

export function rejectCookies() {
  localStorage.setItem("cookieConsent", "rejected");

  // Debug log (solo en desarrollo)
  if (window.location.hostname === "localhost" || window.location.hostname === "127.0.0.1") {
    console.log("[GA Debug] Cookies REJECTED - Analytics will remain disabled");
  }

  hideBanner();
}

export function initializeCookieConsent() {
  const consent = localStorage.getItem("cookieConsent");
  // Debug habilitado en desarrollo O si hay parámetro ?ga_debug=1
  const isDebug =
    window.location.hostname === "localhost" ||
    window.location.hostname === "127.0.0.1" ||
    new URLSearchParams(window.location.search).get("ga_debug") === "1";

  // Función que actualiza el consentimiento cuando gtag esté listo
  const updateConsent = () => {
    if (consent === "accepted") {
      const consentUpdate = {
        ad_storage: "granted",
        ad_user_data: "granted",
        ad_personalization: "granted",
        analytics_storage: "granted",
      };

      if (typeof window.gtag !== "undefined") {
        window.gtag("consent", "update", consentUpdate);

        // CRÍTICO: Enviar un evento para que GA4 registre el hit
        window.gtag("event", "cookie_consent_restored", {
          consent_type: "previously_accepted",
        });

        // Enviar page_view explícito para que la extensión lo detecte
        window.gtag("event", "page_view");

        if (isDebug) {
          console.log("[GA Debug] Initializing with ACCEPTED consent:", consentUpdate);
          console.log("[GA Debug] Event sent: cookie_consent_restored");
          console.log("[GA Debug] Event sent: page_view");
          console.log("[GA Debug] DataLayer after init:", window.dataLayer);
        }
      } else if (isDebug) {
        console.log("[GA Debug] gtag not ready yet, waiting...");
      }
    } else if (consent === "rejected") {
      if (isDebug) {
        console.log("[GA Debug] User previously REJECTED cookies - Analytics disabled");
      }
    } else {
      if (isDebug) {
        console.log("[GA Debug] No consent decision found - Banner will show in 5s");
      }
    }
  };

  // Esperar a que gtag esté disponible
  if (typeof window.gtag !== "undefined") {
    updateConsent();
  } else {
    // Si gtag no está listo, esperar a que se cargue
    const checkGtag = setInterval(() => {
      if (typeof window.gtag !== "undefined") {
        clearInterval(checkGtag);
        updateConsent();
      }
    }, 100);

    // Timeout de seguridad (máximo 5 segundos)
    setTimeout(() => clearInterval(checkGtag), 5000);
  }

  // Mostrar banner si no hay decisión (después de 5 segundos)
  if (!consent) {
    setTimeout(() => {
      const banner = document.getElementById("cookie-banner");
      if (banner) banner.classList.remove("hidden");
    }, 5000);
  }
}

/// Language
export function writeLangCookie(value: Languages) {
  // Validar que el valor sea un idioma válido
  if (!value || value.includes("/") || value.includes(".")) {
    console.warn(`[Lang Cookie] Intento de escribir valor inválido: ${value}`);
    return;
  }

  const maxAge = 60 * 60 * 24 * 365;
  document.cookie = `langCookie=${value}; path=/; max-age=${maxAge}; SameSite=Lax${location.protocol === "https:" ? "; Secure" : ""}`;
}

export function getLangFromCookie(): string {
  const match = document.cookie.match(/(?:^|; )langCookie=([^;]*)/);
  return match ? match[1] : "es";
}

/// Theme
export function writeThemeCookie(value: string) {
  const maxAge = 60 * 60 * 24 * 365; // 1 año
  document.cookie = `theme=${value}; path=/; max-age=${maxAge}; SameSite=Lax${location.protocol === "https:" ? "; Secure" : ""}`;
}

export function getThemeFromCookie() {
  if (typeof document === "undefined" || !document.cookie) return "light";
  const cookies = document.cookie.split(";").map((c) => c.trim());
  for (const c of cookies) if (c.startsWith("theme=")) return c.substring("theme=".length);
  return "light";
}
