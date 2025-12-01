import type {
  ResponseAPI,
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
  StripeRelation,
  PaymentIntentSimplified,
  BookingRequest,
  Schedule,
  CalBookingPayload,
} from "@/types/bakend-types";
import { getCurrentUserToken } from "@/services/firebase";
import type { MetricsResponse } from "@/types/types";
import { log } from "./logger";

export class ApiService {
  private readonly baseUrl: string;

  constructor() {
    this.baseUrl = import.meta.env.PUBLIC_BACKEND_URL || "http://localhost:3000";
  }

  //////////////////// COMENTARIOS /////////////////////
  // Obtener todos los comentarios (GET)
  async getAllComments(): Promise<ResponseAPI<Comment[]>> {
    return this.fetchApi<Comment[]>("/comments/all", { method: "GET" });
  }

  // Enviar comentario (POST)
  async postComment(comment: Comment): Promise<ResponseAPI<Comment>> {
    const token = await getCurrentUserToken();
    return this.fetchApi<Comment>("/comments/add", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(comment),
    });
  }

  // Darle like a un comentario (PUT)
  async setLike(commentId: string): Promise<ResponseAPI<Comment>> {
    const token = await getCurrentUserToken();
    return this.fetchApi<Comment>(`/comments/like/${commentId}`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
    });
  }

  // Delete a comment (DELETE)
  async deleteComment(commentId: string): Promise<ResponseAPI<void>> {
    const token = await getCurrentUserToken();
    return this.fetchApi<void>(`/comments/del/${commentId}`, {
      method: "DELETE",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
    });
  }

  // Editar un comentario (PUT)
  async editComment(commentId: string, comment: UpdateComment): Promise<ResponseAPI<Comment>> {
    const token = await getCurrentUserToken();
    return this.fetchApi<Comment>(`/comments/edit/${commentId}`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(comment),
    });
  }

  // Obtener un comentario con una id especifica (GET)
  async getCommentById(commentId: string): Promise<ResponseAPI<Comment>> {
    const token = await getCurrentUserToken();
    return this.fetchApi<Comment>(`/comments/${commentId}`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
    });
  }

  // Crear una respuesta a un comentario (POST)
  async createReply(parentCommentId: string, content: ReplyComment): Promise<ResponseAPI<ReplyComment>> {
    const token = await getCurrentUserToken();
    return this.fetchApi<ReplyComment>(`/comments/reply/${parentCommentId}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(content),
    });
  }

  // Editar una respuesta específica (PUT)
  async editReply(commentId: string, replyIndex: string, content: ReplyComment): Promise<ResponseAPI<ReplyComment>> {
    const token = await getCurrentUserToken();
    return this.fetchApi<ReplyComment>(`/comments/reply/${commentId}/${replyIndex}/edit`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(content),
    });
  }

  // Eliminar una respuesta específica (DELETE)
  async deleteReply(parentCommentId: string, replyId: string): Promise<ResponseAPI<void>> {
    const token = await getCurrentUserToken();
    return this.fetchApi<void>(`/comments/del/${parentCommentId}/reply/${replyId}`, {
      method: "DELETE",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
    });
  }

  // Obtener un reply con una id especifica (GET)
  async getCommentReplyById(commentId: string, replyId: string): Promise<ResponseAPI<Comment>> {
    const token = await getCurrentUserToken();
    return this.fetchApi<Comment>(`/comments/${commentId}/reply/${replyId}`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
    });
  }

  //////////////////// PROFESORES /////////////////////
  // Obtener un profesor por su ID (GET)
  async getTeacher(teacher: string): Promise<ResponseAPI<Teacher>> {
    return this.fetchApi<Teacher>(`/teachers/${teacher}`, {
      method: "GET",
      headers: { "Content-Type": "application/json" },
    });
  }

  // Obtener todos los profesores (GET)
  async getTeachers(): Promise<ResponseAPI<Teacher[]>> {
    return this.fetchApi<Teacher[]>("/teachers/all", {
      method: "GET",
      headers: { "Content-Type": "application/json" },
    });
  }

  // Obtener un calendario especifico
  async getAvailableTimeSchedule(id: string): Promise<ResponseAPI<Schedule>> {
    return this.fetchApi<Schedule>(`/cal/schedule/${id}`, {
      method: "GET",
      headers: { "Content-Type": "application/json" },
    });
  }

  //////////////////// RESEND /////////////////////
  // Enviar email de contacto (Resend) (POST)
  async sendContact(resendEmail: EmailResend): Promise<ResponseAPI<Record<string, string>>> {
    return this.fetchApi<Record<string, string>>("/email/contact", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(resendEmail),
    });
  }

  //////////////////// USUARIOS /////////////////////

  // Registrar a un usuario
  async registerUser(userRequest: UserRequest): Promise<ResponseAPI<string>> {
    return this.fetchApi<string>("/users/register", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(userRequest),
    });
  }

  // Logear a un usuario
  async loginUser(userRequest: UserRequest): Promise<ResponseAPI<string>> {
    return this.fetchApi<string>("/users/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(userRequest),
    });
  }

  // Obtener el usuario actual (GET)
  async getUser(): Promise<ResponseAPI<UserMerged>> {
    const token = await getCurrentUserToken();

    return this.fetchApi<UserMerged>("/users/me", {
      method: "GET",
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
  }

  // Comprobar si el usuario es admin (GET)
  async isAdminUser(token_cookie: string): Promise<ResponseAPI<boolean>> {
    const token = (await getCurrentUserToken()) || token_cookie;
    return this.fetchApi<boolean>("/users/admin_check", {
      method: "GET",
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
  }

  //////////////////// MAILCHIMP /////////////////////
  // Añadir usuarios a la newsletter
  async addContactToNewsletter(contactMailchimp: ContactMailchimp): Promise<ResponseAPI<AddContactResponse>> {
    return this.fetchApi<AddContactResponse>("/mailchimp/add_contact", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(contactMailchimp),
    });
  }

  //////////////////// STRIPE /////////////////////
  // Payment intent para la sesion de firebase
  async checkout(payload: CheckoutPaymentIntentRequest): Promise<ResponseAPI<CheckoutPaymentIntentResponse>> {
    const token = await getCurrentUserToken();
    return this.fetchApi<CheckoutPaymentIntentResponse>("/payment/intent", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(payload),
    });
  }

  // Guardar la relacion entre cal.com y stripe en firebase
  async saveCalStripeConnection(payload: RelationalCalStripe): Promise<ResponseAPI<void>> {
    const token = await getCurrentUserToken();
    return this.fetchApi<void>("/payment/cal/connection", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(payload),
    });
  }

  // Obtener el historial de reservas pagadas
  async getPaidReservations(token_cookie: string): Promise<ResponseAPI<PaymentIntentSimplified[]>> {
    const token = token_cookie || (await getCurrentUserToken());
    return this.fetchApi<PaymentIntentSimplified[]>("/payment/history", {
      method: "GET",
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
  }

  // Obtener conexiones cal.com - stripe
  async getAllConnections(token_cookie: string): Promise<ResponseAPI<StripeRelation[]>> {
    const token = token_cookie || (await getCurrentUserToken());
    return this.fetchApi<StripeRelation[]>("/payment/cal/connection/all", {
      method: "GET",
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
  }

  //////////////////// CAL.COM ////////////////////
  // Confirmación del booking al pagar
  async confirmBooking(bookingUid: string): Promise<ResponseAPI<void>> {
    const token = await getCurrentUserToken();
    return this.fetchApi<void>(`/cal/bookings/${bookingUid}/confirm`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
    });
  }

  // Obtener booking por id
  async getBookingById(bookingUid: string, tooken_cookie?: string): Promise<ResponseAPI<CalBookingPayload>> {
    const token = tooken_cookie || (await getCurrentUserToken());
    return this.fetchApi<CalBookingPayload>(`/cal/bookings/${bookingUid}`, {
      method: "GET",
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
  }

  // Crear un booking
  async createBooking(booking: BookingRequest, token_cookie?: string): Promise<ResponseAPI<CalBookingPayload>> {
    const token = token_cookie || (await getCurrentUserToken());
    const data = await fetch(`${this.baseUrl}/cal/bookings`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(booking),
    });

    log.debug("Respuesta raw de createBooking:", data);

    return data.json();
  }

  // Obtener todos los bookings de grupo
  async getGroupBookings(token_cookie?: string): Promise<ResponseAPI<CalBookingPayload[]>> {
    const token = token_cookie || (await getCurrentUserToken());
    return this.fetchApi<CalBookingPayload[]>("/cal/bookings", {
      method: "GET",
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
  }

  ///////////// Google Analytics ////////////////
  async getUserMetrics(cookie_token?: string): Promise<ResponseAPI<MetricsResponse>> {
    const token = cookie_token || (await getCurrentUserToken());
    return this.fetchApi<MetricsResponse>("/metrics/users", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
    });
  }

  async getArticlesMetrics(): Promise<ResponseAPI<MetricsResponse>> {
    const token = await getCurrentUserToken();
    return this.fetchApi<MetricsResponse>("/metrics/articles", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
    });
  }

  async getClassMetrics(): Promise<ResponseAPI<MetricsResponse>> {
    const token = await getCurrentUserToken();
    return this.fetchApi<MetricsResponse>("/metrics/class", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
    });
  }

  //////////////////// Método privado unificado ////////////////////
  /**
   * Método privado para hacer fetch y retornar la respuesta del backend sin transformaciones.
   * Mantiene exactamente la estructura { success: boolean, data?: T, error?: string }
   */
  private async fetchApi<T>(endpoint: string, options: RequestInit): Promise<ResponseAPI<T>> {
    try {
      const response = await fetch(`${this.baseUrl}${endpoint}`, options);

      // Si es 204 No Content, retornar éxito sin data
      if (response.status === 204) {
        return { success: true, data: undefined as T };
      }

      // Obtener el contenido como texto primero
      const text = await response.text();

      // Intentar parsear como JSON
      try {
        const data: ResponseAPI<T> = JSON.parse(text);
        return data;
      } catch (parseError) {
        // Si no es JSON válido, retornar el texto como error
        console.error("Failed to parse JSON response:", text);
        return {
          success: false,
          error: `Invalid JSON response: ${text.substring(0, 200)}`,
        };
      }
    } catch (error) {
      // Solo capturar errores de red
      console.error("Network error:", error);
      return {
        success: false,
        error: error instanceof Error ? `Network error: ${error.message}` : "Unknown network error",
      };
    }
  }
}

export const apiService = new ApiService();
