import { type ResponseAPI, type Result, type Comment, type Teacher } from "@/types/bakend-types";
import { ApiErrorType } from "@/enums/enums";
import { ApiError, ErrorHandler } from "@/services/globalHandler";

export class ApiService {
  private readonly baseUrl: string;

  constructor() {
    this.baseUrl = import.meta.env.PUBLIC_BACKEND_URL || "http://localhost:3000";
  }

  // Helper privado para manejar respuestas
  private async handleResponse<T>(response: Response): Promise<Result<T>> {
    try {
      if (!response.ok) {
        return this.handleHttpError(response);
      }

      // Para 204 No Content (CREATE/DELETE)
      if (response.status === 204) {
        return ResultUtils.ok(null as T);
      }

      const data: ResponseAPI<T> = await response.json();

      if (data.success && data.data !== undefined) {
        return ResultUtils.ok(data.data);
      } else {
        return ResultUtils.error(
          new ApiError(ApiErrorType.SERVER_ERROR, data.error || data.message || "Error del servidor")
        );
      }
    } catch (error) {
      return ResultUtils.error(
        new ApiError(ApiErrorType.NETWORK_ERROR, "Error de red o parsing", undefined, error as Error)
      );
    }
  }

  // Helper privado para manejar errores HTTP
  private handleHttpError<T>(response: Response): Result<T> {
    let errorType: ApiErrorType;
    let message: string;

    switch (response.status) {
      case 401:
        errorType = ApiErrorType.AUTHENTICATION_ERROR;
        message = "No autorizado";
        break;
      case 404:
        errorType = ApiErrorType.SESSION_NOT_FOUND;
        message = "Sesión no encontrada";
        break;
      case 422:
        errorType = ApiErrorType.VALIDATION_ERROR;
        message = "Datos inválidos";
        break;
      case 500:
        errorType = ApiErrorType.SERVER_ERROR;
        message = "Error interno del servidor";
        break;
      default:
        errorType = ApiErrorType.UNKNOWN_ERROR;
        message = `Error HTTP ${response.status}`;
    }

    return ResultUtils.error(new ApiError(errorType, message, response.status));
  }

  // Obtener todos los comentarios (GET)
  async getAllComments(): Promise<Result<Comment[]>> {
    try {
      const response = await fetch(`${this.baseUrl}/comments/all`, {
        method: "GET",
        credentials: "include",
      });

      return this.handleResponse<Comment[]>(response);
    } catch (error) {
      return ResultUtils.error(
        new ApiError(ApiErrorType.NETWORK_ERROR, "Error de conexión", undefined, error as Error)
      );
    }
  }

  // Darle like a un comentario (PUT)
  async setLike(token: string, commentId: string): Promise<Result<Comment>> {
    try {
      const response = await fetch(`${this.baseUrl}/comments/like/${commentId}`, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer " + token,
        },
      });

      return this.handleResponse<Comment>(response);
    } catch (error) {
      return ResultUtils.error(
        new ApiError(ApiErrorType.NETWORK_ERROR, "Error de conexión", undefined, error as Error)
      );
    }
  }

  // Obtener un profesor por su ID (GET)
  async getTeacher(teacher: string): Promise<Result<Teacher>> {
    let res = await fetch(`${this.baseUrl}/teachers/${teacher}`, {
      method: "GET",
      headers: { "Content-Type": "application/json" },
    });
    return this.handleResponse<Teacher>(res);
  }

  // Obtener todos los profesores (GET)
  async getTeachers(): Promise<Result<Teacher[]>> {
    let res = await fetch(`${this.baseUrl}/teachers/all`, {
      method: "GET",
      headers: { "Content-Type": "application/json" },
    });
    return this.handleResponse<Teacher[]>(res);
  }

  // Método helper para manejo automático de errores
  async executeWithErrorHandling<T>(operation: () => Promise<Result<T>>): Promise<T | null> {
    const result = await operation();

    if (ResultUtils.isOk(result)) {
      return result.data;
    } else {
      ErrorHandler.handleApiError(result.error);
      return null;
    }
  }
}

class ResultUtils {
  static ok<T>(data: T): Result<T> {
    return { success: true, data };
  }

  static error<E>(error: E): Result<never, E> {
    return { success: false, error };
  }

  static isOk<T, E>(result: Result<T, E>): result is { success: true; data: T } {
    return result.success;
  }

  static isError<T, E>(result: Result<T, E>): result is { success: false; error: E } {
    return !result.success;
  }
}

export const apiService = new ApiService();
