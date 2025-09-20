import type { ApiError } from "@/services/globalHandler";

export type Teacher = {
  uid?: string;
  calLink: string;
  name: string;
  native_lang: string;
  url_image: string;
  description: string;
  data_cal_teacher?: string; // ✅ Opcional, si lo usas
};

export interface ResponseAPI<T> {
  success: boolean;
  message?: string;
  data?: T;
  error?: string;
}

export interface Comment {
  author_uid?: string;
  name: string;
  timestamp: string;
  content: string;
  url_img?: string; // ✅ Opcional
  stars?: number; // ✅ Opcional, si lo usas
  like?: number; // Opcional
  reply?: Comment[]; // Opcional
  users_liked?: string[]; // Opcional
}

// Resultado de operaciones que pueden fallar en la api
export type Result<T, E = ApiError> = { success: true; data: T } | { success: false; error: E };
