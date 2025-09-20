import type { Class } from "@/enums/enums";
import { ApiService } from "./helper";
import type { Teacher } from "@/types/bakend-types";

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
          const api = function () {
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
      light: { "cal-brand": "#ffbba8" },
      dark: { "cal-brand": "#8a4141" },
    },
    hideEventTypeDetails: false,
    layout: "month_view",
  });
}

export async function getTeacherURL(teacher: string): Promise<Teacher> {
  try {
    const helper = new ApiService();
    const response = await helper.getTeacher(teacher);
    if (response.success && response.data) return response.data;
  } catch (error) {
    console.error("Error fetching teacher data:", error);
    throw error;
  }
  throw new Error("Teacher data not found.");
}
