import english from "./en.json";
import spanish from "./es.json";
import french from "./fr.json";
import german from "./de.json";
import italian from "./it.json";
import portuguese from "./pt.json";
import arabic from "./ar.json";
import type { I18nTranslations } from "@/types/types";

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
  if (lang === Languages.ENGLISH) return english;
  else if (lang === Languages.SPANISH) return spanish;
  else if (lang === Languages.FRENCH) return french;
  else if (lang === Languages.GERMAN) return german;
  else if (lang === Languages.ITALIAN) return italian;
  else if (lang === Languages.PORTUGUESE) return portuguese;
  else if (lang === Languages.ARABIC) return arabic;
  else return spanish;
};
