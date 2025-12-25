import type { Class } from "@/enums/enums";
import type { PricingApiResponse } from "@/types/types";
import { getPricingByCountry } from "@/utils/auth";

/**
 * Inicializa y configura el calendario embebido de Cal.com para el namespace dado.
 * @param namespaceId - Identificador único del calendario (namespace)
 * @returns true si se inicializa correctamente, false si requiere autenticación
 */
export function initCalendar(namespaceId: Class): boolean {
  const CAL_EMBED_SCRIPT_URL = "https://app.cal.com/embed/embed.js";
  const CAL_INIT_COMMAND = "init";
  const CAL_ORIGIN = "https://app.cal.com";

  // Embebe el script de Cal.com si no está cargado
  (function (globalWindow, embedScriptUrl, initCommandName) {
    const documentReference = globalWindow.document;

    // Función auxiliar para encolar comandos en la API de Cal
    const enqueueCommand = function (calendarApi: any, commandArguments: any) {
      calendarApi.q.push(commandArguments);
    };

    // Inicializa el objeto Cal en el contexto global si no existe
    globalWindow.Cal =
      globalWindow.Cal ||
      function () {
        const calInstance = globalWindow.Cal;
        const functionArguments = arguments;

        // Primera carga: inicializa estructuras y carga el script
        if (!calInstance.loaded) {
          calInstance.ns = {}; // Namespaces para múltiples calendarios
          calInstance.q = calInstance.q || []; // Cola de comandos

          // Carga dinámica del script de Cal.com
          const scriptElement = documentReference.createElement("script");
          scriptElement.src = embedScriptUrl;
          documentReference.head.appendChild(scriptElement);

          calInstance.loaded = true;
        }

        // Manejo del comando de inicialización
        if (functionArguments[0] === initCommandName) {
          // Crea una API proxy para el namespace
          const namespaceApi: any = function () {
            enqueueCommand(namespaceApi, arguments);
          };

          const calendarNamespace = functionArguments[1];
          namespaceApi.q = namespaceApi.q || [];

          // Si es un namespace válido, lo registra
          if (typeof calendarNamespace === "string") {
            calInstance.ns[calendarNamespace] = calInstance.ns[calendarNamespace] || namespaceApi;
            enqueueCommand(calInstance.ns[calendarNamespace], functionArguments);
            enqueueCommand(calInstance, ["initNamespace", calendarNamespace]);
          } else {
            enqueueCommand(calInstance, functionArguments);
          }
          return;
        }

        // Encola cualquier otro comando
        enqueueCommand(calInstance, functionArguments);
      };
  })(window, CAL_EMBED_SCRIPT_URL, CAL_INIT_COMMAND);

  // Inicializa el calendario para el namespace específico
  window.Cal(CAL_INIT_COMMAND, `${namespaceId}`, { origin: CAL_ORIGIN });

  // Configura la UI del calendario embebido con temas personalizados
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

  return true;
}

// Actualizar en el frontend los precios
export async function updatePricing() {
  try {
    const pricingData: PricingApiResponse = await getPricingByCountry();

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
