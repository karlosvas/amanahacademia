import { ApiErrorType } from "@/enums/enums";

export class ApiError extends Error {
  constructor(
    public type: ApiErrorType,
    public message: string,
    public statusCode?: number,
    public originalError?: Error
  ) {
    super(message);
    this.name = "ApiError";
  }
}

// Global error handler
export class ErrorHandler {
  static handleApiError(error: ApiError): void {
    console.error(`[${error.type}] ${error.message}`, {
      statusCode: error.statusCode,
      originalError: error.originalError,
    });

    // Aquí puedes agregar notificaciones, logging, etc.
    switch (error.type) {
      case ApiErrorType.AUTHENTICATION_ERROR:
      case ApiErrorType.SESSION_EXPIRED:
        // Redirect to login
        window.location.href = "/login";
        break;
      case ApiErrorType.NETWORK_ERROR:
        // Show network error toast
        break;
      default:
        // Show generic error
        break;
    }
  }
}
