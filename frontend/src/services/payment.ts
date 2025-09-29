import type { RelationalCalStripe } from "@/types/bakend-types";
import { ApiService } from "./helper";
import { getErrorFrontStripe, FrontendStripe } from "@/enums/enums";

// Función para procesar el pago
export async function handlePayment(stripe: any, elements: any, bookingUid: string | null) {
  const helper = new ApiService();

  if (!stripe || !elements) {
    showError(getErrorFrontStripe(FrontendStripe.STRIPE_NOT_INITIALIZED));
    return;
  }

  const submitButton = document.getElementById("submit-button") as HTMLButtonElement | null;
  const buttonText = document.getElementById("button-text") as HTMLButtonElement | null;
  if (!submitButton || !buttonText) {
    showError(getErrorFrontStripe(FrontendStripe.MISSING_ELEMENTS));
    return;
  }

  // Deshabilitar botón y mostrar loading
  submitButton.disabled = true;
  buttonText.textContent = "Procesando...";
  clearMessages();

  try {
    // Confirmar el pago sin redirigir automáticamente
    const { _, paymentIntent } = await stripe.confirmPayment({
      elements,
      confirmParams: {
        return_url: window.location.origin + "/payment/payment-success",
      },
      redirect: "if_required",
    });

    if (!paymentIntent) {
      showError(getErrorFrontStripe(FrontendStripe.UNKNOWN_PAYMENT_STATUS));
      return;
    }

    if (paymentIntent.status === "succeeded") {
      showSuccess(getErrorFrontStripe(FrontendStripe.PAYMENT_SUCCESS));

      if (!bookingUid) {
        showError(getErrorFrontStripe(FrontendStripe.MISSING_BOOKING));
        return;
      }

      // Confirmar booking y guardar relación en la base de datos
      try {
        const responseBookingConfirm = await helper.confirmBooking(bookingUid);
        if (!responseBookingConfirm.success) {
          showError(getErrorFrontStripe(FrontendStripe.BOOKING_CONFIRM_ERROR));
          return;
        }

        const relation: RelationalCalStripe = {
          cal_id: bookingUid,
          stripe_id: paymentIntent.id,
        };

        const responseSaveRelation = await helper.saveCalStripeConnection(relation);
        if (!responseSaveRelation.success) {
          showError(getErrorFrontStripe(FrontendStripe.RELATION_SAVE_ERROR));
          return;
        }

        // Redirigir al usuario
        window.location.href = "/payment/payment-success";
        return; // Evitar ejecutar el finally innecesariamente
      } catch (e) {
        console.error("Error al confirmar booking:", e);
        showError(getErrorFrontStripe(FrontendStripe.GENERIC_ERROR));
      }
    } else {
      showError(getErrorFrontStripe(FrontendStripe.UNKNOWN_PAYMENT_STATUS));
    }
  } catch (error) {
    console.error("Error procesando pago:", error);
    showError(getErrorFrontStripe(FrontendStripe.CONNECTION_ERROR));
  } finally {
    // Rehabilitar el botón solo si no hubo redirección
    if (!submitButton.disabled) {
      submitButton.disabled = false;
      buttonText.textContent = "Pagar ahora";
    }
  }
}

// Funciones de utilidad para mostrar mensajes
export function showError(content: string) {
  const errorDiv = document.getElementById("error-message");
  if (!errorDiv) return;
  errorDiv.textContent = content;
  errorDiv.style.display = "block";
}

// Mostrar mensaje de éxito
export function showSuccess(content: string) {
  const successDiv = document.getElementById("success-message");
  if (!successDiv) return;
  successDiv.textContent = content;
  successDiv.style.display = "block";
}

// Limpiar mensajes
export function clearMessages() {
  const errorDiv = document.getElementById("error-message");
  const successDiv = document.getElementById("success-message");
  if (!errorDiv || !successDiv) return;
  errorDiv.style.display = "none";
  successDiv.style.display = "none";
}

function handleStripeError(error: any) {
  if (error.type === "card_error" || error.type === "validation_error") {
    showError(error.message);
  } else {
    showError("Ha ocurrido un error inesperado. Inténtalo de nuevo.");
  }
}
