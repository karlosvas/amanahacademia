import { Languages, Theme } from "@/enums/enums";
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

export function getLangFromCookie(): Languages {
  const match = document.cookie.match(/(?:^|; )langCookie=([^;]*)/);

  if (!match) return Languages.SPANISH;

  const cookieValue = match[1];

  // Validar que el valor sea un idioma válido del enum Languages
  if (Object.values(Languages).includes(cookieValue as Languages)) return cookieValue as Languages;

  // Si el valor es inválido, registrar advertencia y devolver español por defecto
  console.warn(`[Lang Cookie] Valor inválido detectado: "${cookieValue}". Usando idioma por defecto.`);

  // Opcionalmente, limpiar la cookie corrupta
  writeLangCookie(Languages.SPANISH);

  return Languages.SPANISH;
}

/// Theme
function writeThemeCookie(value: Theme) {
  // Validar que el valor sea un tema válido
  if (!value || (value !== Theme.DARK && value !== Theme.LIGHT)) {
    console.warn(`[Theme Cookie] Intento de escribir valor inválido: ${value}`);
    return;
  }

  const maxAge = 60 * 60 * 24 * 365; // 1 año
  document.cookie = `theme=${value}; path=/; max-age=${maxAge}; SameSite=Lax${location.protocol === "https:" ? "; Secure" : ""}`;
}

export function getThemeFromCookie(): Theme {
  // Si no hay acceso al documento (SSR), devolver tema por defecto
  if (typeof document === "undefined") {
    return Theme.LIGHT;
  }

  // Buscar la cookie de tema
  const match = document.cookie.match(/(?:^|; )theme=([^;]*)/);

  if (!match) {
    // No existe cookie, usar preferencia del sistema
    const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    const systemTheme = prefersDark ? Theme.DARK : Theme.LIGHT;
    writeThemeCookie(systemTheme);
    return systemTheme;
  }

  const cookieValue = match[1];

  // Validar que el valor sea un tema válido del enum Theme
  if (cookieValue === Theme.DARK || cookieValue === Theme.LIGHT) {
    return cookieValue as Theme;
  }

  // Si el valor es inválido, registrar advertencia y usar tema por defecto
  console.warn(`[Theme Cookie] Valor inválido detectado: "${cookieValue}". Usando tema por defecto.`);

  // Limpiar la cookie corrupta y establecer tema por defecto
  const defaultTheme = Theme.LIGHT;
  writeThemeCookie(defaultTheme);
  return defaultTheme;
}

export function applyTheme(newTheme: Theme) {
  const html = document.documentElement;

  // Validación estricta del tema
  if (newTheme !== Theme.DARK && newTheme !== Theme.LIGHT) {
    console.warn(`[Apply Theme] Tema inválido: ${newTheme}`);
    return;
  }

  // Remover clases de tema anteriores y agregar la nueva
  html.classList.remove(Theme.DARK, Theme.LIGHT);
  html.classList.add(newTheme);

  // Actualizar la cookie
  writeThemeCookie(newTheme);

  // Actualizar logo si existe
  const logo = document.getElementById("logo_amanah");
  if (logo && logo instanceof HTMLImageElement) {
    logo.src = newTheme === Theme.DARK ? "/img/logo_amanah_dark.webp" : "/img/logo_amanah.webp";
  }
}
