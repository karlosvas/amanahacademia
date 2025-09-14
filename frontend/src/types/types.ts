// I18n
export interface i18nTranslations {
  header: HeaderI18n;
  main: MainI18n;
  footer: FooterI18n;
  security: SecurityHallOfFameI18n;
  info: InfoI18n;
}

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

interface PrivacyPolicyItem {
  title: string;
  description: string;
}

interface ContactItem {
  type: string;
  value: string;
  link?: string;
}

interface InfoPrivacyPolicySection {
  informationCollection: {
    title: string;
    description: string;
    items: PrivacyPolicyItem[];
  };
  informationUsage: {
    title: string;
    description: string;
    items: string[];
  };
  dataProtection: {
    title: string;
    description: string;
    items: PrivacyPolicyItem[];
    additionalInfo: string;
  };
  userRights: {
    title: string;
    description: string;
    items: PrivacyPolicyItem[];
    contactInfo: string;
    email: string;
  };
  communications: {
    title: string;
    description: string;
    items: string[];
    unsubscribeInfo: string;
  };
  thirdPartySharing: {
    title: string;
    description: string;
    items: PrivacyPolicyItem[];
    noSellingInfo: string;
  };
  cookies: {
    title: string;
    description: string;
    items: string[];
    managementInfo: string;
  };
  internationalTransfers: {
    title: string;
    description: string;
    additionalInfo: string;
  };
  dataRetention: {
    title: string;
    description: string;
    additionalInfo: string;
  };
  policyChanges: {
    title: string;
    description: string;
    additionalInfo: string;
  };
  contact: {
    title: string;
    description: string;
    items: ContactItem[];
  };
}

interface InfoTermsAndConditionsSection {
  platformUsage: {
    title: string;
    description: string;
    items: string[];
    consequence: string;
  };
  intellectualProperty: {
    title: string;
    description: string;
    licenseDescription: string;
    exclusions: string[];
    userContent: string;
  };
  liabilityLimitation: {
    title: string;
    description: string;
    nonResponsibility: string;
    items: string[];
    liabilityLimit: string;
  };
  payments: {
    title: string;
    description: string;
    items: string[];
    subscriptionInfo: string;
  };
  governingLaw: {
    title: string;
    description: string;
    disputeResolution: string;
    jurisdiction: string;
  };
}

interface InfoLicenseSection {
  usageLicense: {
    title: string;
    description: string;
    includes: string;
    items: string[];
    termination: string;
  };
  restrictions: {
    title: string;
    description: string;
    items: string[];
    consequence: string;
  };
  licenseLink: {
    title: string;
    description: string;
    buttonText: string;
    url: string;
  };
}

interface InfoSecurityHallOfFame {
  title: string;
  description: string;
  recognition: string;
  reporting: string;
  securityEmail: string;
  ctaText: string;
  ctaUrl: string;
}

// Tipo principal que representa toda la estructura
export type InfoI18n = {
  info: {
    privacyPolicy: {
      title: string;
      sections: InfoPrivacyPolicySection;
    };
    termsAndConditions: {
      title: string;
      sections: InfoTermsAndConditionsSection;
    };
    license: {
      title: string;
      sections: InfoLicenseSection;
    };
    securityHallOfFame: InfoSecurityHallOfFame;
  };
};
