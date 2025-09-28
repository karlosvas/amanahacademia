export enum Class {
  Standard = "standard-class", // ✅ Coincide con tu event type
  Conversacion = "conversation-class", // ✅ Coincide con tu event type
  Grupales = "group-class", // ✅ Coincide con tu event type
  Free = "free-class", // ✅ Coincide con tu event type
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
