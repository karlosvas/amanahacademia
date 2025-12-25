import { getCurrentUserToken } from "@/services/firebase";
import type { PricingApiResponse } from "@/types/types";

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

// Sincronización del cierre de sesión con el servidor
export async function syncLogoutWithServer(): Promise<void> {
  await fetch("/api/auth", { method: "DELETE" });
}

// Obtener los datos de precios según el país
export async function getPricingByCountry(): Promise<PricingApiResponse> {
  // Obtener el país de la URL si existe
  const urlParams: URLSearchParams = new URLSearchParams(window.location.search);
  const testCountry: string | null = urlParams.get("test_country");
  const apiUrl: string = testCountry ? `/api/pricing?test_country=${testCountry}` : "/api/pricing";
  const response: Response = await fetch(apiUrl);

  if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);

  const pricingData: PricingApiResponse = (await response.json()) as PricingApiResponse;

  return pricingData;
}
