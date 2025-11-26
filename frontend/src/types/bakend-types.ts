export type Teacher = {
  uid?: string;
  cal_id: string;
  cal_link: string;
  name: string;
  native_lang: string;
  other_lang: string[];
  url_image: string;
  description: string[];
};

export interface Comment {
  id?: string;
  author_uid?: string;
  name: string;
  timestamp: string;
  content: string;
  url_img?: string;
  stars: number;
  like?: number;
  reply?: ReplyComment[];
  users_liked?: string[];
}

export interface UpdateComment {
  content: string;
  stars: number;
}

export type ResponseAPI<T> = { success: true; data: T } | { success: false; error: string };

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
  first_free_class: boolean;
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

export interface AddContactResponse {
  id: string;
  email_address: string;
  status: string;
}

export type ProviderType = "email" | "google";

export type UserRequest = {
  // Datos obligatorios requeridos por firebase auth
  email: string;
  password: string;
  name?: string;
  provider: ProviderType;
  first_free_class: boolean;

  // Datos opcionales que el cliente puede enviar
  phone_number?: string;
  id_token?: string; // Token JWT de Firebase Auth (required for Google provider)

  // Datos específicos para la DB
  role?: string; // Si tienes un enum ROLE, puedes usarlo aquí
  permissions?: string[];
  subscription_tier?: string;
};

export type UserRequestGoogle = {
  // Datos obligatorios requeridos por firebase auth
  email: string;
  password: string;

  // Datos opcionales que el cliente puede enviar
  name: string;
  phone_number?: string;

  // Datos específicos para la DB
  role?: string; // Si tienes un enum ROLE, puedes usarlo aquí
  permissions?: string[];
  subscription_tier?: string;
};

export interface ContactMailchimp {
  email_address: string;
  status: string;
  merge_fields?: MergeFields;
}

export interface MergeFields {
  FNAME?: string;
  LNAME?: string;
}

export interface CheckoutPaymentIntentResponse {
  client_secret: string;
  status: string;
  error?: string;
}

export interface CheckoutPaymentIntentRequest {
  amount: number;
  currency: string;
}

export interface RelationalCalStripe {
  cal_id: string;
  stripe_id: string;
}

export interface ReplyComment {
  id: string;
  author_uid: string; // author_uid
  name: string;
  timestamp: string;
  content: string;
  url_img?: string | null;
  like?: number;
  users_liked?: string[];
}

export interface Booking {
  uid: string;
  bookingId: string | null;
  eventTypeId: string | null;
  type: string | null;

  title: string;
  description: string | null;

  startTime: string | null; // ISO date
  endTime: string | null; // ISO date

  attendees: BookingAttendee[];
  organizer: BookingOrganizer | null;

  location: string | null;
  metadata: Record<string, unknown> | null;
  status: BookingStatus;
  cancellationReason: string | null;
}

export interface BookingAttendee {
  email: string;
  name: string;
  timeZone: string;
  language: {
    locale: string;
  };
}

export interface BookingOrganizer {
  email?: string | null;
  name?: string | null;
}

export type BookingStatus = "accepted" | "pending" | "cancelled" | "rejected";

export interface StripeRelation {
  stripe_id: string;
}

export interface PaymentIntentSimplified {
  id: string;
  amount: number;
  currency: string;
  status: string;
  created: number;
  description?: string;
  metadata: Record<string, string>;
  payment_method_types: string[];
}
