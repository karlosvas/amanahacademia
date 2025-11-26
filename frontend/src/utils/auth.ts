import { getCurrentUserToken } from "@/services/firebase";

// Sincronización del token de autenticación del usuario actual con el servidor
export async function syncTokenWithServer(): Promise<void> {
  const token = await getCurrentUserToken();
  if (!token) return;

  await fetch("/api/auth", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ token }),
  });
}

export async function syncLogoutWithServer(): Promise<void> {
  await fetch("/api/auth", { method: "DELETE" });
}
