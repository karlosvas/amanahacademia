import {
  type ResponseAPI,
  type Result,
  type Comment,
  type Teacher,
  type UserMerged,
  type EmailResend,
  type AddContactResponse,
  type UserRequest,
  type UpdateComment,
} from "@/types/bakend-types";
import { ApiErrorType } from "@/enums/enums";
import { ApiError, ErrorHandler } from "@/services/globalHandler";
import { auth } from "@/config/firebase";

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

  //////////////////// COMENTARIOS /////////////////////
  // Obtener todos los comentarios (GET)
  async getAllComments(): Promise<Result<Comment[]>> {
    try {
      const response = await fetch(`${this.baseUrl}/comments/all`, {
        method: "GET",
      });

      return this.handleResponse<Comment[]>(response);
    } catch (error) {
      return ResultUtils.error(
        new ApiError(ApiErrorType.NETWORK_ERROR, "Error de conexión", undefined, error as Error)
      );
    }
  }

  // Enviar comentario (POST)
  async postComment(comment: Comment): Promise<Result<Comment>> {
    const currentUser = auth.currentUser;
    const token = currentUser ? await currentUser.getIdToken() : null;
    let res = await fetch(`${this.baseUrl}/comments/add`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer " + token,
      },
      body: JSON.stringify(comment),
    });

    return this.handleResponse<Comment>(res);
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

  // Delete a comment (DELETE)
  async deleteComment(commentId: string): Promise<Result<void>> {
    const currentUser = auth.currentUser;

    if (!currentUser) {
      return ResultUtils.error(new ApiError(ApiErrorType.AUTHENTICATION_ERROR, "Usuario no autenticado"));
    }

    const token = await currentUser.getIdToken();
    let res = await fetch(`${this.baseUrl}/comments/del/${commentId}`, {
      method: "DELETE",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer " + token,
      },
    });
    return this.handleResponse<void>(res);
  }

  // Editar un comentario (PUT)
  async editComment(commentId: string, comment: UpdateComment): Promise<Result<Comment>> {
    const currentUser = auth.currentUser;
    const token = currentUser ? await currentUser.getIdToken() : null;
    let res = await fetch(`${this.baseUrl}/comments/edit/${commentId}`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer " + token,
      },
      body: JSON.stringify(comment),
    });
    return this.handleResponse<Comment>(res);
  }

  // Obtener un comentario con una id especifica (GET)
  async getCommentById(commentId: string): Promise<Result<Comment>> {
    const currentUser = auth.currentUser;
    const token = currentUser ? await currentUser.getIdToken() : null;
    let res = await fetch(`${this.baseUrl}/comments/${commentId}`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer " + token,
      },
    });
    return this.handleResponse<Comment>(res);
  }

  //////////////////// PROFESORES /////////////////////
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

  //////////////////// EMAIL /////////////////////
  // Enviar email de contacto (Resend) (POST)
  async sendContact(resendEmail: EmailResend): Promise<Result<Record<string, string>>> {
    let res = await fetch(`${this.baseUrl}/email/contact`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(resendEmail),
    });

    return this.handleResponse<Record<string, string>>(res);
  }

  //////////////////// USUARIOS /////////////////////

  // Registrar a un usuario
  async registerUser(userRequest: UserRequest): Promise<Result<UserMerged>> {
    let res = await fetch(`${this.baseUrl}/users/register`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(userRequest),
    });
    return this.handleResponse<UserMerged>(res);
  }

  // Logear a un usuario
  async loginUser(userRequest: UserRequest): Promise<Result<UserMerged>> {
    let res = await fetch(`${this.baseUrl}/users/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(userRequest),
    });
    return this.handleResponse<UserMerged>(res);
  }

  // Obtener el usuario actual (GET)
  async getUser(): Promise<Result<UserMerged>> {
    const currentUser = auth.currentUser;
    const token = currentUser ? await currentUser.getIdToken() : null;
    let url = `${this.baseUrl}/users/me`;
    let res = await fetch(url, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer " + token,
      },
    });
    return this.handleResponse<UserMerged>(res);
  }

  //////////////////// Mailchimp /////////////////////
  // Añadir usuarios a la newsletter
  async addContactToNewsletter(email: string): Promise<Result<AddContactResponse>> {
    let res = await fetch(`${this.baseUrl}/mailchimp/add_newsletter`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(email),
    });
    return this.handleResponse<AddContactResponse>(res);
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
