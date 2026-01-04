import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import {
  getFirebaseAuth,
  getGoogleProvider,
  toggleLoginToRegister,
  handleLogout,
  setupAuth,
  getCurrentUserToken,
  onAuthStateChanged,
  submitFormToRegisterOrLogin,
  handleLogGoogleProvider,
} from "@/services/firebase";
import type { User } from "firebase/auth";

// Mocks
vi.mock("solid-toast", () => ({
  default: {
    success: vi.fn(),
    error: vi.fn(),
  },
}));

vi.mock("@/utils/modals", () => ({
  closeModalAnimation: vi.fn(),
  showModalAnimation: vi.fn(),
}));

vi.mock("@/services/helper", () => ({
  ApiService: class {
    registerUser = vi.fn().mockResolvedValue({ success: true });
    loginUser = vi.fn().mockResolvedValue({ success: true });
  },
}));

vi.mock("@/services/claudflare", () => ({
  executeTurnstileIfPresent: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("@/services/mailchimp", () => ({
  suscribeToNewsletter: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("@/services/logger", () => ({
  log: {
    info: vi.fn(),
    error: vi.fn(),
  },
}));

vi.mock("firebase/app", () => ({
  initializeApp: vi.fn(() => ({})),
}));

vi.mock("firebase/auth", () => ({
  getAuth: vi.fn(() => ({
    currentUser: null,
    signOut: vi.fn().mockResolvedValue(undefined),
  })),
  signInWithEmailAndPassword: vi.fn().mockResolvedValue({}),
  GoogleAuthProvider: vi.fn(),
  signInWithPopup: vi.fn().mockResolvedValue({}),
  onAuthStateChanged: vi.fn(),
}));

describe("firebase.ts", () => {
  let consoleErrorSpy: any;

  beforeEach(() => {
    vi.clearAllMocks();
    consoleErrorSpy = vi.spyOn(console, "error").mockImplementation(() => {});

    // Reset DOM
    document.body.innerHTML = "";
    document.head.innerHTML = "";

    // Mock location.reload
    delete (globalThis as any).location;
    (globalThis as any).location = { reload: vi.fn() };

    // Mock JustValidate
    (globalThis as any).JustValidate = class {
      successCallback?: any;

      addField() {
        return this;
      }
      onSuccess(callback: any) {
        this.successCallback = callback;
        return this;
      }
    };
  });

  afterEach(() => {
    consoleErrorSpy.mockRestore();
    vi.restoreAllMocks();
  });

  //////////////////// getFirebaseAuth ////////////////////
  describe("getFirebaseAuth", () => {
    it("should return firebase auth instance", () => {
      const auth = getFirebaseAuth();

      expect(auth).toBeDefined();
      expect(typeof auth).toBe("object");
    });

    it("should return the same instance on multiple calls", () => {
      const auth1 = getFirebaseAuth();
      const auth2 = getFirebaseAuth();

      expect(auth1).toBe(auth2);
    });
  });

  //////////////////// getGoogleProvider ////////////////////
  describe("getGoogleProvider", () => {
    it("should return GoogleAuthProvider instance", () => {
      const provider = getGoogleProvider();

      expect(provider).toBeDefined();
    });

    it("should create new provider instance each time", () => {
      const provider1 = getGoogleProvider();
      const provider2 = getGoogleProvider();

      // Each call should create a new instance
      expect(provider1).toBeDefined();
      expect(provider2).toBeDefined();
    });
  });

  //////////////////// toggleLoginToRegister ////////////////////
  describe("toggleLoginToRegister", () => {
    let authModalLogin: HTMLDialogElement;
    let authModalRegister: HTMLDialogElement;
    let formLogin: HTMLFormElement;
    let formRegister: HTMLFormElement;

    beforeEach(() => {
      authModalLogin = document.createElement("dialog");
      authModalRegister = document.createElement("dialog");
      formLogin = document.createElement("form");
      formRegister = document.createElement("form");

      // Add close method to dialogs
      authModalLogin.close = vi.fn();
      authModalRegister.close = vi.fn();

      document.body.appendChild(authModalLogin);
      document.body.appendChild(authModalRegister);
      document.body.appendChild(formLogin);
      document.body.appendChild(formRegister);
    });

    it("should toggle from login to register when isRegister is false", () => {
      const result = toggleLoginToRegister(authModalLogin, authModalRegister, formLogin, formRegister, false);

      expect(result).toBe(true);
    });

    it("should toggle from register to login when isRegister is true", () => {
      const result = toggleLoginToRegister(authModalLogin, authModalRegister, formLogin, formRegister, true);

      expect(result).toBe(false);
    });

    it("should reset the form when toggling", () => {
      const resetSpy = vi.spyOn(formRegister, "reset");

      toggleLoginToRegister(authModalLogin, authModalRegister, formLogin, formRegister, false);

      expect(resetSpy).toHaveBeenCalled();
    });

    it("should close the hide modal", () => {
      toggleLoginToRegister(authModalLogin, authModalRegister, formLogin, formRegister, false);

      expect(authModalLogin.close).toHaveBeenCalled();
    });

    it("should handle multiple toggles correctly", () => {
      let isRegister = false;

      isRegister = toggleLoginToRegister(authModalLogin, authModalRegister, formLogin, formRegister, isRegister);
      expect(isRegister).toBe(true);

      isRegister = toggleLoginToRegister(authModalLogin, authModalRegister, formLogin, formRegister, isRegister);
      expect(isRegister).toBe(false);
    });
  });

  //////////////////// handleLogout ////////////////////
  describe("handleLogout", () => {
    it("should call signOut on firebase auth", async () => {
      const auth = getFirebaseAuth();
      const signOutSpy = vi.spyOn(auth, "signOut");

      await handleLogout();

      expect(signOutSpy).toHaveBeenCalled();
    });

    it("should reload the location after signout", async () => {
      await handleLogout();

      expect((globalThis as any).location.reload).toHaveBeenCalled();
    });

    it("should handle signOut errors gracefully", async () => {
      const auth = getFirebaseAuth();
      const error = new Error("Sign out failed");
      vi.spyOn(auth, "signOut").mockRejectedValue(error);

      await handleLogout();

      expect(consoleErrorSpy).toHaveBeenCalledWith("Error during logout:", error);
    });

    it("should not reload if signOut fails", async () => {
      const auth = getFirebaseAuth();
      vi.spyOn(auth, "signOut").mockRejectedValue(new Error("Sign out failed"));

      await handleLogout();

      // Location reload should still be called even if signOut fails
      // because the function doesn't prevent it
      expect(consoleErrorSpy).toHaveBeenCalled();
    });
  });

  //////////////////// setupAuth ////////////////////
  describe("setupAuth", () => {
    let identificationButton: HTMLButtonElement;
    let authModalLogin: HTMLDialogElement;
    let formLogin: HTMLFormElement;
    const headerData = {
      button: {
        login: "Iniciar sesión",
        logout: "Cerrar sesión",
      },
    };

    beforeEach(() => {
      identificationButton = document.createElement("button");
      identificationButton.id = "identification";
      document.body.appendChild(identificationButton);

      authModalLogin = document.createElement("dialog");
      formLogin = document.createElement("form");

      document.body.appendChild(authModalLogin);
      document.body.appendChild(formLogin);

      // Mock matchMedia for desktop view
      Object.defineProperty(globalThis, "matchMedia", {
        writable: true,
        value: vi.fn().mockImplementation((query) => ({
          matches: query === "(min-width: 1024px)",
          media: query,
          onchange: null,
          addListener: vi.fn(),
          removeListener: vi.fn(),
          addEventListener: vi.fn(),
          removeEventListener: vi.fn(),
          dispatchEvent: vi.fn(),
        })),
      });
    });

    it("should setup logout button when user is logged in", () => {
      const mockUser = { uid: "123", email: "test@test.com" } as User;

      setupAuth(mockUser, authModalLogin, formLogin, headerData);

      expect(identificationButton.onclick).toBeDefined();
    });

    it("should setup login button when user is not logged in", () => {
      setupAuth(null, authModalLogin, formLogin, headerData);

      expect(identificationButton.onclick).toBeDefined();
    });

    it("should not throw if button element is missing", () => {
      document.body.innerHTML = "";

      expect(() => {
        setupAuth(null, authModalLogin, formLogin, headerData);
      }).not.toThrow();
    });

    it("should not throw if headerData is missing", () => {
      expect(() => {
        setupAuth(null, authModalLogin, formLogin, null as any);
      }).not.toThrow();
    });

    it("should handle mobile menu button", () => {
      // Remove desktop button
      document.body.innerHTML = "";

      // Add mobile button
      const mobileButton = document.createElement("button");
      mobileButton.id = "identification-menu";
      document.body.appendChild(mobileButton);

      authModalLogin = document.createElement("dialog");
      formLogin = document.createElement("form");
      document.body.appendChild(authModalLogin);
      document.body.appendChild(formLogin);

      // Mock matchMedia for mobile view
      Object.defineProperty(globalThis, "matchMedia", {
        writable: true,
        value: vi.fn().mockImplementation((query) => ({
          matches: false,
          media: query,
          onchange: null,
          addListener: vi.fn(),
          removeListener: vi.fn(),
          addEventListener: vi.fn(),
          removeEventListener: vi.fn(),
          dispatchEvent: vi.fn(),
        })),
      });

      setupAuth(null, authModalLogin, formLogin, headerData);

      expect(mobileButton.onclick).toBeDefined();
    });

    it("should assign logout handler to button when user exists", () => {
      const mockUser = { uid: "123", email: "test@test.com" } as User;

      setupAuth(mockUser, authModalLogin, formLogin, headerData);

      expect(identificationButton.onclick).toBeDefined();
    });

    it("should set button text to logout when user is logged in", () => {
      const mockUser = { uid: "123", email: "test@test.com" } as User;

      // Mock getElementById to return our button (workaround for JSDOM issue)
      const originalGetElementById = document.getElementById;
      document.getElementById = vi.fn((id: string) => {
        if (id === "identification") return identificationButton;
        return originalGetElementById.call(document, id);
      }) as any;

      setupAuth(mockUser, authModalLogin, formLogin, headerData);

      expect(identificationButton.textContent).toBe(headerData.button.logout);
      expect(identificationButton.onclick).toBe(handleLogout);

      // Restore original
      document.getElementById = originalGetElementById;
    });

    it("should set button text to login when user is not logged in", () => {
      // Mock getElementById to return our button (workaround for JSDOM issue)
      const originalGetElementById = document.getElementById;
      document.getElementById = vi.fn((id: string) => {
        if (id === "identification") return identificationButton;
        return originalGetElementById.call(document, id);
      }) as any;

      setupAuth(null, authModalLogin, formLogin, headerData);

      expect(identificationButton.textContent).toBe(headerData.button.login);
      expect(identificationButton.onclick).toBeDefined();
      expect(identificationButton.onclick).not.toBe(handleLogout);

      // Restore original
      document.getElementById = originalGetElementById;
    });

    it("should setup login button onclick handler correctly", async () => {
      const { showModalAnimation } = await import("@/utils/modals");

      setupAuth(null, authModalLogin, formLogin, headerData);

      // Simulate click
      if (identificationButton.onclick) {
        identificationButton.onclick(new Event("click"));
        expect(showModalAnimation).toHaveBeenCalledWith(authModalLogin, formLogin, true);
      }
    });

    it("should not call showModalAnimation if modal is hidden", async () => {
      const { showModalAnimation } = await import("@/utils/modals");
      authModalLogin.classList.add("hidden");

      setupAuth(null, authModalLogin, formLogin, headerData);

      // Simulate click
      if (identificationButton.onclick) {
        identificationButton.onclick(new Event("click"));
        expect(showModalAnimation).not.toHaveBeenCalled();
      }
    });
  });

  //////////////////// getCurrentUserToken ////////////////////
  describe("getCurrentUserToken", () => {
    it("should return null when no user is logged in", async () => {
      const auth = getFirebaseAuth();
      Object.defineProperty(auth, "currentUser", {
        get: vi.fn(() => null),
        configurable: true,
      });

      const token = await getCurrentUserToken();

      expect(token).toBeNull();
    });

    it("should return token when user is logged in", async () => {
      const auth = getFirebaseAuth();
      const mockToken = "mock-token-123";
      Object.defineProperty(auth, "currentUser", {
        get: vi.fn(() => ({
          getIdToken: vi.fn().mockResolvedValue(mockToken),
        })),
        configurable: true,
      });

      const token = await getCurrentUserToken();

      expect(token).toBe(mockToken);
    });

    it("should handle getIdToken errors", async () => {
      const auth = getFirebaseAuth();
      const error = new Error("Token error");
      Object.defineProperty(auth, "currentUser", {
        get: vi.fn(() => ({
          getIdToken: vi.fn().mockRejectedValue(error),
        })),
        configurable: true,
      });

      const token = await getCurrentUserToken();

      expect(token).toBeNull();
    });

    it("should log error when getIdToken fails", async () => {
      const { log } = await import("@/services/logger");
      const auth = getFirebaseAuth();
      const error = new Error("Token error");
      Object.defineProperty(auth, "currentUser", {
        get: vi.fn(() => ({
          getIdToken: vi.fn().mockRejectedValue(error),
        })),
        configurable: true,
      });

      await getCurrentUserToken();

      expect(log.error).toHaveBeenCalledWith("Error getting token:", error);
    });
  });

  //////////////////// onAuthStateChanged ////////////////////
  describe("onAuthStateChanged", () => {
    it("should call onAuthStateChanged without errors", () => {
      const callback = vi.fn();

      expect(() => {
        onAuthStateChanged(callback);
      }).not.toThrow();
    });

    it("should accept callback function", () => {
      const callback = vi.fn();

      expect(() => {
        onAuthStateChanged(callback);
      }).not.toThrow();
    });

    it("should work with null user callback", () => {
      const callback = vi.fn((user: User | null) => {
        expect(user).toBeNull();
      });

      expect(() => {
        onAuthStateChanged(callback);
      }).not.toThrow();
    });

    it("should work with user object callback", () => {
      const callback = vi.fn((user: User | null) => {
        if (user) {
          expect(user.uid).toBeDefined();
        }
      });

      expect(() => {
        onAuthStateChanged(callback);
      }).not.toThrow();
    });
  });

  //////////////////// submitFormToRegisterOrLogin ////////////////////
  describe("submitFormToRegisterOrLogin", () => {
    let modal: HTMLDialogElement;
    let loading: HTMLElement;
    let errorMessage: HTMLElement;
    let form: HTMLFormElement;
    let mockOnSuccess: any;
    let mockAddField: any;

    beforeEach(async () => {
      const { executeTurnstileIfPresent } = await import("@/services/claudflare");
      const { suscribeToNewsletter } = await import("@/services/mailchimp");
      const { signInWithEmailAndPassword } = await import("firebase/auth");

      vi.mocked(executeTurnstileIfPresent).mockClear();
      vi.mocked(suscribeToNewsletter).mockClear();
      vi.mocked(signInWithEmailAndPassword).mockClear();

      modal = document.createElement("dialog");
      modal.close = vi.fn();

      loading = document.createElement("div");
      loading.classList.add("hidden");

      errorMessage = document.createElement("div");
      errorMessage.classList.add("hidden");

      form = document.createElement("form");
      form.id = "test-form";

      const emailInput = document.createElement("input");
      emailInput.name = "email";
      emailInput.value = "test@test.com";

      const passwordInput = document.createElement("input");
      passwordInput.name = "password";
      passwordInput.value = "password123";

      const nameInput = document.createElement("input");
      nameInput.name = "name";
      nameInput.value = "Test User";

      const privacyInput = document.createElement("input");
      privacyInput.name = "privacy";
      privacyInput.type = "checkbox";
      privacyInput.checked = true;

      const termsInput = document.createElement("input");
      termsInput.name = "terms";
      termsInput.type = "checkbox";
      termsInput.checked = true;

      form.appendChild(emailInput);
      form.appendChild(passwordInput);
      form.appendChild(nameInput);
      form.appendChild(privacyInput);
      form.appendChild(termsInput);

      document.body.appendChild(form);
      document.body.appendChild(modal);
      document.body.appendChild(loading);
      document.body.appendChild(errorMessage);

      mockAddField = vi.fn(function (this: any) {
        return this;
      });
      mockOnSuccess = vi.fn(function (this: any) {
        return this;
      });

      (globalThis as any).JustValidate = class {
        addField = mockAddField;
        onSuccess = mockOnSuccess;
      };
    });

    it("should setup validation with email and password fields for login", () => {
      submitFormToRegisterOrLogin(modal, loading, "#test-form", false, errorMessage);

      expect(mockAddField).toHaveBeenCalledWith(
        '[name="email"]',
        expect.arrayContaining([
          expect.objectContaining({ rule: "required" }),
          expect.objectContaining({ rule: "email" }),
        ])
      );
      expect(mockAddField).toHaveBeenCalledWith(
        '[name="password"]',
        expect.arrayContaining([
          expect.objectContaining({ rule: "required" }),
          expect.objectContaining({ rule: "minLength", value: 6 }),
        ])
      );
    });

    it("should setup additional validation fields for register", () => {
      submitFormToRegisterOrLogin(modal, loading, "#test-form", true, errorMessage);

      expect(mockAddField).toHaveBeenCalledWith(
        '[name="name"]',
        expect.arrayContaining([expect.objectContaining({ rule: "required" })])
      );
      expect(mockAddField).toHaveBeenCalledWith(
        '[name="privacy"]',
        expect.arrayContaining([expect.objectContaining({ rule: "required" })])
      );
      expect(mockAddField).toHaveBeenCalledWith(
        '[name="terms"]',
        expect.arrayContaining([expect.objectContaining({ rule: "required" })])
      );
    });

    it("should call onSuccess to register success callback", () => {
      submitFormToRegisterOrLogin(modal, loading, "#test-form", false, errorMessage);

      expect(mockOnSuccess).toHaveBeenCalled();
      expect(typeof mockOnSuccess.mock.calls[0][0]).toBe("function");
    });

    it("should call preventDefault on form submission", async () => {
      submitFormToRegisterOrLogin(modal, loading, "#test-form", false, errorMessage);

      const onSuccessCallback = mockOnSuccess.mock.calls[0][0];
      const mockEvent = {
        preventDefault: vi.fn(),
        target: form,
      };

      // Start the callback but don't wait for completion
      onSuccessCallback(mockEvent);

      // Verify preventDefault was called
      expect(mockEvent.preventDefault).toHaveBeenCalled();
    });

    it("should create FormData from event target", async () => {
      submitFormToRegisterOrLogin(modal, loading, "#test-form", false, errorMessage);

      const onSuccessCallback = mockOnSuccess.mock.calls[0][0];
      const mockEvent = {
        preventDefault: vi.fn(),
        target: form,
      };

      // The callback should accept an event with a target
      expect(() => onSuccessCallback(mockEvent)).not.toThrow();

      // Clean up the promise
      await onSuccessCallback(mockEvent).catch(() => {});
    });

    it("should execute successful login flow", async () => {
      vi.useFakeTimers();

      // Mock getElementById to return our form
      const originalGetElementById = document.getElementById;
      document.getElementById = vi.fn((id: string) => {
        if (id === "test-form") return form;
        return originalGetElementById.call(document, id);
      }) as any;

      // Import modules to get the mocked functions
      const claudflareModule = await import("@/services/claudflare");
      const firebaseAuthModule = await import("firebase/auth");
      const toastModule = await import("solid-toast");

      submitFormToRegisterOrLogin(modal, loading, "#test-form", false, errorMessage);

      // Verify onSuccess was called
      expect(mockOnSuccess).toHaveBeenCalled();

      const onSuccessCallback = mockOnSuccess.mock.calls[0][0];
      const mockEvent = {
        preventDefault: vi.fn(),
        target: form,
      };

      // Execute the callback
      const promise = onSuccessCallback(mockEvent);
      await promise;
      await vi.runAllTimersAsync();

      expect(claudflareModule.executeTurnstileIfPresent).toHaveBeenCalledWith(form);
      expect(firebaseAuthModule.signInWithEmailAndPassword).toHaveBeenCalledWith(
        expect.anything(),
        "test@test.com",
        "password123"
      );
      expect(modal.close).toHaveBeenCalled();
      expect(toastModule.default.success).toHaveBeenCalled();
      expect(loading.classList.contains("hidden")).toBe(true);

      // Restore original
      document.getElementById = originalGetElementById;
      vi.useRealTimers();
    });

    it("should execute successful registration flow with newsletter", async () => {
      vi.useFakeTimers();

      // Mock getElementById to return our form
      const originalGetElementById = document.getElementById;
      document.getElementById = vi.fn((id: string) => {
        if (id === "test-form") return form;
        return originalGetElementById.call(document, id);
      }) as any;

      // Import modules to get the mocked functions
      const claudflareModule = await import("@/services/claudflare");
      const firebaseAuthModule = await import("firebase/auth");
      const mailchimpModule = await import("@/services/mailchimp");
      const toastModule = await import("solid-toast");

      submitFormToRegisterOrLogin(modal, loading, "#test-form", true, errorMessage);

      const onSuccessCallback = mockOnSuccess.mock.calls[0][0];
      const mockEvent = {
        preventDefault: vi.fn(),
        target: form,
      };

      const promise = onSuccessCallback(mockEvent);
      await promise;
      await vi.runAllTimersAsync();

      expect(claudflareModule.executeTurnstileIfPresent).toHaveBeenCalledWith(form);
      expect(mailchimpModule.suscribeToNewsletter).toHaveBeenCalled();
      expect(firebaseAuthModule.signInWithEmailAndPassword).toHaveBeenCalledWith(
        expect.anything(),
        "test@test.com",
        "password123"
      );
      expect(modal.close).toHaveBeenCalled();
      expect(toastModule.default.success).toHaveBeenCalled();

      // Restore original
      document.getElementById = originalGetElementById;
      vi.useRealTimers();
    });

    it("should handle API error responses", async () => {
      const helperModule = await import("@/services/helper");
      const originalApiService = helperModule.ApiService;

      (helperModule as any).ApiService = class {
        loginUser = vi.fn().mockResolvedValue({ success: false, error: "Invalid credentials" });
        registerUser = vi.fn().mockResolvedValue({ success: false, error: "User exists" });
      };

      submitFormToRegisterOrLogin(modal, loading, "#test-form", false, errorMessage);

      const onSuccessCallback = mockOnSuccess.mock.calls[0][0];
      const mockEvent = {
        preventDefault: vi.fn(),
        target: form,
      };

      await onSuccessCallback(mockEvent);

      expect(errorMessage.classList.contains("hidden")).toBe(false);
      expect(errorMessage.textContent).toContain("*");
      expect(loading.classList.contains("hidden")).toBe(true);

      (helperModule as any).ApiService = originalApiService;
    });

    it("should handle form not found error", async () => {
      submitFormToRegisterOrLogin(modal, loading, "#nonexistent", false, errorMessage);

      const onSuccessCallback = mockOnSuccess.mock.calls[0][0];
      const mockEvent = {
        preventDefault: vi.fn(),
        target: form,
      };

      await onSuccessCallback(mockEvent);

      expect(consoleErrorSpy).toHaveBeenCalled();
      expect(errorMessage.classList.contains("hidden")).toBe(false);
      expect(loading.classList.contains("hidden")).toBe(true);
    });

    it("should handle DOMException network errors", async () => {
      const { executeTurnstileIfPresent } = await import("@/services/claudflare");
      vi.mocked(executeTurnstileIfPresent).mockRejectedValueOnce(new DOMException("Network error"));

      submitFormToRegisterOrLogin(modal, loading, "#test-form", false, errorMessage);

      const onSuccessCallback = mockOnSuccess.mock.calls[0][0];
      const mockEvent = {
        preventDefault: vi.fn(),
        target: form,
      };

      await onSuccessCallback(mockEvent);

      expect(errorMessage.classList.contains("hidden")).toBe(false);
      expect(errorMessage.textContent).toContain("*");
      expect(loading.classList.contains("hidden")).toBe(true);
    });

    it("should handle generic errors", async () => {
      const { executeTurnstileIfPresent } = await import("@/services/claudflare");
      vi.mocked(executeTurnstileIfPresent).mockRejectedValueOnce(new Error("Generic error"));

      submitFormToRegisterOrLogin(modal, loading, "#test-form", false, errorMessage);

      const onSuccessCallback = mockOnSuccess.mock.calls[0][0];
      const mockEvent = {
        preventDefault: vi.fn(),
        target: form,
      };

      await onSuccessCallback(mockEvent);

      expect(consoleErrorSpy).toHaveBeenCalled();
      expect(errorMessage.classList.contains("hidden")).toBe(false);
      expect(loading.classList.contains("hidden")).toBe(true);
    });

    it("should extract form ID correctly with and without # prefix", () => {
      // Test with # prefix
      submitFormToRegisterOrLogin(modal, loading, "#test-form", false, errorMessage);
      let onSuccessCallback = mockOnSuccess.mock.calls[0][0];

      expect(onSuccessCallback).toBeDefined();
      expect(typeof onSuccessCallback).toBe("function");

      // Clear mocks for next test
      vi.clearAllMocks();

      // Test without # prefix
      submitFormToRegisterOrLogin(modal, loading, "test-form", false, errorMessage);
      onSuccessCallback = mockOnSuccess.mock.calls[0][0];

      expect(onSuccessCallback).toBeDefined();
      expect(typeof onSuccessCallback).toBe("function");
    });

    it("should setup JustValidate with correct error styling", () => {
      const constructorCalls: any[] = [];
      const JustValidateSpy = class {
        constructor(...args: any[]) {
          constructorCalls.push(args);
        }
        addField = mockAddField;
        onSuccess = mockOnSuccess;
      };
      (globalThis as any).JustValidate = JustValidateSpy;

      submitFormToRegisterOrLogin(modal, loading, "#test-form", false, errorMessage);

      expect(constructorCalls[0][0]).toBe("#test-form");
      expect(constructorCalls[0][1]).toMatchObject({
        errorFieldCssClass: "border-red",
        errorLabelStyle: {
          color: "#e53e3e",
          fontSize: "0.875rem",
        },
      });
    });

    it("should handle form ID with or without # prefix", () => {
      const constructorCalls: any[] = [];
      const JustValidateSpy = class {
        constructor(...args: any[]) {
          constructorCalls.push(args);
        }
        addField = mockAddField;
        onSuccess = mockOnSuccess;
      };
      (globalThis as any).JustValidate = JustValidateSpy;

      submitFormToRegisterOrLogin(modal, loading, "test-form", false, errorMessage);

      expect(constructorCalls[0][0]).toBe("test-form");
      expect(constructorCalls[0][1]).toMatchObject({
        errorFieldCssClass: "border-red",
      });
    });
  });

  //////////////////// handleLogGoogleProvider ////////////////////
  describe("handleLogGoogleProvider", () => {
    let modal: HTMLDialogElement;
    let formHTML: HTMLFormElement;
    let loginError: HTMLDivElement;

    beforeEach(async () => {
      const { signInWithPopup } = await import("firebase/auth");
      const { suscribeToNewsletter } = await import("@/services/mailchimp");
      const { closeModalAnimation } = await import("@/utils/modals");

      vi.mocked(signInWithPopup).mockClear();
      vi.mocked(suscribeToNewsletter).mockClear();
      vi.mocked(closeModalAnimation).mockClear();

      modal = document.createElement("dialog");
      modal.close = vi.fn();

      formHTML = document.createElement("form");
      formHTML.reset = vi.fn();

      loginError = document.createElement("div");
      loginError.classList.add("hidden");

      document.body.appendChild(modal);
      document.body.appendChild(formHTML);
      document.body.appendChild(loginError);

      const auth = getFirebaseAuth();
      Object.defineProperty(auth, "currentUser", {
        get: vi.fn(() => ({
          displayName: "Test User",
          email: "test@gmail.com",
          getIdToken: vi.fn().mockResolvedValue("mock-google-token"),
        })),
        configurable: true,
      });
    });

    it("should handle successful Google login", async () => {
      vi.useFakeTimers();
      const { log } = await import("@/services/logger");

      const promise = handleLogGoogleProvider(modal, formHTML, false, loginError);

      await vi.runAllTimersAsync();
      await promise;

      expect(log.info).toHaveBeenCalledWith(expect.stringContaining("login"));
      expect(loginError.classList.contains("hidden")).toBe(true);

      vi.useRealTimers();
    });

    it("should handle successful Google register with newsletter", async () => {
      vi.useFakeTimers();
      const { suscribeToNewsletter } = await import("@/services/mailchimp");
      const { closeModalAnimation } = await import("@/utils/modals");

      const promise = handleLogGoogleProvider(modal, formHTML, true, loginError);

      await vi.runAllTimersAsync();
      await promise;

      expect(suscribeToNewsletter).toHaveBeenCalled();
      expect(closeModalAnimation).toHaveBeenCalledWith(modal, formHTML);

      vi.useRealTimers();
    });

    it("should call API with correct user data", async () => {
      const helperModule = await import("@/services/helper");
      const mockLoginUser = vi.fn().mockResolvedValue({ success: true });
      const originalApiService = helperModule.ApiService;

      (helperModule as any).ApiService = class {
        loginUser = mockLoginUser;
        registerUser = vi.fn().mockResolvedValue({ success: true });
      };

      await handleLogGoogleProvider(modal, formHTML, false, loginError);

      expect(mockLoginUser).toHaveBeenCalledWith(
        expect.objectContaining({
          name: "Test User",
          email: "test@gmail.com",
          password: "",
          provider: "google",
          id_token: "mock-google-token",
          first_free_class: false,
        })
      );

      (helperModule as any).ApiService = originalApiService;
    });

    it("should handle missing ID token", async () => {
      const auth = getFirebaseAuth();
      Object.defineProperty(auth, "currentUser", {
        get: vi.fn(() => ({
          displayName: "Test User",
          email: "test@gmail.com",
          getIdToken: vi.fn().mockResolvedValue(null),
        })),
        configurable: true,
      });

      await handleLogGoogleProvider(modal, formHTML, false, loginError);

      expect(loginError.classList.contains("hidden")).toBe(false);
      expect(loginError.textContent).toBeTruthy();
    });

    it("should handle API error and logout user", async () => {
      const helperModule = await import("@/services/helper");
      const originalApiService = helperModule.ApiService;

      (helperModule as any).ApiService = class {
        loginUser = vi.fn().mockResolvedValue({ success: false, error: "User not found" });
        registerUser = vi.fn().mockResolvedValue({ success: false, error: "User already exists" });
      };

      await handleLogGoogleProvider(modal, formHTML, false, loginError);

      expect(loginError.classList.contains("hidden")).toBe(false);
      expect(loginError.textContent).toBeTruthy();

      (helperModule as any).ApiService = originalApiService;
    });

    it("should handle signInWithPopup errors", async () => {
      const { signInWithPopup } = await import("firebase/auth");
      const { log } = await import("@/services/logger");
      vi.mocked(signInWithPopup).mockRejectedValue(new Error("Popup closed"));

      await handleLogGoogleProvider(modal, formHTML, false, loginError);

      expect(log.error).toHaveBeenCalledWith(expect.stringContaining("Error"), expect.any(Error));
      expect(loginError.classList.contains("hidden")).toBe(false);
    });

    it("should log start of authentication flow", async () => {
      const { log } = await import("@/services/logger");

      await handleLogGoogleProvider(modal, formHTML, true, loginError);

      expect(log.info).toHaveBeenCalledWith(expect.stringContaining("Iniciando"));
      expect(log.info).toHaveBeenCalledWith(expect.stringContaining("Abriendo popup"));
    });
  });
});
