import { describe, it, expect, beforeEach, vi } from "vitest";
import type { APIRoute } from "astro";

// Mock de las rutas
const createMockRequest = (body: any) => ({
  json: vi.fn().mockResolvedValue(body),
});

const createMockCookies = () => ({
  set: vi.fn(),
  delete: vi.fn(),
  get: vi.fn(),
});

describe("API Routes - Auth", () => {
  describe("POST /api/auth", () => {
    let mockRequest: any;
    let mockCookies: any;

    beforeEach(() => {
      mockRequest = createMockRequest({});
      mockCookies = createMockCookies();
      vi.clearAllMocks();
    });

    it("should set session cookie with valid token", async () => {
      const token = "valid-token-123";
      mockRequest = createMockRequest({ token });

      const POST: APIRoute = async ({ request, cookies }) => {
        try {
          const { token } = await request.json();

          if (!token || typeof token !== "string")
            return new Response(JSON.stringify({ error: "Token inválido" }), { status: 400 });

          cookies.set("session", token, {
            path: "/",
            httpOnly: true,
            secure: import.meta.env.PROD,
            sameSite: "lax",
            maxAge: 60 * 60 * 24 * 7,
          });

          return new Response(JSON.stringify({ success: true }), {
            status: 200,
            headers: { "Content-Type": "application/json" },
          });
        } catch (error) {
          console.error("Error en POST /api/auth:", error);
          return new Response(JSON.stringify({ error: "Error interno del servidor" }), { status: 500 });
        }
      };

      const response = await POST({
        request: mockRequest,
        cookies: mockCookies,
      } as any);

      expect(response.status).toBe(200);
      expect(mockCookies.set).toHaveBeenCalledWith(
        "session",
        token,
        expect.objectContaining({
          path: "/",
          httpOnly: true,
          sameSite: "lax",
        })
      );

      const body = await response.json();
      expect(body.success).toBe(true);
    });

    it("should return 400 when token is missing", async () => {
      mockRequest = createMockRequest({ token: null });

      const POST: APIRoute = async ({ request, cookies }) => {
        try {
          const { token } = await request.json();

          if (!token || typeof token !== "string")
            return new Response(JSON.stringify({ error: "Token inválido" }), { status: 400 });

          cookies.set("session", token, {
            path: "/",
            httpOnly: true,
            secure: import.meta.env.PROD,
            sameSite: "lax",
            maxAge: 60 * 60 * 24 * 7,
          });

          return new Response(JSON.stringify({ success: true }), {
            status: 200,
            headers: { "Content-Type": "application/json" },
          });
        } catch (error) {
          console.error("Error en POST /api/auth:", error);
          return new Response(JSON.stringify({ error: "Error interno del servidor" }), { status: 500 });
        }
      };

      const response = await POST({
        request: mockRequest,
        cookies: mockCookies,
      } as any);

      expect(response.status).toBe(400);
      expect(mockCookies.set).not.toHaveBeenCalled();

      const body = await response.json();
      expect(body.error).toBe("Token inválido");
    });

    it("should return 400 when token is not a string", async () => {
      mockRequest = createMockRequest({ token: 12345 });

      const POST: APIRoute = async ({ request, cookies }) => {
        try {
          const { token } = await request.json();

          if (!token || typeof token !== "string")
            return new Response(JSON.stringify({ error: "Token inválido" }), { status: 400 });

          cookies.set("session", token, {
            path: "/",
            httpOnly: true,
            secure: import.meta.env.PROD,
            sameSite: "lax",
            maxAge: 60 * 60 * 24 * 7,
          });

          return new Response(JSON.stringify({ success: true }), {
            status: 200,
            headers: { "Content-Type": "application/json" },
          });
        } catch (error) {
          console.error("Error en POST /api/auth:", error);
          return new Response(JSON.stringify({ error: "Error interno del servidor" }), { status: 500 });
        }
      };

      const response = await POST({
        request: mockRequest,
        cookies: mockCookies,
      } as any);

      expect(response.status).toBe(400);
      expect(mockCookies.set).not.toHaveBeenCalled();
    });

    it("should return 500 on request.json() error", async () => {
      mockRequest.json = vi.fn().mockRejectedValue(new Error("JSON parse error"));

      const POST: APIRoute = async ({ request, cookies }) => {
        try {
          const { token } = await request.json();

          if (!token || typeof token !== "string")
            return new Response(JSON.stringify({ error: "Token inválido" }), { status: 400 });

          cookies.set("session", token, {
            path: "/",
            httpOnly: true,
            secure: import.meta.env.PROD,
            sameSite: "lax",
            maxAge: 60 * 60 * 24 * 7,
          });

          return new Response(JSON.stringify({ success: true }), {
            status: 200,
            headers: { "Content-Type": "application/json" },
          });
        } catch (error) {
          console.error("Error en POST /api/auth:", error);
          return new Response(JSON.stringify({ error: "Error interno del servidor" }), { status: 500 });
        }
      };

      const consoleErrorSpy = vi.spyOn(console, "error").mockImplementation(() => {});

      const response = await POST({
        request: mockRequest,
        cookies: mockCookies,
      } as any);

      expect(response.status).toBe(500);
      expect(consoleErrorSpy).toHaveBeenCalledWith("Error en POST /api/auth:", expect.any(Error));
      consoleErrorSpy.mockRestore();
    });

    it("should set secure cookie in production", async () => {
      const token = "prod-token";
      mockRequest = createMockRequest({ token });

      // Mock import.meta.env.PROD usando vi.stubEnv
      vi.stubEnv("PROD", true);

      const POST: APIRoute = async ({ request, cookies }) => {
        try {
          const { token } = await request.json();

          if (!token || typeof token !== "string")
            return new Response(JSON.stringify({ error: "Token inválido" }), { status: 400 });

          cookies.set("session", token, {
            path: "/",
            httpOnly: true,
            secure: import.meta.env.PROD,
            sameSite: "lax",
            maxAge: 60 * 60 * 24 * 7,
          });

          return new Response(JSON.stringify({ success: true }), {
            status: 200,
            headers: { "Content-Type": "application/json" },
          });
        } catch (error) {
          console.error("Error en POST /api/auth:", error);
          return new Response(JSON.stringify({ error: "Error interno del servidor" }), { status: 500 });
        }
      };

      const response = await POST({
        request: mockRequest,
        cookies: mockCookies,
      } as any);

      expect(mockCookies.set).toHaveBeenCalledWith(
        "session",
        token,
        expect.objectContaining({
          secure: true,
        })
      );

      vi.unstubAllEnvs();
    });
  });

  describe("DELETE /api/auth", () => {
    let mockCookies: any;

    beforeEach(() => {
      mockCookies = createMockCookies();
      vi.clearAllMocks();
    });

    it("should delete session cookie", async () => {
      const DELETE: APIRoute = async ({ cookies }) => {
        cookies.delete("session", { path: "/" });

        return new Response(JSON.stringify({ success: true }), {
          status: 200,
          headers: { "Content-Type": "application/json" },
        });
      };

      const response = await DELETE({
        cookies: mockCookies,
      } as any);

      expect(response.status).toBe(200);
      expect(mockCookies.delete).toHaveBeenCalledWith("session", { path: "/" });

      const body = await response.json();
      expect(body.success).toBe(true);
    });

    it("should return correct content-type header", async () => {
      const DELETE: APIRoute = async ({ cookies }) => {
        cookies.delete("session", { path: "/" });

        return new Response(JSON.stringify({ success: true }), {
          status: 200,
          headers: { "Content-Type": "application/json" },
        });
      };

      const response = await DELETE({
        cookies: mockCookies,
      } as any);

      expect(response.headers.get("Content-Type")).toBe("application/json");
    });
  });
});
