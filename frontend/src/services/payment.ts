import type {
  CheckoutPaymentIntentRequest,
  CheckoutPaymentIntentResponse,
  RelationalCalStripe,
  ResponseAPI,
} from "@/types/bakend-types";
import { ApiService } from "./helper";
import { getErrorFrontStripe, FrontendStripe } from "@/enums/enums";
import type { PricingApiResponse } from "@/types/types";
import { getPrice } from "./calendar";
import { log } from "./logger";

// Función para procesar el pago
export async function handlePayment(stripe: any, elements: any, bookingUid: string) {
  const helper = new ApiService();

  // Verificar que Stripe esté inicializado
  if (!stripe || !elements) {
    console.error("❌ Stripe o Elements no inicializados");
    showError(getErrorFrontStripe(FrontendStripe.STRIPE_NOT_INITIALIZED));
    return;
  }

  // Verificar elementos del DOM
  const submitButton = document.getElementById("submit-button") as HTMLButtonElement | null;
  const buttonText = document.getElementById("button-text") as HTMLButtonElement | null;

  if (!submitButton || !buttonText) {
    console.error("❌ Botones no encontrados en el DOM");
    showError(getErrorFrontStripe(FrontendStripe.MISSING_ELEMENTS));
    return;
  }

  // Deshabilitar botón y mostrar loading
  submitButton.disabled = true;
  buttonText.textContent = "Procesando...";
  clearMessages();

  try {
    const result = await stripe.confirmPayment({
      elements,
      confirmParams: {
        return_url: location.origin + "/payments/payment-success",
      },
      redirect: "if_required",
    });

    const { error, paymentIntent } = result;

    // ═══════════════════════════════════════
    // MANEJO DE ERRORES
    // ═══════════════════════════════════════
    if (error) {
      console.error("═══════════════════════════════════════");
      console.error("❌ ERROR EN STRIPE.CONFIRMPAYMENT");
      console.error("═══════════════════════════════════════");
      console.error("Error type:", error.type);
      console.error("Error code:", error.code);
      console.error("Error message:", error.message);
      console.error("Error decline_code:", error.decline_code);
      console.error("Error doc_url:", error.doc_url);
      console.error("Payment Intent ID:", error.payment_intent?.id);
      console.error("Payment Intent status:", error.payment_intent?.status);
      console.error("Error completo:", JSON.stringify(error, null, 2));
      console.error("═══════════════════════════════════════");

      // Mensaje específico según el tipo de error
      let errorMessage = error.message || "Error al procesar el pago";

      if (error.type === "card_error") {
        console.error("🔴 Tipo: Error de tarjeta");
        errorMessage = `Tarjeta rechazada: ${error.message}`;
      } else if (error.type === "validation_error") {
        console.error("🔴 Tipo: Error de validación");
        errorMessage = `Validación: ${error.message}`;
      } else if (error.type === "invalid_request_error") {
        console.error("🔴 Tipo: Error de petición inválida (posible problema de configuración)");
        errorMessage = `Configuración: ${error.message}`;
      } else if (error.type === "api_error") {
        console.error("🔴 Tipo: Error de API de Stripe");
        errorMessage = `Error del servidor: ${error.message}`;
      }

      showError(errorMessage);
      submitButton.disabled = false;
      buttonText.textContent = "Pagar ahora";
      return;
    }

    // ═══════════════════════════════════════
    // PAGO EXITOSO - PROCESAR
    // ═══════════════════════════════════════

    if (!paymentIntent) {
      showError(getErrorFrontStripe(FrontendStripe.UNKNOWN_PAYMENT_STATUS));
      submitButton.disabled = false;
      buttonText.textContent = "Pagar ahora";
      return;
    }

    if (paymentIntent.status === "succeeded") {
      showSuccess(getErrorFrontStripe(FrontendStripe.PAYMENT_SUCCESS));

      if (!bookingUid) {
        console.error("❌ No hay bookingUid, no se puede confirmar la reserva");
        showError(getErrorFrontStripe(FrontendStripe.MISSING_BOOKING));
        return;
      }

      try {
        const responseBookingConfirm = await helper.confirmBooking(bookingUid);

        if (!responseBookingConfirm.success) {
          console.error("❌ Error al confirmar booking");
          showError(getErrorFrontStripe(FrontendStripe.BOOKING_CONFIRM_ERROR));
          return;
        }

        const relation: RelationalCalStripe = {
          cal_id: bookingUid,
          stripe_id: paymentIntent.id,
        };

        const responseSaveRelation = await helper.saveCalStripeConnection(relation);

        if (!responseSaveRelation.success) {
          console.error("❌ Error al guardar relación");
          showError(getErrorFrontStripe(FrontendStripe.RELATION_SAVE_ERROR));
          return;
        }

        // Todo a salido bien
        // Enviar evento a Google Analytics
        if (window.gtag) window.gtag("event", "class_booking", { bookingUid });
        globalThis.location.href = "/payments/payment-success";
        return;
      } catch (e) {
        console.error("═══════════════════════════════════════");
        console.error("💥 ERROR al confirmar booking o guardar relación");
        console.error("═══════════════════════════════════════");
        console.error("Error completo:", e);
        console.error("Error message:", (e as Error).message);
        console.error("Error stack:", (e as Error).stack);
        showError(getErrorFrontStripe(FrontendStripe.GENERIC_ERROR));
      }
    } else {
      // if (paymentIntent.status === "requires_action") {
      // } else if (paymentIntent.status === "processing") {
      // } else if (paymentIntent.status === "requires_payment_method") {
      // }

      showError(getErrorFrontStripe(FrontendStripe.UNKNOWN_PAYMENT_STATUS));
    }
  } catch (error) {
    console.error("Error name:", (error as Error).name);
    showError(getErrorFrontStripe(FrontendStripe.CONNECTION_ERROR));
  } finally {
    if (!submitButton.disabled) {
      submitButton.disabled = false;
      buttonText.textContent = "Pagar ahora";
    }
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
export function showSuccess(message: string): void {
  const errorDiv = document.getElementById("error-message");
  const successDiv = document.getElementById("success-message");

  if (errorDiv) errorDiv.textContent = "";
  if (successDiv) {
    successDiv.textContent = message;
    successDiv.style.display = "block";
  }
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

export async function initializePrice(testCountry: string | null, slugType: string): Promise<number | undefined> {
  try {
    // Obtenemos la lista de precios desde el backend
    const apiUrl: string = testCountry ? `/api/pricing?test_country=${testCountry}` : "/api/pricing";
    const response: Response = await fetch(apiUrl);
    const pricingData: PricingApiResponse = (await response.json()) as PricingApiResponse;

    if (!slugType) {
      showError(getErrorFrontStripe(FrontendStripe.MISSING_SLUG));
      return;
    } else if (!response.ok) {
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
  }
}

export async function initializeStripe(
  STRIPE_PUBLIC_KEY: string,
  pricing: number
): Promise<{ stripe: any; elements: any } | null> {
  try {
    const helper = new ApiService();

    // Inicializar Stripe
    const stripe = window.Stripe(STRIPE_PUBLIC_KEY);

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
    const loading = document.querySelector(".loading") as HTMLDivElement | null;
    if (!loading || !loading.style) {
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
    console.error("Error inicializando Stripe:", error);
    showError(getErrorFrontStripe(FrontendStripe.STRIPE_INITIALIZATION_ERROR));
    return null;
  }
}
