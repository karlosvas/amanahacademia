import type { APIRoute } from "astro";
import { INTERNAL_API_SECRET } from "astro:env/server";
import {
  getActivePrices,
  getPriceLevel,
  resolveCountry,
} from "@/config/pricing";
import { log } from "@/services/logger";

function jsonResponse(status: number, body: unknown): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "Content-Type": "application/json" },
  });
}

function optionalString(value: unknown): string | undefined {
  return typeof value === "string" && value.trim() !== ""
    ? value.trim()
    : undefined;
}

function optionalInteger(value: unknown): number | undefined {
  return typeof value === "number" && Number.isInteger(value)
    ? value
    : undefined;
}

export const POST: APIRoute = async ({ request }) => {
  const authorization = request.headers.get("authorization");
  if (!authorization?.startsWith("Bearer ")) {
    return jsonResponse(401, {
      success: false,
      error: "Authentication required",
    });
  }

  if (!INTERNAL_API_SECRET) {
    log.error("INTERNAL_API_SECRET is not configured");
    return jsonResponse(503, {
      success: false,
      error: "Payments are not configured",
    });
  }

  let body: Record<string, unknown>;
  try {
    body = await request.json();
  } catch {
    return jsonResponse(400, { success: false, error: "Invalid body" });
  }

  const prices = getActivePrices(getPriceLevel(resolveCountry(request)));

  try {
    const response = await fetch(
      `${import.meta.env.PUBLIC_BACKEND_URL}/payment/intent`,
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: authorization,
          "X-Internal-Secret": INTERNAL_API_SECRET,
        },
        body: JSON.stringify({
          booking_uid: optionalString(body.booking_uid),
          class_type: optionalString(body.class_type),
          weeks: optionalInteger(body.weeks),
          prices,
        }),
      },
    );

    const text = await response.text();
    return new Response(
      text || JSON.stringify({ success: false, error: response.statusText }),
      {
        status: response.status,
        headers: {
          "Content-Type": "application/json",
          "Cache-Control": "no-store",
        },
      },
    );
  } catch (error) {
    log.error("Error creating checkout:", error);
    return jsonResponse(502, {
      success: false,
      error: "Payment service unavailable",
    });
  }
};
