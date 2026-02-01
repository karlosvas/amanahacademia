import type {
  Attendee,
  BookingRequest,
  CalBookingPayload,
  CheckoutPaymentIntentRequest,
  CheckoutPaymentIntentResponse,
  ResponseAPI,
} from "@/types/bakend-types";
import { ApiService } from "./helper";
import { getErrorFrontStripe, FrontendStripe } from "@/enums/enums";
import type { PricingApiResponse } from "@/types/types";
import { getPrice } from "./calendar";
import { log } from "./logger";

// Función para procesar el pago
export async function handlePayment(
  stripe: any,
  elements: any,
  bookingUid: string,
  status: string,
  slug: string,
  email: string,
): Promise<void> {
  const helper = new ApiService();

  // Verificar que Stripe esté inicializado
  if (!stripe || !elements) {
    log.error("Stripe o Elements dont initialized");
    showError(getErrorFrontStripe(FrontendStripe.STRIPE_NOT_INITIALIZED));
    throw new Error("Stripe o Elements no inicializados");
  }

  // Verificar elementos del DOM
  const submitButton = document.getElementById("submit-button") as HTMLButtonElement | null;
  const buttonText = document.getElementById("button-text") as HTMLButtonElement | null;

  if (!submitButton || !buttonText) {
    log.error("Submit button or button text elements not found in DOM");
    showError(getErrorFrontStripe(FrontendStripe.MISSING_ELEMENTS));
    throw new Error("Botones no encontrados en el DOM");
  }

  // Deshabilitar botón y mostrar loading
  submitButton.disabled = true;
  buttonText.textContent = "Procesando...";
  clearMessages();

  const result = await stripe.confirmPayment({
    elements,
    confirmParams: {
      return_url: location.origin + "/payments/payment-success",
    },
    redirect: "if_required",
  });

  const { error, paymentIntent } = result;

  if (error) {
    // Mensaje específico según el tipo de error
    let errorMessage = error.message || "Error al procesar el pago";

    if (error.type === "card_error") {
      log.error("🔴 Stripe: Card Error", result);
      errorMessage = `Tarjeta rechazada: ${error.message}`;
    } else if (error.type === "validation_error") {
      log.error("🔴 Stripe: Validation Error", result);
      errorMessage = `Validación: ${error.message}`;
    } else if (error.type === "invalid_request_error") {
      log.error("🔴 Stripe: Invalid Request Error (possible configuration issue)", result);
      errorMessage = `Configuración: ${error.message}`;
    } else if (error.type === "api_error") {
      log.error("🔴 Stripe: Stripe API Error", result);
      errorMessage = `Error desconocido: ${error.message}`;
    }

    log.error("💥 Error en el pago", errorMessage);
    submitButton.disabled = false;

    throw new Error(`Error en el pago: ${errorMessage}`);
  }

  if (!paymentIntent) {
    showError(getErrorFrontStripe(FrontendStripe.UNKNOWN_PAYMENT_STATUS));
    submitButton.disabled = false;
    throw new Error("PaymentIntent no disponible después de la confirmación");
  }

  if (paymentIntent.status === "succeeded") {
    // El pago fue exitoso
    await successPayment(helper, paymentIntent, bookingUid, status, slug, email);
  } else {
    log.error("Estado de pago desconocido: ", paymentIntent.status);
    showError(getErrorFrontStripe(FrontendStripe.UNKNOWN_PAYMENT_STATUS));
    throw new Error(`Estado de pago desconocido: ${paymentIntent.status}`);
  }
}

// Funciones de utilidad para mostrar mensajes
export function showError(message: string) {
  const errorDiv = document.getElementById("error-message");
  const successDiv = document.getElementById("success-message");

  if (successDiv) successDiv.textContent = "";
  if (errorDiv) {
    errorDiv.textContent = message;
    errorDiv.style.display = "block";
  }
}

// Mostrar mensaje de éxito
export async function successPayment(
  helper: ApiService,
  paymentIntent: any,
  bookingUid: string,
  status: string,
  slug: string,
  email: string,
): Promise<void> {
  if (slug === "group-class") {
    const actualBooking: ResponseAPI<CalBookingPayload> = await helper.getBookingById(bookingUid);
    if (!actualBooking.success) {
      log.error("Error to get booking");
      showError(getErrorFrontStripe(FrontendStripe.MISSING_BOOKING));
      throw new Error("Error to get booking");
    }

    let attendees: Attendee[] = actualBooking.data.attendees;
    attendees.push({ name: "", email: email });

    let booking: BookingRequest = {
      ...actualBooking.data,
      attendees: attendees,
      startTime: actualBooking.data.startTime,
      endTime: actualBooking.data.endTime,
    } as BookingRequest;
    const response: ResponseAPI<CalBookingPayload> = await helper.createBooking(booking);
    if (!response.success) {
      log.error("Error to update booking");
      throw new Error("Error to update booking");
    }
  }

  // Confirmar la reserva en el backend si procede
  if (status !== "accepted") {
    const responseBookingConfirm = await helper.confirmBooking(bookingUid);
    if (!responseBookingConfirm.success) {
      log.error("Error to confirm booking");
      showError(getErrorFrontStripe(FrontendStripe.BOOKING_CONFIRM_ERROR));
      throw new Error("Error to confirm booking");
    }
  }

  // Guardar la relacion en el bakend
  const responseSaveRelation = await helper.saveCalStripeConnection({
    cal_id: bookingUid,
    stripe_id: paymentIntent.id,
  });

  const errorDiv = document.getElementById("error-message");
  if (errorDiv) errorDiv.textContent = "";

  if (!responseSaveRelation.success) {
    log.error("Error to save relation");
    showError(getErrorFrontStripe(FrontendStripe.RELATION_SAVE_ERROR));
    throw new Error("Error to save relation");
  }

  // Enviar evento a Google Analytics
  if (gtag) gtag("event", "class_booking", { bookingUid });

  // Redirigir a la página de éxito
  setTimeout(() => {
    globalThis.location.href = "/payments/payment-success";
  }, 2000);
}

// Limpiar mensajes
export function clearMessages() {
  const errorDiv = document.getElementById("error-message");
  const successDiv = document.getElementById("success-message");
  if (errorDiv) {
    errorDiv.textContent = "";
    errorDiv.style.display = "none";
  }
  if (successDiv) {
    successDiv.textContent = "";
    successDiv.style.display = "none";
  }
}

// Inicializar el precio basado en el país de prueba y el tipo de clase
export async function initializePrice(testCountry: string | null, slugType: string): Promise<number | undefined> {
  // Validar que el tipo de clase esté definido
  if (!slugType) {
    showError(getErrorFrontStripe(FrontendStripe.MISSING_SLUG));
    return;
  }

  try {
    // Obtenemos la lista de precios desde el backend
    const apiUrl: string = testCountry ? `/api/pricing?test_country=${testCountry}` : "/api/pricing";
    const response: Response = await fetch(apiUrl);
    const pricingData: PricingApiResponse = (await response.json()) as PricingApiResponse;
    if (!response.ok) {
      showError(getErrorFrontStripe(FrontendStripe.PRICING_FETCH_ERROR));
      return;
    }

    // Obtenemos el precio para el tipo de clase seleccionado
    const pricing = getPrice(slugType, pricingData);
    const pricingElement = document.getElementById("pricing");
    if (!pricingElement) {
      showError(getErrorFrontStripe(FrontendStripe.PRICING_ELEMENT_NOT_FOUND));
      return;
    }

    pricingElement.textContent = `${pricing} €`;

    return pricing;
  } catch (error) {
    log.error(getErrorFrontStripe(FrontendStripe.GENERIC_ERROR));
    showError(getErrorFrontStripe(FrontendStripe.GENERIC_ERROR));
    return;
  }
}

// Inicializar Stripe y crear el Payment Element
export async function initializeStripe(
  STRIPE_PUBLIC_KEY: string,
  pricing: number,
): Promise<{ stripe: any; elements: any } | null> {
  try {
    const helper = new ApiService();

    // Inicializar Stripe
    const stripe = globalThis.Stripe(STRIPE_PUBLIC_KEY);

    // Transformamos de euros a centimos
    const amount = Math.round(pricing * 100);

    const carry: CheckoutPaymentIntentRequest = {
      amount,
      currency: "EUR",
    };

    if (!carry) throw new Error(getErrorFrontStripe(FrontendStripe.GENERIC_ERROR));

    // Obtenemos el clinet secreat para elements
    let response: ResponseAPI<CheckoutPaymentIntentResponse> = await helper.checkout(carry);

    // Comprobamos que la respuesta sea válida
    if (!response.success) throw new Error(getErrorFrontStripe(FrontendStripe.SERVER_ERROR));

    // Obtenemos el secreto de cliente
    const data = response.data;

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
    const elements = stripe.elements({
      clientSecret: data.client_secret,
      appearance,
    });

    // Crear y montar el Payment Element
    const paymentElement = elements.create("payment");

    // Ocultar loading y mostrar el elemento de pago
    const loading = document.querySelector(".loading") as HTMLDivElement;
    if (!loading.style) {
      showError(getErrorFrontStripe(FrontendStripe.PAYMENT_FORM_ERROR));
      return null;
    }
    loading.style.display = "none";
    await paymentElement.mount("#payment-element");

    // Habilitar el botón de pago
    const submitButton = document.getElementById("submit-button") as HTMLButtonElement | null;
    if (!submitButton) return null;
    submitButton.disabled = false;

    // Manejar cambios en el elemento de pago
    paymentElement.on("change", (event: any) => {
      if (event.error) showError(event.error.message);
      else clearMessages();
    });

    // Retornar stripe y elements inicializados
    return { stripe, elements };
  } catch (error: any) {
    log.error("Error to initialize Stripe:", error);
    showError(getErrorFrontStripe(FrontendStripe.STRIPE_INITIALIZATION_ERROR));
    return null;
  }
}
