import english from "./en.json" with { type: "json" };
import spanish from "./es.json" with { type: "json" };
import french from "./fr.json" with { type: "json" };
import german from "./de.json" with { type: "json" };
import italian from "./it.json" with { type: "json" };
import portuguese from "./pt.json" with { type: "json" };
import arabic from "./ar.json" with { type: "json" };
import type { I18nTranslations, Lang } from "@/types/types";

// Lenguajes permitidos
const enum Languages {
  ENGLISH = "en",
  SPANISH = "es",
  FRENCH = "fr",
  GERMAN = "de",
  ITALIAN = "it",
  PORTUGUESE = "pt",
  ARABIC = "ar",
}

// Obtener la traduccion correspondiente al idioma selecionado
export const getI18N = ({ lang = "es" }: { lang: string | undefined }): I18nTranslations => {
  if (lang === Languages.ENGLISH) return english as I18nTranslations;
  else if (lang === Languages.SPANISH) return spanish as I18nTranslations;
  else if (lang === Languages.FRENCH) return french as I18nTranslations;
  else if (lang === Languages.GERMAN) return german as I18nTranslations;
  else if (lang === Languages.ITALIAN) return italian as I18nTranslations;
  else if (lang === Languages.PORTUGUESE) return portuguese as I18nTranslations;
  else if (lang === Languages.ARABIC) return arabic as I18nTranslations;
  else return spanish as I18nTranslations;
};

export const labels_header: Record<
  Lang,
  {
    traductor: string;
    identificationButton: string;
    legalLink: string;
  }
> = {
  es: {
    traductor: "Cambiar idioma",
    identificationButton: "Iniciar sesión en Amanah Academia",
    legalLink:
      "Página de información legal de Amanah Academia: Licencia, Política de privacidad y Términos y condiciones",
  },
  en: {
    traductor: "Change language",
    identificationButton: "Log in to Amanah Academia",
    legalLink: "Amanah Academia legal information page: License, Privacy Policy, and Terms and Conditions",
  },
  fr: {
    traductor: "Changer de langue",
    identificationButton: "Se connecter à Amanah Academia",
    legalLink:
      "Page d'informations légales d'Amanah Academia : Licence, Politique de confidentialité et Conditions générales",
  },
  de: {
    traductor: "Sprache ändern",
    identificationButton: "Bei Amanah Academia anmelden",
    legalLink:
      "Rechtliche Informationsseite von Amanah Academia: Lizenz, Datenschutzrichtlinie und Allgemeine Geschäftsbedingungen",
  },
  ar: {
    traductor: "تغيير اللغة",
    identificationButton: "تسجيل الدخول إلى أكاديمية أمانة",
    legalLink: "صفحة المعلومات القانونية لأكاديمية أمانة: الترخيص، سياسة الخصوصية والشروط والأحكام",
  },
  it: {
    traductor: "Cambia lingua",
    identificationButton: "Accedi ad Amanah Academia",
    legalLink:
      "Pagina delle informazioni legali di Amanah Academia: Licenza, Informativa sulla privacy e Termini e condizioni",
  },
  pt: {
    traductor: "Mudar idioma",
    identificationButton: "Entrar na Amanah Academia",
    legalLink: "Página de informações legais da Amanah Academia: Licença, Política de Privacidade e Termos e Condições",
  },
};
