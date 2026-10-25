import type {
  CheckoutPaymentIntentRequest,
  CheckoutPaymentIntentResponse,
  ResponseAPI,
} from "@/types/bakend-types";
import { getErrorFrontStripe, FrontendStripe } from "@/enums/enums";
import { log } from "./logger";

const MAX_WEEKS = 12;

export function buildCheckoutRequest(
  params: URLSearchParams,
): CheckoutPaymentIntentRequest | null {
  const bookingUid = params.get("bookingUid") || params.get("uid");
  if (bookingUid) return { booking_uid: bookingUid };

  const classType = params.get("type");
  const weeks = Number(params.get("weeks"));
  if (classType && Number.isInteger(weeks) && weeks >= 1 && weeks <= MAX_WEEKS)
    return { class_type: classType, weeks };

  return null;
}

function checkoutErrorCode(status: number): FrontendStripe {
  if (status === 409) return FrontendStripe.BOOKING_NOT_PAYABLE;
  if (status === 400 || status === 404) return FrontendStripe.MISSING_BOOKING;
  if (status === 503) return FrontendStripe.MISSING_CONFIG;
  return FrontendStripe.SERVER_ERROR;
}

export async function createCheckout(
  token: string,
  request: CheckoutPaymentIntentRequest,
  testCountry: string | null,
): Promise<CheckoutPaymentIntentResponse | null> {
  const url = testCountry
    ? `/api/checkout?test_country=${encodeURIComponent(testCountry)}`
    : "/api/checkout";

  try {
    const response = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(request),
    });

    const result = (await response
      .json()
      .catch(() => null)) as ResponseAPI<CheckoutPaymentIntentResponse> | null;

    if (!response.ok || !result?.success) {
      log.error("Checkout failed", response.status, result);
      showError(getErrorFrontStripe(checkoutErrorCode(response.status)));
      return null;
    }

    return result.data;
  } catch (error) {
    log.error("Error creating checkout", error);
    showError(getErrorFrontStripe(FrontendStripe.CONNECTION_ERROR));
    return null;
  }
}

export async function handlePayment(
  stripe: any,
  elements: any,
  clientSecret: string,
): Promise<void> {
  if (!stripe || !elements) {
    log.error("Stripe or Elements not initialized");
    showError(getErrorFrontStripe(FrontendStripe.STRIPE_NOT_INITIALIZED));
    return;
  }

  const submitButton = document.getElementById(
    "submit-button",
  ) as HTMLButtonElement | null;
  const buttonText = document.getElementById(
    "button-text",
  ) as HTMLSpanElement | null;

  if (!submitButton || !buttonText) {
    log.error("Submit button or button text elements not found in DOM");
    showError(getErrorFrontStripe(FrontendStripe.MISSING_ELEMENTS));
    return;
  }

  const originalText = buttonText.textContent;
  submitButton.disabled = true;
  buttonText.textContent = "Procesando...";
  clearMessages();

  const restoreButton = () => {
    submitButton.disabled = false;
    buttonText.textContent = originalText;
  };

  const { error, paymentIntent } = await stripe.confirmPayment({
    elements,
    confirmParams: {
      return_url: `${location.origin}/payments/payment-success`,
    },
    redirect: "if_required",
  });

  if (error) {
    log.error("Stripe payment error", error.type, error.message);
    showError(
      error.type === "card_error" || error.type === "validation_error"
        ? error.message
        : getErrorFrontStripe(FrontendStripe.UNKNOWN_PAYMENT_STATUS),
    );
    restoreButton();
    return;
  }

  if (
    !paymentIntent ||
    !["succeeded", "processing"].includes(paymentIntent.status)
  ) {
    log.error("Unexpected payment status", paymentIntent?.status);
    showError(getErrorFrontStripe(FrontendStripe.UNKNOWN_PAYMENT_STATUS));
    restoreButton();
    return;
  }

  if (globalThis.gtag)
    globalThis.gtag("event", "class_booking", {
      payment_intent: paymentIntent.id,
    });

  const params = new URLSearchParams({
    payment_intent_client_secret: clientSecret,
  });
  globalThis.location.href = `/payments/payment-success?${params}`;
}

export function showError(message: string) {
  const errorDiv = document.getElementById("error-message");
  if (errorDiv) {
    errorDiv.textContent = message;
    errorDiv.style.display = "block";
  }
}

export function clearMessages() {
  const errorDiv = document.getElementById("error-message");
  if (errorDiv) {
    errorDiv.textContent = "";
    errorDiv.style.display = "none";
  }
}

export async function initializeStripe(
  STRIPE_PUBLIC_KEY: string,
  clientSecret: string,
): Promise<{ stripe: any; elements: any } | null> {
  try {
    const stripe = globalThis.Stripe(STRIPE_PUBLIC_KEY);

    // Crear Stripe Elements
    const appearance = {
      theme: "stripe",
      variables: {
        // Colores principales
        colorPrimary: "#eb5e61", // Color principal botones
        colorBackground: "transparent", // Fondo general
        colorText: "#808080", // Texto principal (gris)
        colorTextSecondary: "#8a4141", // Texto secundario / placeholders
        colorDanger: "#fa8072", // Mensajes de error
        colorSuccess: "#28a745", // Mensajes correctos

        // Tipografía
        fontSizeBase: "16px",
        fontFamily: "Arial, sans-serif",

        // Bordes
        borderRadius: "8px",
      },
      rules: {
        ".Input": {
          padding: "10px",
          border: "1px solid #ddd",
        },
        ".Input:focus": {
          borderColor: "#6d0006",
        },
        ".Button": {
          backgroundColor: "#6d0006",
          color: "#fff",
        },
        ".Button:hover": {
          backgroundColor: "#eb5e61", // Un color bonito, el mismo que el primario
          color: "#fff", // Mantén el texto blanco
          boxShadow: "0 2px 8px rgba(235, 94, 97, 0.2)", // Sombra suave
        },
      },
    };
    const elements = stripe.elements({ clientSecret, appearance });

    // Crear y montar el Payment Element
    const paymentElement = elements.create("payment");

    const loading = document.querySelector(".loading") as HTMLDivElement | null;
    if (loading) loading.style.display = "none";
    await paymentElement.mount("#payment-element");

    // Habilitar el botón de pago
    const submitButton = document.getElementById(
      "submit-button",
    ) as HTMLButtonElement | null;
    if (!submitButton) return null;
    submitButton.disabled = false;

    // Manejar cambios en el elemento de pago
    paymentElement.on("change", (event: any) => {
      if (event.error) showError(event.error.message);
      else clearMessages();
    });

    return { stripe, elements };
  } catch (error: any) {
    log.error("Error to initialize Stripe:", error);
    showError(getErrorFrontStripe(FrontendStripe.STRIPE_INITIALIZATION_ERROR));
    return null;
  }
}
