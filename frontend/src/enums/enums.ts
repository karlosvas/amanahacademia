export enum Class {
  Standard = "standard-class",
  Conversacion = "conversation-class",
  Grupales = "group-class",
  Free = "free-class",
}

export enum ApiErrorType {
  NETWORK_ERROR = "NETWORK_ERROR",
  AUTHENTICATION_ERROR = "AUTHENTICATION_ERROR",
  SESSION_EXPIRED = "SESSION_EXPIRED",
  SESSION_NOT_FOUND = "SESSION_NOT_FOUND",
  VALIDATION_ERROR = "VALIDATION_ERROR",
  SERVER_ERROR = "SERVER_ERROR",
  UNKNOWN_ERROR = "UNKNOWN_ERROR",
}

import { getLangFromCookie } from "@/utils/cookie";

export enum FrontendErrorCode {
  NEED_AUTHENTICATION = "NEED_AUTHENTICATION",
  MUST_BE_OWNER = "MUST_BE_OWNER",
  NOT_FOUND = "NOT_FOUND",
  FORBIDDEN = "FORBIDDEN",
  UNKNOWN_ERROR = "UNKNOWN_ERROR",
  NEWSLETTER_ERROR = "NEWSLETTER_ERROR",
}

export const FrontendErrorMessages: Record<string, Record<FrontendErrorCode, string>> = {
  ar: {
    [FrontendErrorCode.NEED_AUTHENTICATION]: "يجب عليك تسجيل الدخول للمتابعة",
    [FrontendErrorCode.MUST_BE_OWNER]: "يجب أن تكون مالك العنصر لتنفيذ هذا الإجراء",
    [FrontendErrorCode.NOT_FOUND]: "العنصر غير موجود",
    [FrontendErrorCode.FORBIDDEN]: "ليس لديك إذن للوصول",
    [FrontendErrorCode.UNKNOWN_ERROR]: "حدث خطأ غير معروف",
    [FrontendErrorCode.NEWSLETTER_ERROR]: "حدث خطأ في الاشتراك في النشرة الإخبارية",
  },
  de: {
    [FrontendErrorCode.NEED_AUTHENTICATION]: "Sie müssen sich anmelden, um fortzufahren",
    [FrontendErrorCode.MUST_BE_OWNER]: "Sie müssen Eigentümer sein, um diese Aktion auszuführen",
    [FrontendErrorCode.NOT_FOUND]: "Element nicht gefunden",
    [FrontendErrorCode.FORBIDDEN]: "Keine Berechtigung zum Zugriff",
    [FrontendErrorCode.UNKNOWN_ERROR]: "Unbekannter Fehler ist aufgetreten",
    [FrontendErrorCode.NEWSLETTER_ERROR]: "Fehler beim Abonnieren des Newsletters",
  },
  en: {
    [FrontendErrorCode.NEED_AUTHENTICATION]: "You must log in to continue",
    [FrontendErrorCode.MUST_BE_OWNER]: "You must be the owner to perform this action",
    [FrontendErrorCode.NOT_FOUND]: "Item not found",
    [FrontendErrorCode.FORBIDDEN]: "You do not have permission to access",
    [FrontendErrorCode.UNKNOWN_ERROR]: "An unknown error occurred",
    [FrontendErrorCode.NEWSLETTER_ERROR]: "Error subscribing to the newsletter",
  },
  es: {
    [FrontendErrorCode.NEED_AUTHENTICATION]: "Debes iniciar sesión para continuar",
    [FrontendErrorCode.MUST_BE_OWNER]: "Debes de ser dueño para realizar esta acción",
    [FrontendErrorCode.NOT_FOUND]: "Elemento no encontrado",
    [FrontendErrorCode.FORBIDDEN]: "No tienes permiso para acceder",
    [FrontendErrorCode.UNKNOWN_ERROR]: "Ha ocurrido un error desconocido",
    [FrontendErrorCode.NEWSLETTER_ERROR]: "Error al suscribirse al newsletter",
  },
  fr: {
    [FrontendErrorCode.NEED_AUTHENTICATION]: "Vous devez vous connecter pour continuer",
    [FrontendErrorCode.MUST_BE_OWNER]: "Vous devez être le propriétaire pour effectuer cette action",
    [FrontendErrorCode.NOT_FOUND]: "Élément introuvable",
    [FrontendErrorCode.FORBIDDEN]: "Vous n'avez pas la permission d'accéder",
    [FrontendErrorCode.UNKNOWN_ERROR]: "Une erreur inconnue s'est produite",
    [FrontendErrorCode.NEWSLETTER_ERROR]: "Erreur lors de l'abonnement à la newsletter",
  },
  it: {
    [FrontendErrorCode.NEED_AUTHENTICATION]: "Devi accedere per continuare",
    [FrontendErrorCode.MUST_BE_OWNER]: "Devi essere il proprietario per eseguire questa azione",
    [FrontendErrorCode.NOT_FOUND]: "Elemento non trovato",
    [FrontendErrorCode.FORBIDDEN]: "Non hai il permesso di accedere",
    [FrontendErrorCode.UNKNOWN_ERROR]: "Si è verificato un errore sconosciuto",
    [FrontendErrorCode.NEWSLETTER_ERROR]: "Errore durante l'iscrizione alla newsletter",
  },
  pt: {
    [FrontendErrorCode.NEED_AUTHENTICATION]: "Você deve fazer login para continuar",
    [FrontendErrorCode.MUST_BE_OWNER]: "Você deve ser o dono para realizar esta ação",
    [FrontendErrorCode.NOT_FOUND]: "Item não encontrado",
    [FrontendErrorCode.FORBIDDEN]: "Você não tem permissão para acessar",
    [FrontendErrorCode.UNKNOWN_ERROR]: "Ocorreu um erro desconhecido",
    [FrontendErrorCode.NEWSLETTER_ERROR]: "Erro ao se inscrever na newsletter",
  },
};

export function getErrorToast(code: FrontendErrorCode) {
  const lang = getLangFromCookie();
  return FrontendErrorMessages[lang]?.[code] || FrontendErrorMessages["es"][code];
}

export enum FrontendStripe {
  STRIPE_NOT_INITIALIZED = "STRIPE_NOT_INITIALIZED",
  MISSING_ELEMENTS = "MISSING_ELEMENTS",
  UNKNOWN_PAYMENT_STATUS = "UNKNOWN_PAYMENT_STATUS",
  PAYMENT_SUCCESS = "PAYMENT_SUCCESS",
  MISSING_BOOKING = "MISSING_BOOKING",
  BOOKING_CONFIRM_ERROR = "BOOKING_CONFIRM_ERROR",
  RELATION_SAVE_ERROR = "RELATION_SAVE_ERROR",
  UNCERTAIN_PAYMENT = "UNCERTAIN_PAYMENT",
  CONNECTION_ERROR = "CONNECTION_ERROR",
  GENERIC_ERROR = "GENERIC_ERROR",
}
export const FrontendStripeTranslations: Record<string, Record<FrontendStripe, string>> = {
  es: {
    [FrontendStripe.STRIPE_NOT_INITIALIZED]: "No se pudo inicializar el sistema de pago",
    [FrontendStripe.MISSING_ELEMENTS]: "Error con los elementos de pago",
    [FrontendStripe.UNKNOWN_PAYMENT_STATUS]: "No se pudo completar el pago",
    [FrontendStripe.PAYMENT_SUCCESS]: "¡Pago completado con éxito!",
    [FrontendStripe.MISSING_BOOKING]: "No se encontró la reserva",
    [FrontendStripe.BOOKING_CONFIRM_ERROR]: "Hubo un problema confirmando la reserva",
    [FrontendStripe.RELATION_SAVE_ERROR]: "Hubo un problema guardando la relación de pago",
    [FrontendStripe.UNCERTAIN_PAYMENT]: "Estado del pago incierto. Por favor, revisa tu cuenta",
    [FrontendStripe.CONNECTION_ERROR]: "Error de conexión. Inténtalo de nuevo",
    [FrontendStripe.GENERIC_ERROR]: "Ocurrió un error inesperado. Inténtalo de nuevo",
  },
  en: {
    [FrontendStripe.STRIPE_NOT_INITIALIZED]: "Payment system could not be initialized",
    [FrontendStripe.MISSING_ELEMENTS]: "Payment elements error",
    [FrontendStripe.UNKNOWN_PAYMENT_STATUS]: "Payment could not be completed",
    [FrontendStripe.PAYMENT_SUCCESS]: "Payment completed successfully!",
    [FrontendStripe.MISSING_BOOKING]: "Booking ID not found",
    [FrontendStripe.BOOKING_CONFIRM_ERROR]: "There was a problem confirming the booking",
    [FrontendStripe.RELATION_SAVE_ERROR]: "There was a problem saving the payment relation",
    [FrontendStripe.UNCERTAIN_PAYMENT]: "Payment status uncertain. Please check your account",
    [FrontendStripe.CONNECTION_ERROR]: "Connection error. Please try again",
    [FrontendStripe.GENERIC_ERROR]: "An unexpected error occurred. Please try again",
  },
  pt: {
    [FrontendStripe.STRIPE_NOT_INITIALIZED]: "O sistema de pagamento não pôde ser inicializado",
    [FrontendStripe.MISSING_ELEMENTS]: "Erro nos elementos de pagamento",
    [FrontendStripe.UNKNOWN_PAYMENT_STATUS]: "O pagamento não pôde ser concluído",
    [FrontendStripe.PAYMENT_SUCCESS]: "Pagamento concluído com sucesso!",
    [FrontendStripe.MISSING_BOOKING]: "ID da reserva não encontrado",
    [FrontendStripe.BOOKING_CONFIRM_ERROR]: "Houve um problema ao confirmar a reserva",
    [FrontendStripe.RELATION_SAVE_ERROR]: "Houve um problema ao salvar a relação de pagamento",
    [FrontendStripe.UNCERTAIN_PAYMENT]: "Status do pagamento incerto. Por favor, verifique sua conta",
    [FrontendStripe.CONNECTION_ERROR]: "Erro de conexão. Por favor, tente novamente",
    [FrontendStripe.GENERIC_ERROR]: "Ocorreu um erro inesperado. Por favor, tente novamente",
  },
  it: {
    [FrontendStripe.STRIPE_NOT_INITIALIZED]: "Il sistema di pagamento non è stato inizializzato",
    [FrontendStripe.MISSING_ELEMENTS]: "Errore negli elementi di pagamento",
    [FrontendStripe.UNKNOWN_PAYMENT_STATUS]: "Il pagamento non è stato completato",
    [FrontendStripe.PAYMENT_SUCCESS]: "Pagamento completato con successo!",
    [FrontendStripe.MISSING_BOOKING]: "ID della prenotazione non trovato",
    [FrontendStripe.BOOKING_CONFIRM_ERROR]: "Si è verificato un problema durante la conferma della prenotazione",
    [FrontendStripe.RELATION_SAVE_ERROR]:
      "Si è verificato un problema durante il salvataggio della relazione di pagamento",
    [FrontendStripe.UNCERTAIN_PAYMENT]: "Stato del pagamento incerto. Si prega di controllare il proprio account",
    [FrontendStripe.CONNECTION_ERROR]: "Errore di connessione. Si prega di riprovare",
    [FrontendStripe.GENERIC_ERROR]: "Si è verificato un errore imprevisto. Si prega di riprovare",
  },
  fr: {
    [FrontendStripe.STRIPE_NOT_INITIALIZED]: "Le système de paiement n'a pas pu être initialisé",
    [FrontendStripe.MISSING_ELEMENTS]: "Erreur des éléments de paiement",
    [FrontendStripe.UNKNOWN_PAYMENT_STATUS]: "Le paiement n'a pas pu être complété",
    [FrontendStripe.PAYMENT_SUCCESS]: "Paiement effectué avec succès !",
    [FrontendStripe.MISSING_BOOKING]: "ID de réservation introuvable",
    [FrontendStripe.BOOKING_CONFIRM_ERROR]: "Un problème est survenu lors de la confirmation de la réservation",
    [FrontendStripe.RELATION_SAVE_ERROR]: "Un problème est survenu lors de l'enregistrement de la relation de paiement",
    [FrontendStripe.UNCERTAIN_PAYMENT]: "Statut de paiement incertain. Veuillez vérifier votre compte",
    [FrontendStripe.CONNECTION_ERROR]: "Erreur de connexion. Veuillez réessayer",
    [FrontendStripe.GENERIC_ERROR]: "Une erreur inattendue s'est produite. Veuillez réessayer",
  },
  de: {
    [FrontendStripe.STRIPE_NOT_INITIALIZED]: "Das Zahlungssystem konnte nicht initialisiert werden",
    [FrontendStripe.MISSING_ELEMENTS]: "Fehler bei den Zahlungselementen",
    [FrontendStripe.UNKNOWN_PAYMENT_STATUS]: "Die Zahlung konnte nicht abgeschlossen werden",
    [FrontendStripe.PAYMENT_SUCCESS]: "Zahlung erfolgreich abgeschlossen!",
    [FrontendStripe.MISSING_BOOKING]: "Buchungs-ID nicht gefunden",
    [FrontendStripe.BOOKING_CONFIRM_ERROR]: "Es gab ein Problem bei der Bestätigung der Buchung",
    [FrontendStripe.RELATION_SAVE_ERROR]: "Es gab ein Problem beim Speichern der Zahlungsbeziehung",
    [FrontendStripe.UNCERTAIN_PAYMENT]: "Zahlungsstatus ungewiss. Bitte überprüfen Sie Ihr Konto",
    [FrontendStripe.CONNECTION_ERROR]: "Verbindungsfehler. Bitte versuchen Sie es erneut",
    [FrontendStripe.GENERIC_ERROR]: "Ein unerwarteter Fehler ist aufgetreten. Bitte versuchen Sie es erneut",
  },
  ar: {
    [FrontendStripe.STRIPE_NOT_INITIALIZED]: "تعذر تهيئة نظام الدفع",
    [FrontendStripe.MISSING_ELEMENTS]: "خطأ في عناصر الدفع",
    [FrontendStripe.UNKNOWN_PAYMENT_STATUS]: "تعذر إتمام الدفع",
    [FrontendStripe.PAYMENT_SUCCESS]: "تم إتمام الدفع بنجاح!",
    [FrontendStripe.MISSING_BOOKING]: "لم يتم العثور على معرف الحجز",
    [FrontendStripe.BOOKING_CONFIRM_ERROR]: "حدثت مشكلة أثناء تأكيد الحجز",
    [FrontendStripe.RELATION_SAVE_ERROR]: "حدثت مشكلة أثناء حفظ علاقة الدفع",
    [FrontendStripe.UNCERTAIN_PAYMENT]: "حالة الدفع غير مؤكدة. يرجى التحقق من حسابك",
    [FrontendStripe.CONNECTION_ERROR]: "خطأ في الاتصال. يرجى المحاولة مرة أخرى",
    [FrontendStripe.GENERIC_ERROR]: "حدث خطأ غير متوقع. يرجى المحاولة مرة أخرى",
  },
};

export function getErrorFrontStripe(code: FrontendStripe): string {
  const lang = getLangFromCookie();
  return FrontendStripeTranslations[lang]?.[code] || FrontendStripeTranslations["es"][code];
}
