import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import {
  executeTurnstileIfPresent,
  updateTurnstileVisibility,
} from "@/services/claudflare";
import { log } from "@/services/logger";

vi.mock("@/services/logger", () => ({
  log: {
    error: vi.fn(),
    info: vi.fn(),
    warn: vi.fn(),
    debug: vi.fn(),
  },
}));

describe("claudflare.ts", () => {
  let mockTurnstile: any;

  beforeEach(() => {
    vi.clearAllMocks();

    // Reset DOM
    document.body.innerHTML = "";

    // Setup mock Turnstile
    mockTurnstile = {
      execute: vi.fn(),
    };

    (globalThis as any).turnstile = mockTurnstile;
  });

  afterEach(() => {
    vi.restoreAllMocks();
    delete (globalThis as any).turnstile;
  });

  //////////////////// executeTurnstileIfPresent ////////////////////
  describe("executeTurnstileIfPresent", () => {
    it("should return undefined if turnstile is not present", () => {
      delete (globalThis as any).turnstile;

      const form = document.createElement("form");
      const turnstileDiv = document.createElement("div");
      turnstileDiv.className = "cf-turnstile";
      form.appendChild(turnstileDiv);

      const result = executeTurnstileIfPresent(form);

      expect(result).toBeUndefined();
    });

    it("should return undefined if turnstile div is not present in form", () => {
      const form = document.createElement("form");

      const result = executeTurnstileIfPresent(form);

      expect(result).toBeUndefined();
    });

    it("should execute turnstile if both turnstile and div are present", async () => {
      const form = document.createElement("form");
      const turnstileDiv = document.createElement("div");
      turnstileDiv.className = "cf-turnstile";
      form.appendChild(turnstileDiv);

      // Mock execute to call callback immediately
      mockTurnstile.execute.mockImplementation((element: any, options: any) => {
        options.callback();
      });

      const promise = executeTurnstileIfPresent(form);

      expect(promise).toBeInstanceOf(Promise);

      await expect(promise).resolves.toBeUndefined();

      expect(mockTurnstile.execute).toHaveBeenCalledWith(
        turnstileDiv,
        expect.objectContaining({
          callback: expect.any(Function),
          "error-callback": expect.any(Function),
        }),
      );
    });

    it("should resolve promise when turnstile callback is called", async () => {
      const form = document.createElement("form");
      const turnstileDiv = document.createElement("div");
      turnstileDiv.className = "cf-turnstile";
      form.appendChild(turnstileDiv);

      mockTurnstile.execute.mockImplementation((element: any, options: any) => {
        setTimeout(() => options.callback(), 10);
      });

      const promise = executeTurnstileIfPresent(form);

      await expect(promise).resolves.toBeUndefined();
    });

    it("should reject promise when turnstile error-callback is called", async () => {
      const form = document.createElement("form");
      const turnstileDiv = document.createElement("div");
      turnstileDiv.className = "cf-turnstile";
      form.appendChild(turnstileDiv);

      const errorMessage = "Turnstile verification failed";

      mockTurnstile.execute.mockImplementation((element: any, options: any) => {
        options["error-callback"](errorMessage);
      });

      const promise = executeTurnstileIfPresent(form);

      await expect(promise).rejects.toThrow(
        "Error en la verificación, por favor recarga la página.",
      );

      expect(log.error).toHaveBeenCalled();
    });

    it("should log error and reject when error-callback is called", async () => {
      const form = document.createElement("form");
      const turnstileDiv = document.createElement("div");
      turnstileDiv.className = "cf-turnstile";
      form.appendChild(turnstileDiv);

      const errorObject = { code: "error-123", message: "Failed" };

      mockTurnstile.execute.mockImplementation((element: any, options: any) => {
        options["error-callback"](errorObject);
      });

      const promise = executeTurnstileIfPresent(form);

      await expect(promise).rejects.toThrow(
        "Error en la verificación, por favor recarga la página.",
      );

      expect(log.error).toHaveBeenCalledWith(
        "Error de Turnstile:",
        errorObject,
      );
    });

    it("should reject promise when execute throws an error", async () => {
      const form = document.createElement("form");
      const turnstileDiv = document.createElement("div");
      turnstileDiv.className = "cf-turnstile";
      form.appendChild(turnstileDiv);

      const thrownError = new Error("Execute failed");

      mockTurnstile.execute.mockImplementation(() => {
        throw thrownError;
      });

      const promise = executeTurnstileIfPresent(form);

      await expect(promise).rejects.toThrow("Execute failed");
    });

    it("should reject promise when execute throws a non-Error object", async () => {
      const form = document.createElement("form");
      const turnstileDiv = document.createElement("div");
      turnstileDiv.className = "cf-turnstile";
      form.appendChild(turnstileDiv);

      mockTurnstile.execute.mockImplementation(() => {
        throw "String error";
      });

      const promise = executeTurnstileIfPresent(form);

      await expect(promise).rejects.toThrow("String error");
    });

    it("should handle number thrown as error", async () => {
      const form = document.createElement("form");
      const turnstileDiv = document.createElement("div");
      turnstileDiv.className = "cf-turnstile";
      form.appendChild(turnstileDiv);

      mockTurnstile.execute.mockImplementation(() => {
        throw 404;
      });

      const promise = executeTurnstileIfPresent(form);

      await expect(promise).rejects.toThrow("404");
    });

    it("should handle object thrown as error", async () => {
      const form = document.createElement("form");
      const turnstileDiv = document.createElement("div");
      turnstileDiv.className = "cf-turnstile";
      form.appendChild(turnstileDiv);

      mockTurnstile.execute.mockImplementation(() => {
        throw { custom: "error" };
      });

      const promise = executeTurnstileIfPresent(form);

      await expect(promise).rejects.toThrow("[object Object]");
    });

    it("should find turnstile div by class selector", () => {
      const form = document.createElement("form");
      const turnstileDiv = document.createElement("div");
      turnstileDiv.className = "cf-turnstile extra-class";
      form.appendChild(turnstileDiv);

      mockTurnstile.execute.mockImplementation((element: any, options: any) => {
        options.callback();
      });

      const promise = executeTurnstileIfPresent(form);

      expect(promise).toBeInstanceOf(Promise);
      expect(mockTurnstile.execute).toHaveBeenCalledWith(
        turnstileDiv,
        expect.any(Object),
      );
    });

    it("should only select first turnstile div if multiple exist", async () => {
      const form = document.createElement("form");

      const turnstileDiv1 = document.createElement("div");
      turnstileDiv1.className = "cf-turnstile";
      form.appendChild(turnstileDiv1);

      const turnstileDiv2 = document.createElement("div");
      turnstileDiv2.className = "cf-turnstile";
      form.appendChild(turnstileDiv2);

      mockTurnstile.execute.mockImplementation((element: any, options: any) => {
        options.callback();
      });

      await executeTurnstileIfPresent(form);

      expect(mockTurnstile.execute).toHaveBeenCalledWith(
        turnstileDiv1,
        expect.any(Object),
      );
    });

    it("should work with form containing other elements", async () => {
      const form = document.createElement("form");

      const input = document.createElement("input");
      input.type = "text";
      form.appendChild(input);

      const button = document.createElement("button");
      form.appendChild(button);

      const turnstileDiv = document.createElement("div");
      turnstileDiv.className = "cf-turnstile";
      form.appendChild(turnstileDiv);

      mockTurnstile.execute.mockImplementation((element: any, options: any) => {
        options.callback();
      });

      const promise = executeTurnstileIfPresent(form);

      await expect(promise).resolves.toBeUndefined();
      expect(mockTurnstile.execute).toHaveBeenCalledWith(
        turnstileDiv,
        expect.any(Object),
      );
    });
  });

  //////////////////// updateTurnstileVisibility ////////////////////
  describe("updateTurnstileVisibility", () => {
    it("should execute without errors when both widgets exist - isRegister false", () => {
      const loginWidget = document.createElement("div");
      loginWidget.id = "turnstile-login";
      document.body.appendChild(loginWidget);

      const registerWidget = document.createElement("div");
      registerWidget.id = "turnstile-register";
      document.body.appendChild(registerWidget);

      expect(() => updateTurnstileVisibility(false)).not.toThrow();
    });

    it("should execute without errors when both widgets exist - isRegister true", () => {
      const loginWidget = document.createElement("div");
      loginWidget.id = "turnstile-login";
      document.body.appendChild(loginWidget);

      const registerWidget = document.createElement("div");
      registerWidget.id = "turnstile-register";
      document.body.appendChild(registerWidget);

      expect(() => updateTurnstileVisibility(true)).not.toThrow();
    });

    it("should toggle visibility correctly when called multiple times", () => {
      const loginWidget = document.createElement("div");
      loginWidget.id = "turnstile-login";
      document.body.appendChild(loginWidget);

      const registerWidget = document.createElement("div");
      registerWidget.id = "turnstile-register";
      document.body.appendChild(registerWidget);

      expect(() => {
        updateTurnstileVisibility(true);
        updateTurnstileVisibility(false);
        updateTurnstileVisibility(true);
      }).not.toThrow();
    });

    it("should not throw error if login widget is missing", () => {
      const registerWidget = document.createElement("div");
      registerWidget.id = "turnstile-register";
      document.body.appendChild(registerWidget);

      expect(() => updateTurnstileVisibility(false)).not.toThrow();
      expect(() => updateTurnstileVisibility(true)).not.toThrow();
    });

    it("should not throw error if register widget is missing", () => {
      const loginWidget = document.createElement("div");
      loginWidget.id = "turnstile-login";
      document.body.appendChild(loginWidget);

      expect(() => updateTurnstileVisibility(false)).not.toThrow();
      expect(() => updateTurnstileVisibility(true)).not.toThrow();
    });

    it("should not throw error if both widgets are missing", () => {
      expect(() => updateTurnstileVisibility(false)).not.toThrow();
      expect(() => updateTurnstileVisibility(true)).not.toThrow();
    });

    it("should execute without errors when only register widget exists", () => {
      const registerWidget = document.createElement("div");
      registerWidget.id = "turnstile-register";
      document.body.appendChild(registerWidget);

      expect(() => updateTurnstileVisibility(true)).not.toThrow();
      expect(() => updateTurnstileVisibility(false)).not.toThrow();
    });

    it("should execute without errors with elements in different initial states", () => {
      const loginWidget = document.createElement("div");
      loginWidget.id = "turnstile-login";
      loginWidget.style.display = "flex";
      document.body.appendChild(loginWidget);

      const registerWidget = document.createElement("div");
      registerWidget.id = "turnstile-register";
      registerWidget.style.display = "block";
      document.body.appendChild(registerWidget);

      expect(() => updateTurnstileVisibility(false)).not.toThrow();
      expect(() => updateTurnstileVisibility(true)).not.toThrow();
    });

    it("should use correct element IDs when retrieving widgets", () => {
      const loginWidget = document.createElement("div");
      loginWidget.id = "turnstile-login";
      document.body.appendChild(loginWidget);

      const registerWidget = document.createElement("div");
      registerWidget.id = "turnstile-register";
      document.body.appendChild(registerWidget);

      const getElementByIdSpy = vi.spyOn(document, "getElementById");

      updateTurnstileVisibility(false);

      expect(getElementByIdSpy).toHaveBeenCalledWith("turnstile-login");
      expect(getElementByIdSpy).toHaveBeenCalledWith("turnstile-register");
    });

    it("should handle consecutive calls with same parameter", () => {
      const loginWidget = document.createElement("div");
      loginWidget.id = "turnstile-login";
      document.body.appendChild(loginWidget);

      const registerWidget = document.createElement("div");
      registerWidget.id = "turnstile-register";
      document.body.appendChild(registerWidget);

      expect(() => {
        updateTurnstileVisibility(true);
        updateTurnstileVisibility(true);
      }).not.toThrow();
    });

    it("should handle alternating parameter calls", () => {
      const loginWidget = document.createElement("div");
      loginWidget.id = "turnstile-login";
      document.body.appendChild(loginWidget);

      const registerWidget = document.createElement("div");
      registerWidget.id = "turnstile-register";
      document.body.appendChild(registerWidget);

      expect(() => {
        updateTurnstileVisibility(true);
        updateTurnstileVisibility(false);
        updateTurnstileVisibility(true);
        updateTurnstileVisibility(false);
      }).not.toThrow();
    });
  });
});
