export interface i18nTranslations {
  header: HeaderIn18;
  main: MainIn18;
  footer: FooterIn18;
}

export type HeaderIn18 = {
  about_us: string;
  articles: string;
  pricing: string;
  courses: string;
  comments: string;
  contact: string;
};

export type MainIn18 = {
  home: HomeIn18;
  articles: ArticleIn18;
  courses: SimpleSectionIn18;
  comments: SimpleSectionIn18;
  contact: SimpleSectionIn18;
};

export type HomeIn18 = {
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
    cards: [FocusCardIn18, FocusCardIn18, FocusCardIn18];
    note: string[];
  };
};

export type FocusCardIn18 = {
  title: string;
  description: string;
};

export type SimpleSectionIn18 = {
  title: string;
};

export type ArticleIn18 = {
  title: string;
  articles: [
    {
      title: string;
      summary: string[];
    }
  ];
};

export type FooterIn18 = {
  license: string;
  privacy_policy: string;
  terms_of_service: string;
};
