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
