import type { RelationalCalStripe } from "@/types/bakend-types";
import { ApiService } from "./helper";
import { getErrorFrontStripe, FrontendStripe } from "@/enums/enums";

// Función para procesar el pago
export async function handlePayment(stripe: any, elements: any, bookingUid: string | null) {
  const helper = new ApiService();

  console.log("═══════════════════════════════════════");
  console.log("🚀 INICIO handlePayment");
  console.log("═══════════════════════════════════════");

  // Verificar que Stripe esté inicializado
  if (!stripe || !elements) {
    console.error("❌ Stripe o Elements no inicializados");
    showError(getErrorFrontStripe(FrontendStripe.STRIPE_NOT_INITIALIZED));
    return;
  }
  console.log("✅ Stripe y Elements inicializados correctamente");

  // Verificar elementos del DOM
  const submitButton = document.getElementById("submit-button") as HTMLButtonElement | null;
  const buttonText = document.getElementById("button-text") as HTMLButtonElement | null;

  if (!submitButton || !buttonText) {
    console.error("❌ Botones no encontrados en el DOM");
    showError(getErrorFrontStripe(FrontendStripe.MISSING_ELEMENTS));
    return;
  }
  console.log("✅ Elementos del DOM encontrados");

  // Verificar bookingUid
  console.log("📋 BookingUid:", bookingUid || "NO PROPORCIONADO");

  // Deshabilitar botón y mostrar loading
  submitButton.disabled = true;
  buttonText.textContent = "Procesando...";
  clearMessages();

  try {
    console.log("───────────────────────────────────────");
    console.log("🔄 Llamando a stripe.confirmPayment()...");
    console.log("───────────────────────────────────────");

    const result = await stripe.confirmPayment({
      elements,
      confirmParams: {
        return_url: window.location.origin + "/payment/payment-success",
      },
      redirect: "if_required",
    });

    console.log("📦 Resultado completo de confirmPayment:", result);

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
    console.log("───────────────────────────────────────");
    console.log("✅ NO HAY ERRORES, procesando paymentIntent...");
    console.log("───────────────────────────────────────");

    if (!paymentIntent) {
      console.warn("⚠️ PaymentIntent es null o undefined");
      console.warn("Esto puede significar que hubo un redirect automático");
      showError(getErrorFrontStripe(FrontendStripe.UNKNOWN_PAYMENT_STATUS));
      submitButton.disabled = false;
      buttonText.textContent = "Pagar ahora";
      return;
    }

    console.log("📄 PaymentIntent recibido:");
    console.log("  - ID:", paymentIntent.id);
    console.log("  - Status:", paymentIntent.status);
    console.log("  - Amount:", paymentIntent.amount);
    console.log("  - Currency:", paymentIntent.currency);
    console.log("  - Cliente secret (primeros 20 chars):", paymentIntent.client_secret?.substring(0, 20) + "...");

    if (paymentIntent.status === "succeeded") {
      console.log("═══════════════════════════════════════");
      console.log("🎉 PAGO EXITOSO - Status: succeeded");
      console.log("═══════════════════════════════════════");

      showSuccess(getErrorFrontStripe(FrontendStripe.PAYMENT_SUCCESS));

      if (!bookingUid) {
        console.error("❌ No hay bookingUid, no se puede confirmar la reserva");
        showError(getErrorFrontStripe(FrontendStripe.MISSING_BOOKING));
        return;
      }

      console.log("───────────────────────────────────────");
      console.log("📅 Confirmando booking:", bookingUid);
      console.log("───────────────────────────────────────");

      try {
        const responseBookingConfirm = await helper.confirmBooking(bookingUid);
        console.log("📥 Respuesta de confirmBooking:", responseBookingConfirm);

        if (!responseBookingConfirm.success) {
          console.error("❌ Error al confirmar booking");
          showError(getErrorFrontStripe(FrontendStripe.BOOKING_CONFIRM_ERROR));
          return;
        }
        console.log("✅ Booking confirmado exitosamente");

        console.log("───────────────────────────────────────");
        console.log("💾 Guardando relación Cal-Stripe");
        console.log("───────────────────────────────────────");

        const relation: RelationalCalStripe = {
          cal_id: bookingUid,
          stripe_id: paymentIntent.id,
        };
        console.log("📝 Relación a guardar:", relation);

        const responseSaveRelation = await helper.saveCalStripeConnection(relation);
        console.log("📥 Respuesta de saveCalStripeConnection:", responseSaveRelation);

        if (!responseSaveRelation.success) {
          console.error("❌ Error al guardar relación");
          showError(getErrorFrontStripe(FrontendStripe.RELATION_SAVE_ERROR));
          return;
        }
        console.log("✅ Relación guardada exitosamente");

        console.log("═══════════════════════════════════════");
        console.log("🎊 TODO COMPLETADO - Redirigiendo...");
        console.log("═══════════════════════════════════════");

        window.location.href = "/payment/payment-success";
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
      console.warn("═══════════════════════════════════════");
      console.warn("⚠️ STATUS NO ES 'SUCCEEDED'");
      console.warn("═══════════════════════════════════════");
      console.warn("Status actual:", paymentIntent.status);
      console.warn("Posibles valores: requires_action, processing, requires_payment_method, etc.");

      if (paymentIntent.status === "requires_action") {
        console.warn("🔐 Requiere acción adicional (probablemente 3D Secure)");
      } else if (paymentIntent.status === "processing") {
        console.warn("⏳ Pago en proceso, puede tardar");
      } else if (paymentIntent.status === "requires_payment_method") {
        console.warn("💳 Requiere método de pago");
      }

      showError(getErrorFrontStripe(FrontendStripe.UNKNOWN_PAYMENT_STATUS));
    }
  } catch (error) {
    console.error("═══════════════════════════════════════");
    console.error("💥 ERROR INESPERADO EN TRY-CATCH");
    console.error("═══════════════════════════════════════");
    console.error("Error completo:", error);
    console.error("Error message:", (error as Error).message);
    console.error("Error stack:", (error as Error).stack);
    console.error("Error name:", (error as Error).name);
    showError(getErrorFrontStripe(FrontendStripe.CONNECTION_ERROR));
  } finally {
    console.log("───────────────────────────────────────");
    console.log("🏁 FINALLY - Limpieza");
    console.log("Botón deshabilitado:", submitButton.disabled);
    console.log("───────────────────────────────────────");

    if (!submitButton.disabled) {
      submitButton.disabled = false;
      buttonText.textContent = "Pagar ahora";
      console.log("✅ Botón rehabilitado");
    }
  }

  console.log("═══════════════════════════════════════");
  console.log("FIN handlePayment");
  console.log("═══════════════════════════════════════");
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
