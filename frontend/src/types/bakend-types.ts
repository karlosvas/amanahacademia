/// Teachers
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

/// Comments
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

export interface UpdateComment {
  content: string;
  stars: number;
}

/// Users
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

/// Resend
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

// Mailchimp
export interface ContactMailchimp {
  email_address: string;
  status: string;
  merge_fields?: MergeFields;
}

export interface MergeFields {
  FNAME?: string;
  LNAME?: string;
}

/// Stripe
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

// Cal.com
export interface BookingRequest {
  startTime: string;
  endTime: string;
  eventTypeId?: number;
  type?: string;
  username?: string;
  teamSlug?: string;
  organizationSlug?: string;
  title?: string;
  description?: string;
  attendees: Attendee[];
  location?: string;
  metadata?: Record<string, string>;
  status: string;
}

export interface Attendee {
  name: string;
  email: string;
  timeZone?: string;
  language?: string;
  phoneNumber?: string;
  absent?: boolean;
  locale?: string;
}

export interface Schedule {
  id: number;
  owner_id: number;
  name: string;
  time_zone: string;
  availability: Availability[];
  is_default: boolean;
  overrides: any[];
}

export type Availability = {
  days: string[];
  start_time: string;
  end_time: string;
};

/// Bookings
/**
 * Representa el estado de una reserva en Cal.com.
 * El API utiliza mayúsculas separadas por guiones bajos (SNAKE_CASE).
 */
export type BookingStatus = "ACCEPTED" | "CANCELLED" | "PENDING" | "REJECTED" | "UNKNOWN";

/**
 * Representa a un asistente de la reserva (estudiante o huésped).
 */
export interface Attendee {
  /** Nombre completo del asistente. */
  name: string;
  /** Dirección de correo electrónico. */
  email: string;
  /** Zona horaria del asistente (ej: "Europe/Madrid"). */
  timeZone: string;
  /** Idioma preferido (ej: "es"). */
  locale: string;
  /** Rol del asistente (ej: "booker"). */
  role?: string;
}

/**
 * Representa al organizador (anfitrión/profesor) de la reserva.
 */
export interface Organizer {
  /** Nombre completo del organizador. */
  name: string;
  /** Dirección de correo electrónico. */
  email: string;
  /** Identificador numérico del organizador en Cal.com. */
  id: number;
  /** Username del organizador. */
  username: string;
  /** Zona horaria del organizador. */
  timeZone: string;
}

export interface UserCal {
  id: number;
  username: string;
  email: string;
  timeZone?: string;
}

export interface EventTypeCal {
  id: number;
  slug: string;
  title?: string;
}

/**
 * Payload que representa una reserva completa de Cal.com, usado tanto en
 * respuestas de API como en webhooks.
 */
export interface CalBookingPayload {
  /** Identificador único de la reserva en Cal.com. Opcional en la creación. */
  uid?: string;

  /** ID numérico de la reserva en Cal.com (bookingId). */
  bookingId?: number;

  /** Información del tipo de evento asociado a la reserva. */
  eventType?: EventTypeCal;

  /** ID del tipo de evento (eventTypeId). */
  eventTypeId?: number;

  /** Slug del tipo de evento (mapeado desde el campo 'type' en el payload JSON). */
  type?: string;

  /** Username del usuario/organizador. */
  user?: UserCal;

  /** Slug del equipo (teamSlug). */
  teamSlug?: string;

  /** Slug de la organización (organizationSlug). */
  organizationSlug?: string;

  /** Título descriptivo de la reserva. */
  title?: string;

  /** Descripción del evento. */
  description?: string;

  /**
   * Fecha y hora de inicio de la reserva en formato ISO 8601.
   * Acepta 'startTime' y 'start' en JSON.
   */
  startTime?: string;

  /**
   * Fecha y hora de finalización de la reserva en formato ISO 8601.
   * Acepta 'endTime' y 'end' en JSON.
   */
  endTime?: string;

  /** Duración de la reserva en minutos. */
  duration?: number;

  /** Lista de asistentes a la reserva. */
  attendees: Attendee[];

  /** Información del organizador (anfitrión). */
  organizer?: Organizer;

  /** Ubicación/URL de la videollamada. */
  location?: string;

  /** Metadatos adicionales de la reserva (estructura flexible). */
  metadata?: Record<string, any>; // Usamos Record<string, any> para el objeto JSON flexible

  /** Estado actual de la reserva (ACCEPTED, CANCELLED, PENDING, REJECTED). */
  status: BookingStatus;

  /** Razón de cancelación si la reserva fue cancelada (cancellationReason). */
  cancellationReason?: string;

  /** URL de la reunión (meetingUrl). */
  meetingUrl?: string;

  /** Token de cancelación (cancelToken). */
  cancelToken?: string;

  /** Token de reagendamiento (rescheduleToken). */
  rescheduleToken?: string;
}

// API implementacion
export type ResponseAPI<T> = { success: true; data: T } | { success: false; error: string };
