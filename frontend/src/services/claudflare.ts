// Servicio para manejar Cloudflare Turnstile
export function executeTurnstileIfPresent(formHTML: HTMLFormElement): Promise<void> | void {
  const turnstileDiv = formHTML.querySelector(".cf-turnstile");
  const t = (globalThis as any)?.turnstile;
  if (!t || !turnstileDiv) return;
  return new Promise((resolve, reject) => {
    try {
      t.execute(turnstileDiv, {
        callback: () => resolve(),
        "error-callback": (error: unknown) => {
          console.error("Error de Turnstile:", error);
          reject(new Error("Error en la verificación, por favor recarga la página."));
        },
      });
    } catch (e) {
      reject(e instanceof Error ? e : new Error(String(e)));
    }
  });
}

export function updateTurnstileVisibility(isRegister: boolean) {
  const loginWidget = document.getElementById("turnstile-login");
  const registerWidget = document.getElementById("turnstile-register");
  if (loginWidget) loginWidget.style.display = isRegister ? "none" : "";
  if (registerWidget) registerWidget.style.display = isRegister ? "" : "none";
}
