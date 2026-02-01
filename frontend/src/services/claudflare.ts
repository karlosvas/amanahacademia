import { log } from "./logger";

// Servicio para manejar Cloudflare Turnstile
export function executeTurnstileIfPresent(formHTML: HTMLFormElement): Promise<void> | void {
  const turnstileDiv = formHTML.querySelector(".cf-turnstile");
  if (!globalThis.turnstile || !turnstileDiv) return;
  return new Promise((resolve, reject) => {
    try {
      (globalThis.turnstile as any).execute(turnstileDiv, {
        callback: () => resolve(),
        "error-callback": (error: unknown) => {
          log.error("Error de Turnstile:", error);
          reject(new Error("Error en la verificación, por favor recarga la página."));
        },
      });
    } catch (e) {
      reject(e instanceof Error ? e : new Error(String(e)));
    }
  });
}

// Actualiza la visibilidad de los widgets de Turnstile según el contexto (login o registro)
export function updateTurnstileVisibility(isRegister: boolean) {
  const loginWidget = document.getElementById("turnstile-login");
  const registerWidget = document.getElementById("turnstile-register");
  if (loginWidget) loginWidget.style.display = isRegister ? "none" : "";
  if (registerWidget) registerWidget.style.display = isRegister ? "" : "none";
}
