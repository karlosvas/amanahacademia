// I18n traducciones
export interface I18nTranslations {
  header: HeaderI18n;
  main: MainI18n;
  footer: FooterI18n;
  security: SecurityHallOfFameI18n;
  info: InfoI18n;
}

// Seccion del header
export type HeaderI18n = {
  about_us: string;
  articles: string;
  pricing: string;
  courses: string;
  comments: string;
  contact: string;
  identification: {
    button: {
      login: string;
      logout: string;
    };
    modal: ModalI18n;
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
      },
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
    },
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
};

export type CardModuleType = {
  sessions: [number, string];
  homeworks: [number, string];
  title: string;
  description: string;
  content: string[];
};

export type CourseI18n = {
  title: string;
  summary: string[];
  ideal_for: {
    title: string;
    title_color: string;
    points: {
      title: string;
      description: string;
    }[];
  };
  modules: CardModuleType[];
};

export type ModalComent = {
  title: string;
  description: string;
  submit: string;
  cleanup: string;
  placeholder: string;
};

export type CommentI18n = {
  best: string;
  button: string;
  modal: ModalComent;
  loading: string;
  evaluations: {
    title: string;
    types: string[];
  };
};

export type FooterI18n = {
  license: string;
  privacy_policy: string;
  terms_of_service: string;
};

// Props para SEO de los componentes
export type StructuredDataType = "organization" | "course" | "webpage";
export interface PropsSEO {
  lang: string;
  title: string;
  description: string;
  canonical: string;
  ogImage: string;
  noindex: boolean; // true para páginas legales/internas
  keywords: string;
  structuredDataType?: StructuredDataType;
  structuredData?: Record<string, any>;
}

export interface StructureDataTypes {
  type: StructuredDataType;
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

export type SecurityHallOfFameI18n = {
  title: string;
  subtitle: string;
  intro: string;
  examples_title: string;
  researchers: Array<{
    name: string;
    severity: string;
    title: string;
    finding_title: string;
    finding_desc: string;
  }>;
  thank_you_title: string;
  thank_you_desc: string;
};

// Tipo principal que representa toda la estructura
export interface InfoI18n {
  header: {
    title: string;
    subtitle: string;
  };
  privacyPolicy: {
    title: string;
    sections: {
      informationCollection: {
        title: string;
        content: string;
      };
      dataProtection: {
        title: string;
        content: string;
      };
      communications: {
        title: string;
        content: string;
      };
      cookies: {
        title: string;
        content: string;
      };
      policyChanges: {
        title: string;
        content: string;
      };
    };
  };
  termsConditions: {
    title: string;
    sections: {
      platformUse: {
        title: string;
        content: string;
      };
      intellectualProperty: {
        title: string;
        content: string;
      };
      liabilityLimitation: {
        title: string;
        content: string;
      };
      paymentsSubscriptions: {
        title: string;
        content: string;
      };
      applicableLaw: {
        title: string;
        content: string;
      };
    };
  };
  license: {
    title: string;
    sections: {
      usageLicense: {
        title: string;
        content: string;
      };
      restrictions: {
        title: string;
        content: string;
        button: string;
      };
    };
  };
  securityHallOfFame: {
    title: string;
    content: string;
    button: string;
  };
}

export type AvatarProps = {
  name: string;
  url_img: string;
};

export type PricingApiResponse = {
  country: string;
  currency: string;
  symbol: string;
  level: "high" | "low";
  countryGroup: string;
  isDevelopment: boolean;
  prices: {
    individual_standard: number;
    individual_conversation: number;
    group: number;
  };
};

export interface CalPricingResponse {
  prices: {
    individual_standard: number;
    individual_conversation: number;
    group: number;
  };
  symbol: string;
  country?: string;
  // Añadir precios para Cal.com (en centavos)
  cal_standard: number;
  cal_conversation: number;
  cal_group: number;
}

export interface BookingPaymentRequest {
  amount: number;
  currency: string;
  payment_method: string;
  event_type_id: number;
  start_time: string; // ISO 8601
  attendee_name: string;
  attendee_email: string;
  attendee_timezone: string;
  attendee_phone?: string;
}

export type Articles = {
  title: string;
  pdfUrl: string;
  image: string;
};

export interface ArticlesAstro {
  id: string;
  data: Articles;
  filePath: string;
  digest: string;
  collection: string;
}

export type Lang = "es" | "en" | "fr" | "de" | "ar" | "it" | "pt";
