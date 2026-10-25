import { describe, it, expect, beforeEach, vi, type Mock } from "vitest";
import { getActivePrices } from "@/config/pricing";

const env = vi.hoisted(() => ({
  INTERNAL_API_SECRET: "internal-secret" as string | undefined,
}));

vi.mock("astro:env/server", () => env);
vi.mock("@/services/logger", () => ({
  log: {
    debug: vi.fn(),
    info: vi.fn(),
    warn: vi.fn(),
    error: vi.fn(),
  },
}));

const createRequest = (
  body: unknown,
  headers: Record<string, string> = {},
): Request =>
  new Request("http://example.com/api/checkout", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer user-token",
      ...headers,
    },
    body: typeof body === "string" ? body : JSON.stringify(body),
  });

const callCheckout = async (request: Request): Promise<Response> => {
  const { POST } = await import("@/pages/api/checkout");
  return POST({ request } as any) as Promise<Response>;
};

describe("API Routes - Checkout", () => {
  let mockedFetch: Mock;

  beforeEach(() => {
    vi.clearAllMocks();
    env.INTERNAL_API_SECRET = "internal-secret";
    globalThis.fetch = vi.fn() as any;
    mockedFetch = globalThis.fetch as unknown as Mock;
    mockedFetch.mockResolvedValue(
      new Response(
        JSON.stringify({ success: true, data: { client_secret: "cs" } }),
        { status: 200 },
      ),
    );
  });

  it("should require an authorization header", async () => {
    const request = new Request("http://example.com/api/checkout", {
      method: "POST",
      body: JSON.stringify({ booking_uid: "booking-1" }),
    });

    const response = await callCheckout(request);

    expect(response.status).toBe(401);
    expect(mockedFetch).not.toHaveBeenCalled();
  });

  it("should fail closed when the internal secret is not configured", async () => {
    env.INTERNAL_API_SECRET = undefined;

    const response = await callCheckout(createRequest({ booking_uid: "b-1" }));

    expect(response.status).toBe(503);
    expect(mockedFetch).not.toHaveBeenCalled();
  });

  it("should reject invalid bodies", async () => {
    const response = await callCheckout(createRequest("not json"));

    expect(response.status).toBe(400);
  });

  it("should forward the server prices and never a client amount", async () => {
    const response = await callCheckout(
      createRequest(
        {
          booking_uid: "booking-1",
          amount: 1,
          prices: { individual_standard: 0.01 },
        },
        { "x-vercel-ip-country": "MX", "CF-IPCountry": "ES" },
      ),
    );

    expect(response.status).toBe(200);
    const [url, init] = mockedFetch.mock.calls[0];
    expect(url).toMatch(/\/payment\/intent$/);
    expect(init.headers).toEqual({
      "Content-Type": "application/json",
      Authorization: "Bearer user-token",
      "X-Internal-Secret": "internal-secret",
    });
    expect(JSON.parse(init.body)).toEqual({
      booking_uid: "booking-1",
      prices: getActivePrices("low"),
    });
  });

  it("should forward the weeks flow", async () => {
    await callCheckout(
      createRequest(
        { class_type: "group-class", weeks: 4 },
        { "x-vercel-ip-country": "FR" },
      ),
    );

    expect(JSON.parse(mockedFetch.mock.calls[0][1].body)).toEqual({
      class_type: "group-class",
      weeks: 4,
      prices: getActivePrices("high"),
    });
  });

  it("should pass through backend errors", async () => {
    mockedFetch.mockResolvedValue(
      new Response(JSON.stringify({ success: false, error: "already paid" }), {
        status: 409,
      }),
    );

    const response = await callCheckout(createRequest({ booking_uid: "b-1" }));

    expect(response.status).toBe(409);
    expect(await response.json()).toEqual({
      success: false,
      error: "already paid",
    });
  });

  it("should return 502 when the backend is unreachable", async () => {
    mockedFetch.mockRejectedValue(new Error("Network error"));

    const response = await callCheckout(createRequest({ booking_uid: "b-1" }));

    expect(response.status).toBe(502);
  });
});
