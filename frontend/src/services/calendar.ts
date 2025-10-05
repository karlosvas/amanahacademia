import type { Class } from "@/enums/enums";
import type { PricingApiResponse } from "@/types/types";

/**
 * Inicializa y configura el calendario embebido de Cal.com para el namespace dado.
 * @param namespaceId - Identificador único del calendario (namespace)
 */
export function initCalendar(namespaceId: Class) {
  // Embebe el script de Cal.com si no está cargado
  (function (windowObj, scriptUrl, initKey) {
    // Función para encolar comandos
    let enqueue = function (calObj: any, args: any) {
      calObj.q.push(args);
    };
    let documentObj = windowObj.document;
    windowObj.Cal =
      windowObj.Cal ||
      function () {
        let cal = windowObj.Cal;
        let args = arguments;
        if (!cal.loaded) {
          cal.ns = {};
          cal.q = cal.q || [];
          documentObj.head.appendChild(documentObj.createElement("script")).src = scriptUrl;
          cal.loaded = true;
        }
        if (args[0] === initKey) {
          const api: any = function () {
            enqueue(api, arguments);
          };
          const namespace = args[1];
          api.q = api.q || [];
          if (typeof namespace === "string") {
            cal.ns[namespace] = cal.ns[namespace] || api;
            enqueue(cal.ns[namespace], args);
            enqueue(cal, ["initNamespace", namespace]);
          } else enqueue(cal, args);
          return;
        }
        enqueue(cal, args);
      };
  })(window, "https://app.cal.com/embed/embed.js", "init");

  // Inicializa el calendario para el namespace específico
  window.Cal("init", `${namespaceId}`, { origin: "https://app.cal.com" });

  // Configura la UI del calendario embebido
  window.Cal.ns[`${namespaceId}`]("ui", {
    cssVarsPerTheme: {
      light: {
        "cal-brand": "#fa8072", // Salmón vibrante para marca
        "cal-bg": "#fff4e1", // Blanco cálido de fondo
        "cal-bg-muted": "#ffbba8", // Salmón claro para fondos atenuados
        "cal-text": "#2d1b1b", // Marrón oscuro para texto principal
        "cal-text-emphasis": "#6d0006", // Rojo profundo para énfasis
        "cal-text-subtle": "#8a4141", // Marrón medio para texto secundario
        "cal-border": "#d89c8d", // Marrón claro para bordes
        "cal-bg-success": "#34d399", // Verde éxito
        "cal-bg-error": "#f87171", // Rojo error
      },
      dark: {
        "cal-brand": "#ff8c7a", // Salmón brillante para marca
        "cal-bg": "#2d1b1b", // Marrón muy oscuro de fondo
        "cal-bg-muted": "#3a2b2b", // Marrón oscuro suave para fondos atenuados
        "cal-text": "#f3f3f3", // Blanco cálido para texto principal
        "cal-text-emphasis": "#ffb5a7", // Salmón claro para énfasis
        "cal-text-subtle": "#a6786d", // Marrón cálido para texto secundario
        "cal-border": "#5f4949", // Marrón profundo para bordes
        "cal-bg-success": "#10b981", // Verde éxito más oscuro
        "cal-bg-error": "#ef4444", // Rojo error más intenso
      },
    },
  });
}

// Actualizar en el frontend los precios
export async function updatePricing() {
  try {
    // Obtener el país de la URL si existe
    const urlParams: URLSearchParams = new URLSearchParams(window.location.search);
    const testCountry: string | null = urlParams.get("test_country");
    const apiUrl: string = testCountry ? `/api/pricing?test_country=${testCountry}` : "/api/pricing";
    const response: Response = await fetch(apiUrl);

    if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);

    const pricingData: PricingApiResponse = (await response.json()) as PricingApiResponse;

    document.querySelectorAll("[card-pricing-tier]").forEach((card) => {
      const tier: string | null = card.getAttribute("card-pricing-tier");
      const symbolElement: NodeListOf<HTMLElement> = card.querySelectorAll(".currency-symbol");
      const amountElement: NodeListOf<HTMLElement> = card.querySelectorAll(".price-amount");

      if (!tier) {
        console.error("Dont find card-pracing-tier");
        return;
      }

      let tierPrice: number = getPrice(tier, pricingData);

      if (symbolElement && symbolElement.length > 0)
        symbolElement.forEach((el) => (el.textContent = pricingData.symbol));
      if (amountElement) amountElement.forEach((el) => (el.textContent = tierPrice.toString()));
    });
  } catch (error) {
    console.error("Error loading pricing:", error);
  }
}

export function getPrice(tier: string, pricingData: PricingApiResponse): number {
  switch (tier) {
    case "standard-class":
      return pricingData.prices.individual_standard;
    case "conversation-class":
      return pricingData.prices.individual_conversation;
    case "group-class":
      return pricingData.prices.group;
    default:
      return pricingData.prices.individual_standard;
  }
}
