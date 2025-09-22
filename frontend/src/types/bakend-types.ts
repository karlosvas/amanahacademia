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

export interface ProviderUserInfo {
  provider_id: string;
  federated_id?: string;
  email?: string;
  display_name?: string;
  photo_url?: string;
  raw_id?: string;
}

export interface UserMerged {
  local_id: string;
  email?: string;
  email_verified?: boolean;
  display_name?: string;
  photo_url?: string;
  phone_number?: string;
  disabled?: boolean;
  role?: string;
  subscription_tier?: string;
  permissions?: string[];
  provider_user_info?: ProviderUserInfo[];
  password_hash?: string;
  password_updated_at?: number;
  valid_since?: string;
  last_login_at?: string;
  created_at?: string;
  custom_auth?: boolean;
}

export interface EmailResend {
  from: string;
  name: string;
  to?: string[]; // Opcional, array de strings
  subject: string;
  text: string;
}
