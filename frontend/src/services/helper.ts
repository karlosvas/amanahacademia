import type {
  ResponseAPI,
  Result,
  Comment,
  Teacher,
  UserMerged,
  EmailResend,
  AddContactResponse,
  UserRequest,
  UpdateComment,
  ContactMailchimp,
  CheckoutPaymentIntentResponse,
  CheckoutPaymentIntentRequest,
  RelationalCalStripe,
  ReplyComment,
} from "@/types/bakend-types";
import { ApiErrorType } from "@/enums/enums";
import { ApiError } from "@/services/globalHandler";
import { getCurrentUserToken } from "@/services/firebase";

export class ApiService {
  private readonly baseUrl: string;

  constructor() {
    this.baseUrl = import.meta.env.PUBLIC_BACKEND_URL || "http://localhost:3000";
  }

  //////////////////// COMENTARIOS /////////////////////
  // Obtener todos los comentarios (GET)
  async getAllComments(): Promise<Result<Comment[]>> {
    try {
      const response = await fetch(`${this.baseUrl}/comments/all`, {
        method: "GET",
        headers: { "Content-Type": "application/json" },
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
    const token = await getCurrentUserToken();
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
  async setLike(commentId: string): Promise<Result<Comment>> {
    try {
      const token = await getCurrentUserToken();
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
    const token = await getCurrentUserToken();
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
    const token = await getCurrentUserToken();
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
    const token = await getCurrentUserToken();
    let res = await fetch(`${this.baseUrl}/comments/${commentId}`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer " + token,
      },
    });
    return this.handleResponse<Comment>(res);
  }

  // Crear una respuesta a un comentario (POST)
  async createReply(parentCommentId: string, content: ReplyComment): Promise<Result<ReplyComment>> {
    try {
      const token = await getCurrentUserToken();
      const response = await fetch(`${this.baseUrl}/comments/reply/${parentCommentId}`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer " + token,
        },
        body: JSON.stringify(content),
      });

      return this.handleResponse<ReplyComment>(response);
    } catch (error) {
      return ResultUtils.error(
        new ApiError(ApiErrorType.NETWORK_ERROR, "Error de conexión", undefined, error as Error)
      );
    }
  }

  // Editar una respuesta específica (PUT)
  async editReply(commentId: string, replyIndex: string, content: ReplyComment): Promise<Result<ReplyComment>> {
    try {
      const token = await getCurrentUserToken();
      const response = await fetch(`${this.baseUrl}/comments/reply/${commentId}/${replyIndex}/edit`, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer " + token,
        },
        body: JSON.stringify(content),
      });

      return this.handleResponse<ReplyComment>(response);
    } catch (error) {
      return ResultUtils.error(
        new ApiError(ApiErrorType.NETWORK_ERROR, "Error de conexión", undefined, error as Error)
      );
    }
  }

  // Eliminar una respuesta específica (DELETE)
  async deleteReply(parentCommentId: string, replyId: string): Promise<Result<void>> {
    try {
      const token = await getCurrentUserToken();
      const response = await fetch(`${this.baseUrl}/comments/del/${parentCommentId}/reply/${replyId}`, {
        method: "DELETE",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer " + token,
        },
      });

      return this.handleResponse<void>(response);
    } catch (error) {
      return ResultUtils.error(
        new ApiError(ApiErrorType.NETWORK_ERROR, "Error de conexión", undefined, error as Error)
      );
    }
  }

  // Obtener un reply con una id especifica (GET)
  async getCommentReplyById(commentId: string, replyId: string): Promise<Result<Comment>> {
    const token = await getCurrentUserToken();
    let res = await fetch(`${this.baseUrl}/comments/${commentId}/reply/${replyId}`, {
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

  //////////////////// RESEND /////////////////////
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
  async registerUser(userRequest: UserRequest): Promise<Result<string>> {
    let res = await fetch(`${this.baseUrl}/users/register`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(userRequest),
    });
    return this.handleResponse<string>(res);
  }

  // Logear a un usuario
  async loginUser(userRequest: UserRequest): Promise<Result<string>> {
    let res = await fetch(`${this.baseUrl}/users/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(userRequest),
    });
    return this.handleResponse<string>(res);
  }

  // Obtener el usuario actual (GET)
  async getUser(): Promise<Result<UserMerged>> {
    const token = await getCurrentUserToken();
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

  //////////////////// MAILCHIMP /////////////////////
  // Añadir usuarios a la newsletter
  async addContactToNewsletter(contactMailchimp: ContactMailchimp): Promise<Result<AddContactResponse>> {
    let res = await fetch(`${this.baseUrl}/mailchimp/add_contact`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(contactMailchimp),
    });
    return this.handleResponse<AddContactResponse>(res);
  }

  //////////////////// STRIPE /////////////////////
  // Payment intent para la sesiond e firebase
  async checkout(payload: CheckoutPaymentIntentRequest): Promise<Result<CheckoutPaymentIntentResponse>> {
    const token = await getCurrentUserToken();
    let url = `${this.baseUrl}/payment/intent`;

    // Crear Payment Intent en el backend
    const response = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(payload),
    });

    return this.handleResponse<CheckoutPaymentIntentResponse>(response);
  }

  // Guardar la relacion entre cal.com y stripe en firebase
  async saveCalStripeConnection(payload: RelationalCalStripe): Promise<Result<void>> {
    const token = await getCurrentUserToken();
    const response = await fetch(`${this.baseUrl}/payment/cal/connection`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(payload),
    });
    return this.handleResponse<void>(response);
  }

  //////////////////// CAL.COM ////////////////////
  // Confirmación del booking al pagar
  async confirmBooking(bookingUid: string): Promise<Result<void>> {
    const token = await getCurrentUserToken();
    const response = await fetch(`${this.baseUrl}/cal/bookings/${bookingUid}/confirm`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
    });

    return this.handleResponse<void>(response);
  }

  //////////////////// Utilidades de respuesta ////////////////////
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
        message = "Not authorized";
        break;
      case 404:
        errorType = ApiErrorType.SESSION_NOT_FOUND;
        message = "Session not found";
        break;
      case 422:
        errorType = ApiErrorType.VALIDATION_ERROR;
        message = "Invalid data";
        break;
      case 500:
        errorType = ApiErrorType.SERVER_ERROR;
        message = "Internal server error";
        break;
      default:
        errorType = ApiErrorType.UNKNOWN_ERROR;
        message = `HTTP error ${response.status}`;
    }

    return ResultUtils.error(new ApiError(errorType, message, response.status));
  }
}

export class ResultUtils {
  static ok<T>(data: T): Result<T> {
    return { success: true, data };
  }

  static error<E>(error: E): Result<never, E> {
    return { success: false, error };
  }

  static getErrorType<T>(result: Result<T, ApiError>): ApiErrorType | null {
    if (!result.success && result.error instanceof ApiError) {
      return result.error.type;
    }
    return null;
  }
}

export const apiService = new ApiService();
