import type { ApiError } from "@/services/globalHandler";

export type Teacher = {
  calLink: string;
  name: string;
  native_lang: string;
  url_image: string;
  description: string;
};

export interface ResponseAPI<T> {
  success: boolean;
  message?: string;
  data?: T;
  error?: string;
}

export interface Comment {
  author_uid?: string; // Usuario que comentó (opcional)
  name: string;
  timestamp: string;
  content: string;
  url_img: string;
  like?: number; // Opcional, valor por defecto puede ser 0
  reply?: Comment[]; // Opcional, valor por defecto puede ser []
  users_liked?: string[]; // Opcional, valor por defecto puede ser []
}

export interface SessionData {
  token: string;
  local_id: string;
  email?: string | null;
  name?: string | null;
  exp: number;
  picture?: string | null;
  email_verified: boolean;
  provider?: string | null;
}

// Resultado de operaciones que pueden fallar en la api
export type Result<T, E = ApiError> = { success: true; data: T } | { success: false; error: E };

export enum ApiErrorType {
  NETWORK_ERROR = "NETWORK_ERROR",
  AUTHENTICATION_ERROR = "AUTHENTICATION_ERROR",
  SESSION_EXPIRED = "SESSION_EXPIRED",
  SESSION_NOT_FOUND = "SESSION_NOT_FOUND",
  VALIDATION_ERROR = "VALIDATION_ERROR",
  SERVER_ERROR = "SERVER_ERROR",
  UNKNOWN_ERROR = "UNKNOWN_ERROR",
}
