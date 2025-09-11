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
  identification: {
    identification: IdentificationI18n;
    modal: ModalI18n;
  };
};

export type IdentificationI18n = {
  button: {
    login: string;
    logout: string;
  };
};

export type ModalI18n = {
  login: {
    title: string;
    subtitle: string;
    button: string;
    email: string;
    password: string;
    toggleModal: string;
    forgot_password: string;
  };
  register: {
    title: string;
    subtitle: string;
    info: string[];
    button: string;
    toggleModal: string;
  };
  utils: {
    loading: string;
    labels: {
      email: string;
      password: string;
      name: string;
    };
  };
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
    standard: CardPricingType;
    conversation: CardPricingType;
    group: CardPricingType;
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

// Props para SEO de los componentes
export interface PropsSEO {
  lang: string;
  title: string;
  description: string;
  canonical: string;
  ogImage: string;
  noindex: boolean; // true para páginas legales/internas
  keywords: string;
  structuredDataType?: "organization" | "course" | "webpage";
  structuredData?: Record<string, any>;
}

export interface StructureDataTypes {
  type: "organization" | "course" | "webpage";
  data: Record<string, any>;
}

export type PaymentPayload = {
  amount: number;
  currency: string;
  payment_method: string;
};

export interface SEOTranslations {
  [key: string]: {
    [page: string]: {
      title: string;
      description: string;
      keywords: string;
      structuredDataType: "organization" | "course" | "webpage";
      structuredData: Record<string, any>;
    };
  };
}

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
