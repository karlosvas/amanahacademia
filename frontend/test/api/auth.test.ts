import { describe, it, expect, beforeEach, vi } from "vitest";
import { POST, DELETE } from "../../src/pages/api/auth";
import { log } from "@/services/logger";

// Mock de las rutas
const createMockRequest = (body: any) => ({
  json: vi.fn().mockResolvedValue(body),
});

const createMockCookies = () => ({
  set: vi.fn(),
  delete: vi.fn(),
  get: vi.fn(),
});

vi.mock("@/services/logger", () => ({
  log: {
    error: vi.fn(),
    info: vi.fn(),
    warn: vi.fn(),
    debug: vi.fn(),
  },
}));

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
        }),
      );

      const body = await response.json();
      expect(body.success).toBe(true);
    });

    it("should return 400 when token is missing", async () => {
      mockRequest = createMockRequest({ token: null });

      const response = await POST({
        request: mockRequest,
        cookies: mockCookies,
      } as any);

      expect(response.status).toBe(400);
      expect(mockCookies.set).not.toHaveBeenCalled();

      const body = await response.json();
      expect(body.error).toBe("Invalid token");
    });

    it("should return 400 when token is not a string", async () => {
      mockRequest = createMockRequest({ token: 12345 });

      const response = await POST({
        request: mockRequest,
        cookies: mockCookies,
      } as any);

      expect(response.status).toBe(400);
      expect(mockCookies.set).not.toHaveBeenCalled();
    });

    it("should return 500 on request.json() error", async () => {
      mockRequest.json = vi.fn().mockRejectedValue(new Error("JSON parse error"));

      const response = await POST({
        request: mockRequest,
        cookies: mockCookies,
      } as any);

      expect(response.status).toBe(500);
      expect(log.error).toHaveBeenCalled();
    });

    it("should set secure cookie in production", async () => {
      const token = "prod-token";
      mockRequest = createMockRequest({ token });

      // Mock import.meta.env.PROD usando vi.stubEnv
      vi.stubEnv("PROD", true);

      const response = await POST({
        request: mockRequest,
        cookies: mockCookies,
      } as any);

      expect(mockCookies.set).toHaveBeenCalledWith(
        "session",
        token,
        expect.objectContaining({
          secure: true,
        }),
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
      const response = await DELETE({
        cookies: mockCookies,
      } as any);

      expect(response.status).toBe(200);
      expect(mockCookies.delete).toHaveBeenCalledWith("session", { path: "/" });

      const body = await response.json();
      expect(body.success).toBe(true);
    });

    it("should return correct content-type header", async () => {
      const response = await DELETE({
        cookies: mockCookies,
      } as any);

      expect(response.headers.get("Content-Type")).toBe("application/json");
    });
  });
});
