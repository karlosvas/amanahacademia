import type { PricingApiResponse } from "@/types/types";

export async function GET({ request }: { request: Request }) {
  // Para testing en desarrollo y producción
  const url = new URL(request.url);
  // En desarrollo usa el parámetro, en producción usa CF header
  const country =
    url.searchParams.get("test_country") || // 1. Para pruebas
    request.headers.get("CF-IPCountry") || // 2. Encabezado común (Probablemente funciona)
    request.headers.get("x-vercel-ip-country") || // 3. Encabezado OFICIAL de Vercel (Máxima fiabilidad)
    "ES"; // 4. Valor por defecto

  // También puedes detectar si estás en desarrollo
  const isDevelopment =
    url.hostname === "localhost" ||
    url.hostname === "127.0.0.1" ||
    url.hostname.includes("local");

  // Países con mayor nivel de vida
  const highIncomeCountries = [
    // Europa
    "AT", // Austria
    "BE", // Bélgica
    "BG", // Bulgaria
    "HR", // Croacia
    "CY", // Chipre
    "CZ", // Chequia
    "DK", // Dinamarca
    "EE", // Estonia
    "FI", // Finlandia
    "FR", // Francia
    "DE", // Alemania
    "GR", // Grecia
    "HU", // Hungría
    "IE", // Irlanda
    "IT", // Italia
    "LV", // Letonia
    "LT", // Lituania
    "LU", // Luxemburgo
    "MT", // Malta
    "NL", // Países Bajos
    "PL", // Polonia
    "PT", // Portugal
    "RO", // Rumania
    "SK", // Eslovaquia
    "SI", // Eslovenia
    "ES", // España
    "SE", // Suecia
    "GB", // Reino Unido
    "NO", // Noruega
    "CH", // Suiza
    "IS", // Islandia
    "LI", // Liechtenstein
    "MC", // Mónaco
    "SM", // San Marino
    "VA", // Ciudad del Vaticano
    "AD", // Andorra

    // Otros países desarrollados
    "RU", // Rusia
    "JP", // Japón
    "CN", // China
    "AE", // Emiratos Árabes Unidos
    "KW", // Kuwait
    "BH", // Bahréin
    "QA", // Catar
    "SA", // Arabia Saudita
    "OM", // Omán
    "US", // Estados Unidos
    "CA", // Canadá
    "AU", // Australia
    "NZ", // Nueva Zelanda
    "SG", // Singapur
    "AG", // Antigua y Barbuda
    "AW", // Aruba
    "BB", // Barbados
    "BN", // Brunei
    "CW", // Curazao
    "IL", // Israel
    "SC", // Seychelles
  ];

  const isHighIncome = highIncomeCountries.includes(country);

  const pricing: PricingApiResponse = {
    currency: "EUR",
    symbol: "€",
    level: isHighIncome ? "high" : "low",
    countryGroup: isHighIncome ? "Mayor nivel de vida" : "Menor nivel de vida",
    isDevelopment,
    country,
    prices: {
      individual_standard: isHighIncome ? 30 : 15,
      individual_conversation: isHighIncome ? 20 : 10,
      group: isHighIncome ? 10 : 4.5,
    },
  };

  return new Response(JSON.stringify(pricing), {
    headers: {
      "Content-Type": "application/json",
      "Cache-Control": isDevelopment ? "no-cache" : "public, max-age=3600",
    },
  });
}
