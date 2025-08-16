export interface i18nTranslations {
  header: HeaderIn18;
  main: MainIn18;
}

export type MainIn18 = {
  title: string;
  features: string;
  testimonials: string;
};

export type HeaderIn18 = {
  about_us: string;
  articles: string;
  pricing: string;
  courses: string;
  comments: string;
  contact: string;
};
