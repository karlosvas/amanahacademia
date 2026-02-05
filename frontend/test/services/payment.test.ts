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
  showError,
  successPayment,
  clearMessages,
  initializePrice,
  initializeStripe,
} from "@/services/payment";
import { ApiService } from "@/services/helper";
import { FrontendStripe, getErrorFrontStripe } from "@/enums/enums";
import { getPrice } from "@/services/calendar";

// Mocks
vi.mock("@/services/helper");
vi.mock("@/services/calendar", () => ({
  getPrice: vi.fn(),
}));
vi.mock("@/services/logger", () => ({
  log: {
    debug: vi.fn(),
    info: vi.fn(),
    warn: vi.fn(),
    error: vi.fn(),
  },
}));

describe("payment.ts", () => {
  let mockApiService: any;
  let mockStripe: any;
  let mockElements: any;
  let mockPaymentElement: any;
  let mockedFetch: Mock;
  let consoleErrorSpy: any;

  beforeEach(() => {
    vi.clearAllMocks();
    consoleErrorSpy = vi.spyOn(console, "error").mockImplementation(() => {});

    // Override global mock and use real getElementById
    Object.defineProperty(document, "getElementById", {
      writable: true,
      value: Document.prototype.getElementById.bind(document),
    });

    // Setup DOM elements with proper structure
    const errorDiv = document.createElement("div");
    errorDiv.id = "error-message";
    errorDiv.textContent = "";
    errorDiv.style.display = "none";

    const successDiv = document.createElement("div");
    successDiv.id = "success-message";
    successDiv.textContent = "";
    successDiv.style.display = "none";

    const submitButton = document.createElement("button");
    submitButton.id = "submit-button";
    submitButton.textContent = "Submit";

    const buttonText = document.createElement("span");
    buttonText.id = "button-text";
    buttonText.textContent = "Pay";

    const pricingDiv = document.createElement("div");
    pricingDiv.id = "pricing";
    pricingDiv.textContent = "";

    const paymentElement = document.createElement("div");
    paymentElement.id = "payment-element";

    const loadingDiv = document.createElement("div");
    loadingDiv.className = "loading";
    loadingDiv.style.display = "block";

    document.body.innerHTML = "";
    document.body.appendChild(submitButton);
    document.body.appendChild(buttonText);
    document.body.appendChild(errorDiv);
    document.body.appendChild(successDiv);
    document.body.appendChild(pricingDiv);
    document.body.appendChild(paymentElement);
    document.body.appendChild(loadingDiv);

    // Mock ApiService
    mockApiService = {
      getBookingById: vi.fn(),
      createBooking: vi.fn(),
      confirmBooking: vi.fn(),
      saveCalStripeConnection: vi.fn(),
      checkout: vi.fn(),
    };

    vi.mocked(ApiService).mockImplementation(function () {
      return mockApiService;
    });

    // Mock Stripe
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

    // Mock globalThis and globalThis properly
    if (typeof globalThis !== "undefined") {
      (globalThis as any).Stripe = vi.fn().mockReturnValue(mockStripe);
      (globalThis as any).gtag = vi.fn();
    }

    Object.defineProperty(globalThis, "location", {
      value: {
        href: "",
        origin: "http://localhost:3000",
      },
      writable: true,
      configurable: true,
    });

    // Mock fetch
    globalThis.fetch = vi.fn() as any;
    mockedFetch = globalThis.fetch as unknown as Mock;

    // Mock setTimeout
    vi.spyOn(globalThis, "setTimeout").mockImplementation((fn: any) => {
      fn();
      return 0 as any;
    });
  });

  afterEach(() => {
    consoleErrorSpy.mockRestore();
    vi.restoreAllMocks();
  });

  //////////////////// showError ////////////////////
  describe("showError", () => {
    it("should display error message in error div", () => {
      const errorMessage = "Test error message";

      showError(errorMessage);

      const errorDiv = document.getElementById("error-message")!;
      expect(errorDiv.textContent).toBe(errorMessage);
      expect(errorDiv.style.display).toBe("block");
    });

    it("should clear success message when showing error", () => {
      const successDiv = document.getElementById("success-message")!;
      successDiv.textContent = "Success!";

      showError("Error occurred");

      const updatedSuccessDiv = document.getElementById("success-message")!;
      expect(updatedSuccessDiv.textContent).toBe("");
    });

    it("should handle missing error div gracefully", () => {
      document.getElementById("error-message")?.remove();

      expect(() => showError("Error")).not.toThrow();
    });
  });

  //////////////////// clearMessages ////////////////////
  describe("clearMessages", () => {
    it("should clear and hide both error and success messages", () => {
      const errorDiv = document.getElementById("error-message")!;
      const successDiv = document.getElementById("success-message")!;

      errorDiv.textContent = "Error!";
      errorDiv.style.display = "block";
      successDiv.textContent = "Success!";
      successDiv.style.display = "block";

      clearMessages();

      const updatedErrorDiv = document.getElementById("error-message")!;
      const updatedSuccessDiv = document.getElementById("success-message")!;
      expect(updatedErrorDiv.textContent).toBe("");
      expect(updatedErrorDiv.style.display).toBe("none");
      expect(updatedSuccessDiv.textContent).toBe("");
      expect(updatedSuccessDiv.style.display).toBe("none");
    });

    it("should handle missing divs gracefully", () => {
      document.getElementById("error-message")?.remove();
      document.getElementById("success-message")?.remove();

      expect(() => clearMessages()).not.toThrow();
    });
  });

  //////////////////// successPayment ////////////////////
  describe("successPayment", () => {
    it("should handle group-class by adding attendee and creating booking", async () => {
      const mockBooking = {
        success: true,
        data: {
          attendees: [{ name: "User 1", email: "user1@test.com" }],
          startTime: "2024-01-01T10:00:00Z",
          endTime: "2024-01-01T11:00:00Z",
        },
      };

      mockApiService.getBookingById.mockResolvedValue(mockBooking);
      mockApiService.createBooking.mockResolvedValue({ success: true });
      mockApiService.saveCalStripeConnection.mockResolvedValue({
        success: true,
      });

      const paymentIntent = { id: "pi_123" };

      await successPayment(
        mockApiService,
        paymentIntent,
        "booking-123",
        "accepted",
        "group-class",
        "new@test.com",
      );

      expect(mockApiService.getBookingById).toHaveBeenCalledWith("booking-123");
      expect(mockApiService.createBooking).toHaveBeenCalledWith({
        ...mockBooking.data,
        attendees: [
          { name: "User 1", email: "user1@test.com" },
          { name: "", email: "new@test.com" },
        ],
        startTime: "2024-01-01T10:00:00Z",
        endTime: "2024-01-01T11:00:00Z",
      });
    });

    it("should throw error if booking fetch fails for group-class", async () => {
      mockApiService.getBookingById.mockResolvedValue({ success: false });

      const paymentIntent = { id: "pi_123" };

      await expect(
        successPayment(
          mockApiService,
          paymentIntent,
          "booking-123",
          "accepted",
          "group-class",
          "new@test.com",
        ),
      ).rejects.toThrow("Error to get booking");
    });

    it("should throw error if booking creation fails for group-class", async () => {
      const mockBooking = {
        success: true,
        data: {
          attendees: [],
          startTime: "2024-01-01T10:00:00Z",
          endTime: "2024-01-01T11:00:00Z",
        },
      };

      mockApiService.getBookingById.mockResolvedValue(mockBooking);
      mockApiService.createBooking.mockResolvedValue({ success: false });

      const paymentIntent = { id: "pi_123" };

      await expect(
        successPayment(
          mockApiService,
          paymentIntent,
          "booking-123",
          "accepted",
          "group-class",
          "new@test.com",
        ),
      ).rejects.toThrow("Error to update booking");
    });

    it("should confirm booking if status is not accepted", async () => {
      mockApiService.confirmBooking.mockResolvedValue({ success: true });
      mockApiService.saveCalStripeConnection.mockResolvedValue({
        success: true,
      });

      const paymentIntent = { id: "pi_123" };

      await successPayment(
        mockApiService,
        paymentIntent,
        "booking-123",
        "pending",
        "individual-class",
        "test@test.com",
      );

      expect(mockApiService.confirmBooking).toHaveBeenCalledWith("booking-123");
    });

    it("should not confirm booking if status is accepted", async () => {
      mockApiService.saveCalStripeConnection.mockResolvedValue({
        success: true,
      });

      const paymentIntent = { id: "pi_123" };

      await successPayment(
        mockApiService,
        paymentIntent,
        "booking-123",
        "accepted",
        "individual-class",
        "test@test.com",
      );

      expect(mockApiService.confirmBooking).not.toHaveBeenCalled();
    });

    it("should throw error if booking confirmation fails", async () => {
      mockApiService.confirmBooking.mockResolvedValue({ success: false });

      const paymentIntent = { id: "pi_123" };

      await expect(
        successPayment(
          mockApiService,
          paymentIntent,
          "booking-123",
          "pending",
          "individual-class",
          "test@test.com",
        ),
      ).rejects.toThrow("Error to confirm booking");
    });

    it("should save cal-stripe connection", async () => {
      mockApiService.saveCalStripeConnection.mockResolvedValue({
        success: true,
      });

      const paymentIntent = { id: "pi_123" };

      await successPayment(
        mockApiService,
        paymentIntent,
        "booking-123",
        "accepted",
        "individual-class",
        "test@test.com",
      );

      expect(mockApiService.saveCalStripeConnection).toHaveBeenCalledWith({
        cal_id: "booking-123",
        stripe_id: "pi_123",
      });
    });

    it("should throw error if saving connection fails", async () => {
      mockApiService.saveCalStripeConnection.mockResolvedValue({
        success: false,
      });

      const paymentIntent = { id: "pi_123" };

      await expect(
        successPayment(
          mockApiService,
          paymentIntent,
          "booking-123",
          "accepted",
          "individual-class",
          "test@test.com",
        ),
      ).rejects.toThrow("Error to save relation");
    });

    it("should clear error message on success", async () => {
      mockApiService.saveCalStripeConnection.mockResolvedValue({
        success: true,
      });

      const errorDiv = document.getElementById("error-message")!;
      errorDiv.textContent = "Some error";

      const paymentIntent = { id: "pi_123" };

      await successPayment(
        mockApiService,
        paymentIntent,
        "booking-123",
        "accepted",
        "individual-class",
        "test@test.com",
      );

      const updatedErrorDiv = document.getElementById("error-message")!;
      expect(updatedErrorDiv.textContent).toBe("");
    });

    it("should send gtag event if gtag is available", async () => {
      mockApiService.saveCalStripeConnection.mockResolvedValue({
        success: true,
      });

      const paymentIntent = { id: "pi_123" };

      await successPayment(
        mockApiService,
        paymentIntent,
        "booking-123",
        "accepted",
        "individual-class",
        "test@test.com",
      );

      if (typeof globalThis !== "undefined" && (globalThis as any).gtag) {
        expect((globalThis as any).gtag).toHaveBeenCalledWith(
          "event",
          "class_booking",
          { bookingUid: "booking-123" },
        );
      }
    });

    it("should redirect to success page after 2 seconds", async () => {
      mockApiService.saveCalStripeConnection.mockResolvedValue({
        success: true,
      });

      const paymentIntent = { id: "pi_123" };

      await successPayment(
        mockApiService,
        paymentIntent,
        "booking-123",
        "accepted",
        "individual-class",
        "test@test.com",
      );

      expect(setTimeout).toHaveBeenCalledWith(expect.any(Function), 2000);
      expect(globalThis.location.href).toBe("/payments/payment-success");
    });
  });

  //////////////////// initializePrice ////////////////////
  describe("initializePrice", () => {
    beforeEach(() => {
      vi.mocked(getPrice).mockReturnValue(50);
    });

    it("should fetch pricing without test country", async () => {
      mockedFetch.mockResolvedValue({
        ok: true,
        json: vi.fn().mockResolvedValue({ country: "ES", pricing: {} }),
      } as any);

      await initializePrice(null, "individual-class");

      expect(mockedFetch).toHaveBeenCalledWith("/api/pricing");
    });

    it("should fetch pricing with test country", async () => {
      mockedFetch.mockResolvedValue({
        ok: true,
        json: vi.fn().mockResolvedValue({ country: "US", pricing: {} }),
      } as any);

      await initializePrice("US", "individual-class");

      expect(mockedFetch).toHaveBeenCalledWith("/api/pricing?test_country=US");
    });

    it("should show error if slugType is missing", async () => {
      mockedFetch.mockResolvedValue({
        ok: true,
        json: vi.fn().mockResolvedValue({ country: "ES", pricing: {} }),
      } as any);

      const result = await initializePrice(null, "");

      expect(result).toBeUndefined();
      const errorDiv = document.getElementById("error-message");
      expect(errorDiv?.textContent).toBe(
        getErrorFrontStripe(FrontendStripe.MISSING_SLUG),
      );
    });

    it("should show error if fetch fails", async () => {
      mockedFetch.mockResolvedValue({
        ok: false,
        json: vi.fn().mockResolvedValue({}),
      } as any);

      const result = await initializePrice(null, "individual-class");

      expect(result).toBeUndefined();
      const errorDiv = document.getElementById("error-message");
      expect(errorDiv?.textContent).toBe(
        getErrorFrontStripe(FrontendStripe.PRICING_FETCH_ERROR),
      );
    });

    it("should show error if pricing element not found", async () => {
      document.getElementById("pricing")?.remove();

      mockedFetch.mockResolvedValue({
        ok: true,
        json: vi.fn().mockResolvedValue({ country: "ES", pricing: {} }),
      } as any);

      const result = await initializePrice(null, "individual-class");

      expect(result).toBeUndefined();
      const errorDiv = document.getElementById("error-message");
      expect(errorDiv?.textContent).toBe(
        getErrorFrontStripe(FrontendStripe.PRICING_ELEMENT_NOT_FOUND),
      );
    });

    it("should set pricing element text and return pricing", async () => {
      mockedFetch.mockResolvedValue({
        ok: true,
        json: vi.fn().mockResolvedValue({ country: "ES", pricing: {} }),
      } as any);

      const result = await initializePrice(null, "individual-class");

      const pricingElement = document.getElementById("pricing");
      expect(pricingElement?.textContent).toBe("50 €");
      expect(result).toBe(50);
    });

    it("should handle fetch exception", async () => {
      const result = await initializePrice(null, "individual-class");

      expect(result).toBeUndefined();
      const errorDiv = document.getElementById("error-message");
      expect(errorDiv?.textContent).toBe(
        getErrorFrontStripe(FrontendStripe.GENERIC_ERROR),
      );
    });
  });

  //////////////////// initializeStripe ////////////////////
  describe("initializeStripe", () => {
    const STRIPE_PUBLIC_KEY = "pk_test_123";
    const pricing = 50;

    it("should test that ApiService mock works", () => {
      // First verify the mock is configured correctly
      const instance = new ApiService();
      expect(instance).toBe(mockApiService);
      expect(instance.checkout).toBeDefined();
    });

    it("should initialize stripe and elements successfully", async () => {
      mockApiService.checkout.mockResolvedValue({
        success: true,
        data: { client_secret: "cs_test_123" },
      });

      const result = await initializeStripe(STRIPE_PUBLIC_KEY, pricing);

      if (typeof globalThis !== "undefined") {
        expect((globalThis as any).Stripe).toHaveBeenCalledWith(
          STRIPE_PUBLIC_KEY,
        );
      }
      expect(mockApiService.checkout).toHaveBeenCalledWith({
        amount: 5000,
        currency: "EUR",
      });
      expect(result).toEqual({ stripe: mockStripe, elements: mockElements });
    });

    it("should convert euros to cents correctly", async () => {
      mockApiService.checkout.mockResolvedValue({
        success: true,
        data: { client_secret: "cs_test_123" },
      });

      await initializeStripe(STRIPE_PUBLIC_KEY, 25.5);

      expect(mockApiService.checkout).toHaveBeenCalledWith({
        amount: 2550,
        currency: "EUR",
      });
    });

    it("should create elements with correct appearance config", async () => {
      mockApiService.checkout.mockResolvedValue({
        success: true,
        data: { client_secret: "cs_test_123" },
      });

      await initializeStripe(STRIPE_PUBLIC_KEY, pricing);

      expect(mockStripe.elements).toHaveBeenCalledWith({
        clientSecret: "cs_test_123",
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
      mockApiService.checkout.mockResolvedValue({
        success: true,
        data: { client_secret: "cs_test_123" },
      });

      await initializeStripe(STRIPE_PUBLIC_KEY, pricing);

      expect(mockElements.create).toHaveBeenCalledWith("payment");
      expect(mockPaymentElement.mount).toHaveBeenCalledWith("#payment-element");
    });

    it("should hide loading spinner", async () => {
      mockApiService.checkout.mockResolvedValue({
        success: true,
        data: { client_secret: "cs_test_123" },
      });

      const loading = document.querySelector(".loading") as HTMLDivElement;

      await initializeStripe(STRIPE_PUBLIC_KEY, pricing);

      expect(loading.style.display).toBe("none");
    });

    it("should enable submit button", async () => {
      mockApiService.checkout.mockResolvedValue({
        success: true,
        data: { client_secret: "cs_test_123" },
      });

      const submitButton = document.getElementById(
        "submit-button",
      ) as HTMLButtonElement;
      submitButton.disabled = true;

      await initializeStripe(STRIPE_PUBLIC_KEY, pricing);

      const updatedSubmitButton = document.getElementById(
        "submit-button",
      ) as HTMLButtonElement;
      expect(updatedSubmitButton.disabled).toBe(false);
    });

    it("should handle payment element change events", async () => {
      mockApiService.checkout.mockResolvedValue({
        success: true,
        data: { client_secret: "cs_test_123" },
      });

      await initializeStripe(STRIPE_PUBLIC_KEY, pricing);

      expect(mockPaymentElement.on).toHaveBeenCalledWith(
        "change",
        expect.any(Function),
      );

      const changeHandler = mockPaymentElement.on.mock.calls[0][1];

      // Test error scenario
      changeHandler({ error: { message: "Card error" } });
      let errorDiv = document.getElementById("error-message");
      expect(errorDiv?.textContent).toBe("Card error");

      // Test success scenario
      changeHandler({ error: null });
      errorDiv = document.getElementById("error-message");
      expect(errorDiv?.textContent).toBe("");
    });

    it("should return null if loading element is missing", async () => {
      mockApiService.checkout.mockResolvedValue({
        success: true,
        data: { client_secret: "cs_test_123" },
      });

      const loading = document.querySelector(".loading");
      loading?.remove();

      const result = await initializeStripe(STRIPE_PUBLIC_KEY, pricing);

      expect(result).toBeNull();
    });

    it("should return null if loading element has no style property", async () => {
      mockApiService.checkout.mockResolvedValue({
        success: true,
        data: { client_secret: "cs_test_123" },
      });

      // Crear un objeto que simule un elemento sin style
      const fakeLoading = { style: undefined };

      const originalQuerySelector = document.querySelector.bind(document);
      vi.spyOn(document, "querySelector").mockImplementation(
        (selector: string) => {
          if (selector === ".loading") return fakeLoading as any;
          return originalQuerySelector(selector);
        },
      );

      const result = await initializeStripe(STRIPE_PUBLIC_KEY, pricing);

      expect(result).toBeNull();
    });

    it("should return null if submit button not found", async () => {
      mockApiService.checkout.mockResolvedValue({
        success: true,
        data: { client_secret: "cs_test_123" },
      });

      document.getElementById("submit-button")?.remove();

      const result = await initializeStripe(STRIPE_PUBLIC_KEY, pricing);

      expect(result).toBeNull();
    });

    it("should show error if checkout fails", async () => {
      mockApiService.checkout.mockResolvedValue({
        success: false,
      });

      const result = await initializeStripe(STRIPE_PUBLIC_KEY, pricing);

      expect(result).toBeNull();
    });

    it("should handle exception during initialization", async () => {
      mockApiService.checkout.mockRejectedValue(new Error("Network error"));

      const result = await initializeStripe(STRIPE_PUBLIC_KEY, pricing);

      expect(result).toBeNull();
    });
  });
});
