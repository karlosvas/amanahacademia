// I18n
export interface i18nTranslations {
  header: HeaderI18n;
  main: MainI18n;
  footer: FooterI18n;
}

export type HeaderI18n = {
  about_us: string;
  articles: string;
  pricing: string;
  courses: string;
  comments: string;
  contact: string;
};

export type MainI18n = {
  home: HomeI18n;
  articles: ArticleI18n;
  pricing: PricingI18n;
  courses: CourseI18n;
  comments: CommentI18n;
  contact: ContactI18n;
};

export type HomeI18n = {
  title: string;
  about_us: {
    title: string;
    description: string[];
  };
  info: {
    students: string;
    teachers: string;
    satisfaction: string;
  };
  focus: {
    title: string;
    cards: [
      {
        title: string;
        description: string;
      },
      {
        title: string;
        description: string;
      },
      {
        title: string;
        description: string;
      }
    ];
    note: string[];
  };
};

export type CardPricingType = {
  title: string;
  description: string;
  price: {
    currency: string;
    amount: number;
    time: string;
  };
  content: string[];
  button: string;
};

export type PricingI18n = {
  title: string;
  type: {
    estandar: CardPricingType;
    conversation: CardPricingType;
    grupales: CardPricingType;
  };
  info: [
    {
      title: string;
      description: string;
      button: string;
    },
    {
      title: string;
      description: string;
    }
  ];
};

export type ContactI18n = {
  title: string;
  form: {
    name: string;
    subject: string;
    message: string;
    button: string;
  };
  etsy: {
    title: string;
    description: string;
  };
  podcast: {
    title: string;
    description: string;
  };
  call: {
    title: string;
    description: string;
  };
};

export type ArticleI18n = {
  title: string;
  summary: string[];
  articles: [
    {
      title: string;
      summary: string[];
    }
  ];
};

export type CourseI18n = {
  title: string;
  summary: string[];
  ideal_for: {
    title: string;
    title_color: string;
    points: [
      {
        title: string;
        description: string;
      }
    ];
  };
  modules: {
    title: string;
    title_color: string;
    content: [
      {
        title: string;
        description: string;
        icons: {
          session: string;
          book: string;
        };
      }
    ];
  };
};

export type CommentI18n = {
  title: string;
  summary: string[];
};

export type FooterI18n = {
  license: string;
  privacy_policy: string;
  terms_of_service: string;
};

export interface CardModuleProps {
  sessons: number;
  homeworks: number;
  modulo: number;
  content: string[];
  url: string;
}
