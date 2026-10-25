import {
  describe,
  it,
  expect,
  beforeEach,
  afterEach,
  vi,
  type Mock,
} from "vitest";
import {
  buildCheckoutRequest,
  clearMessages,
  createCheckout,
  handlePayment,
  initializeStripe,
  showError,
} from "@/services/payment";
import { FrontendStripe, getErrorFrontStripe } from "@/enums/enums";

vi.mock("@/services/logger", () => ({
  log: {
    debug: vi.fn(),
    info: vi.fn(),
    warn: vi.fn(),
    error: vi.fn(),
  },
}));

describe("payment.ts", () => {
  let mockStripe: any;
  let mockElements: any;
  let mockPaymentElement: any;
  let mockedFetch: Mock;
  let consoleErrorSpy: any;

  beforeEach(() => {
    vi.clearAllMocks();
    consoleErrorSpy = vi.spyOn(console, "error").mockImplementation(() => {});

    Object.defineProperty(document, "getElementById", {
      writable: true,
      value: Document.prototype.getElementById.bind(document),
    });

    const errorDiv = document.createElement("div");
    errorDiv.id = "error-message";
    errorDiv.textContent = "";
    errorDiv.style.display = "none";

    const submitButton = document.createElement("button");
    submitButton.id = "submit-button";
    submitButton.textContent = "Submit";

    const buttonText = document.createElement("span");
    buttonText.id = "button-text";
    buttonText.textContent = "Pay";

    const paymentElement = document.createElement("div");
    paymentElement.id = "payment-element";

    const loadingDiv = document.createElement("div");
    loadingDiv.className = "loading";
    loadingDiv.style.display = "block";

    document.body.innerHTML = "";
    document.body.appendChild(submitButton);
    document.body.appendChild(buttonText);
    document.body.appendChild(errorDiv);
    document.body.appendChild(paymentElement);
    document.body.appendChild(loadingDiv);

    mockPaymentElement = {
      mount: vi.fn().mockResolvedValue(undefined),
      on: vi.fn(),
    };

    mockElements = {
      create: vi.fn().mockReturnValue(mockPaymentElement),
    };

    mockStripe = {
      confirmPayment: vi.fn(),
      elements: vi.fn().mockReturnValue(mockElements),
    };

    (globalThis as any).Stripe = vi.fn().mockReturnValue(mockStripe);
    (globalThis as any).gtag = vi.fn();

    Object.defineProperty(globalThis, "location", {
      value: {
        href: "",
        origin: "http://localhost:3000",
      },
      writable: true,
      configurable: true,
    });

    globalThis.fetch = vi.fn() as any;
    mockedFetch = globalThis.fetch as unknown as Mock;
  });

  afterEach(() => {
    consoleErrorSpy.mockRestore();
    vi.restoreAllMocks();
  });

  const errorText = () => document.getElementById("error-message")?.textContent;

  //////////////////// showError ////////////////////
  describe("showError", () => {
    it("should display error message in error div", () => {
      showError("Test error message");

      const errorDiv = document.getElementById("error-message")!;
      expect(errorDiv.textContent).toBe("Test error message");
      expect(errorDiv.style.display).toBe("block");
    });

    it("should handle missing error div gracefully", () => {
      document.getElementById("error-message")?.remove();

      expect(() => showError("Error")).not.toThrow();
    });
  });

  //////////////////// clearMessages ////////////////////
  describe("clearMessages", () => {
    it("should clear and hide the error message", () => {
      const errorDiv = document.getElementById("error-message")!;
      errorDiv.textContent = "Error!";
      errorDiv.style.display = "block";

      clearMessages();

      expect(errorDiv.textContent).toBe("");
      expect(errorDiv.style.display).toBe("none");
    });

    it("should handle missing divs gracefully", () => {
      document.getElementById("error-message")?.remove();

      expect(() => clearMessages()).not.toThrow();
    });
  });

  //////////////////// buildCheckoutRequest ////////////////////
  describe("buildCheckoutRequest", () => {
    it("should use the booking uid when present", () => {
      const params = new URLSearchParams(
        "uid=booking-1&type=group-class&status=pending&weeks=4",
      );

      expect(buildCheckoutRequest(params)).toEqual({
        booking_uid: "booking-1",
      });
    });

    it("should accept the bookingUid parameter sent by Cal.com", () => {
      const params = new URLSearchParams("bookingUid=booking-2&uid=booking-2");

      expect(buildCheckoutRequest(params)).toEqual({
        booking_uid: "booking-2",
      });
    });

    it("should build the weeks flow request", () => {
      const params = new URLSearchParams("type=group-class&weeks=4");

      expect(buildCheckoutRequest(params)).toEqual({
        class_type: "group-class",
        weeks: 4,
      });
    });

    it("should reject invalid weeks", () => {
      for (const weeks of ["0", "13", "abc", "2.5", ""]) {
        const params = new URLSearchParams(`type=group-class&weeks=${weeks}`);
        expect(buildCheckoutRequest(params)).toBeNull();
      }
    });

    it("should reject missing parameters", () => {
      expect(buildCheckoutRequest(new URLSearchParams())).toBeNull();
      expect(buildCheckoutRequest(new URLSearchParams("weeks=4"))).toBeNull();
    });
  });

  //////////////////// createCheckout ////////////////////
  describe("createCheckout", () => {
    const checkoutData = {
      client_secret: "pi_123_secret_456",
      amount: 3000,
      currency: "eur",
    };

    it("should send the request to the server checkout endpoint", async () => {
      mockedFetch.mockResolvedValue({
        ok: true,
        status: 200,
        json: async () => ({ success: true, data: checkoutData }),
      });

      const result = await createCheckout(
        "token-123",
        { booking_uid: "booking-1" },
        null,
      );

      expect(result).toEqual(checkoutData);
      expect(mockedFetch).toHaveBeenCalledWith("/api/checkout", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer token-123",
        },
        body: JSON.stringify({ booking_uid: "booking-1" }),
      });
    });

    it("should forward the test country", async () => {
      mockedFetch.mockResolvedValue({
        ok: true,
        status: 200,
        json: async () => ({ success: true, data: checkoutData }),
      });

      await createCheckout("token-123", { booking_uid: "booking-1" }, "MA");

      expect(mockedFetch.mock.calls[0][0]).toBe(
        "/api/checkout?test_country=MA",
      );
    });

    it("should show a specific error when the booking cannot be paid", async () => {
      mockedFetch.mockResolvedValue({
        ok: false,
        status: 409,
        json: async () => ({ success: false, error: "already paid" }),
      });

      const result = await createCheckout(
        "token-123",
        { booking_uid: "booking-1" },
        null,
      );

      expect(result).toBeNull();
      expect(errorText()).toBe(
        getErrorFrontStripe(FrontendStripe.BOOKING_NOT_PAYABLE),
      );
    });

    it("should show a configuration error when payments are unavailable", async () => {
      mockedFetch.mockResolvedValue({
        ok: false,
        status: 503,
        json: async () => {
          throw new Error("empty body");
        },
      });

      const result = await createCheckout(
        "token-123",
        { booking_uid: "booking-1" },
        null,
      );

      expect(result).toBeNull();
      expect(errorText()).toBe(
        getErrorFrontStripe(FrontendStripe.MISSING_CONFIG),
      );
    });

    it("should show a connection error when the request fails", async () => {
      mockedFetch.mockRejectedValue(new Error("Network error"));

      const result = await createCheckout(
        "token-123",
        { booking_uid: "booking-1" },
        null,
      );

      expect(result).toBeNull();
      expect(errorText()).toBe(
        getErrorFrontStripe(FrontendStripe.CONNECTION_ERROR),
      );
    });
  });

  //////////////////// handlePayment ////////////////////
  describe("handlePayment", () => {
    it("should show an error if stripe is not initialized", async () => {
      await handlePayment(null, null, "secret");

      expect(errorText()).toBe(
        getErrorFrontStripe(FrontendStripe.STRIPE_NOT_INITIALIZED),
      );
    });

    it("should show card errors and re-enable the button", async () => {
      mockStripe.confirmPayment.mockResolvedValue({
        error: { type: "card_error", message: "Tarjeta rechazada" },
      });

      await handlePayment(mockStripe, mockElements, "secret");

      const submitButton = document.getElementById(
        "submit-button",
      ) as HTMLButtonElement;
      expect(errorText()).toBe("Tarjeta rechazada");
      expect(submitButton.disabled).toBe(false);
      expect(document.getElementById("button-text")?.textContent).toBe("Pay");
      expect(globalThis.location.href).toBe("");
    });

    it("should show a generic error for unexpected statuses", async () => {
      mockStripe.confirmPayment.mockResolvedValue({
        paymentIntent: { id: "pi_123", status: "requires_payment_method" },
      });

      await handlePayment(mockStripe, mockElements, "secret");

      expect(errorText()).toBe(
        getErrorFrontStripe(FrontendStripe.UNKNOWN_PAYMENT_STATUS),
      );
      expect(globalThis.location.href).toBe("");
    });

    it("should redirect to the success page after a successful payment", async () => {
      mockStripe.confirmPayment.mockResolvedValue({
        paymentIntent: { id: "pi_123", status: "succeeded" },
      });

      await handlePayment(mockStripe, mockElements, "pi_123_secret_456");

      expect(mockStripe.confirmPayment).toHaveBeenCalledWith({
        elements: mockElements,
        confirmParams: {
          return_url: "http://localhost:3000/payments/payment-success",
        },
        redirect: "if_required",
      });
      expect((globalThis as any).gtag).toHaveBeenCalledWith(
        "event",
        "class_booking",
        { payment_intent: "pi_123" },
      );
      expect(globalThis.location.href).toBe(
        "/payments/payment-success?payment_intent_client_secret=pi_123_secret_456",
      );
    });
  });

  //////////////////// initializeStripe ////////////////////
  describe("initializeStripe", () => {
    const STRIPE_PUBLIC_KEY = "pk_test_123";
    const CLIENT_SECRET = "pi_123_secret_456";

    it("should initialize stripe and elements successfully", async () => {
      const result = await initializeStripe(STRIPE_PUBLIC_KEY, CLIENT_SECRET);

      expect((globalThis as any).Stripe).toHaveBeenCalledWith(
        STRIPE_PUBLIC_KEY,
      );
      expect(result).toEqual({ stripe: mockStripe, elements: mockElements });
    });

    it("should create elements with the client secret and appearance", async () => {
      await initializeStripe(STRIPE_PUBLIC_KEY, CLIENT_SECRET);

      expect(mockStripe.elements).toHaveBeenCalledWith({
        clientSecret: CLIENT_SECRET,
        appearance: expect.objectContaining({
          theme: "stripe",
          variables: expect.objectContaining({
            colorPrimary: "#eb5e61",
            colorBackground: "transparent",
            colorText: "#808080",
          }),
        }),
      });
    });

    it("should create and mount payment element", async () => {
      await initializeStripe(STRIPE_PUBLIC_KEY, CLIENT_SECRET);

      expect(mockElements.create).toHaveBeenCalledWith("payment");
      expect(mockPaymentElement.mount).toHaveBeenCalledWith("#payment-element");
    });

    it("should hide loading spinner", async () => {
      const loading = document.querySelector(".loading") as HTMLDivElement;

      await initializeStripe(STRIPE_PUBLIC_KEY, CLIENT_SECRET);

      expect(loading.style.display).toBe("none");
    });

    it("should enable submit button", async () => {
      const submitButton = document.getElementById(
        "submit-button",
      ) as HTMLButtonElement;
      submitButton.disabled = true;

      await initializeStripe(STRIPE_PUBLIC_KEY, CLIENT_SECRET);

      expect(submitButton.disabled).toBe(false);
    });

    it("should handle payment element change events", async () => {
      await initializeStripe(STRIPE_PUBLIC_KEY, CLIENT_SECRET);

      expect(mockPaymentElement.on).toHaveBeenCalledWith(
        "change",
        expect.any(Function),
      );

      const changeHandler = mockPaymentElement.on.mock.calls[0][1];

      changeHandler({ error: { message: "Card error" } });
      expect(errorText()).toBe("Card error");

      changeHandler({ error: null });
      expect(errorText()).toBe("");
    });

    it("should return null if submit button not found", async () => {
      document.getElementById("submit-button")?.remove();

      const result = await initializeStripe(STRIPE_PUBLIC_KEY, CLIENT_SECRET);

      expect(result).toBeNull();
    });

    it("should handle exception during initialization", async () => {
      (globalThis as any).Stripe = vi.fn(() => {
        throw new Error("Stripe failed");
      });

      const result = await initializeStripe(STRIPE_PUBLIC_KEY, CLIENT_SECRET);

      expect(result).toBeNull();
      expect(errorText()).toBe(
        getErrorFrontStripe(FrontendStripe.STRIPE_INITIALIZATION_ERROR),
      );
    });
  });
});
