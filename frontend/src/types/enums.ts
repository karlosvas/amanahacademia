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

export function getErrorMessage(code: FrontendErrorCode) {
  const lang = getLangFromCookie();
  return FrontendErrorMessages[lang]?.[code] || FrontendErrorMessages["es"][code];
}
