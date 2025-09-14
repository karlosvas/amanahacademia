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
