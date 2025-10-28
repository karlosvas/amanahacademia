import english from "./en.json" with { type: "json" };
import spanish from "./es.json" with { type: "json" };
import french from "./fr.json" with { type: "json" };
import german from "./de.json" with { type: "json" };
import italian from "./it.json" with { type: "json" };
import portuguese from "./pt.json" with { type: "json" };
import arabic from "./ar.json" with { type: "json" };
import type { I18nTranslations, Lang } from "@/types/types";
import { Languages } from "@/enums/enums";

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

export const buttonsCommentTraductions: Record<
  string,
  { edit: string; delete: string; reply: string; cancel: string; send: string; response: string; responses: string }
> = {
  es: {
    edit: "Editar",
    delete: "Eliminar",
    reply: "Responder",
    cancel: "Cancelar",
    send: "Enviar",
    response: "respuesta",
    responses: "respuestas",
  },
  en: {
    edit: "Edit",
    delete: "Delete",
    reply: "Reply",
    cancel: "Cancel",
    send: "Send",
    response: "response",
    responses: "responses",
  },
  de: {
    edit: "Bearbeiten",
    delete: "Löschen",
    reply: "Antworten",
    cancel: "Abbrechen",
    send: "Senden",
    response: "Antwort",
    responses: "Antworten",
  },
  ar: {
    edit: "تعديل",
    delete: "حذف",
    reply: "رد",
    cancel: "إلغاء",
    send: "إرسال",
    response: "رد",
    responses: "ردود",
  },
  fr: {
    edit: "Modifier",
    delete: "Supprimer",
    reply: "Répondre",
    cancel: "Annuler",
    send: "Envoyer",
    response: "réponse",
    responses: "réponses",
  },
  it: {
    edit: "Modifica",
    delete: "Elimina",
    reply: "Rispondi",
    cancel: "Annulla",
    send: "Invia",
    response: "risposta",
    responses: "risposte",
  },
  pt: {
    edit: "Editar",
    delete: "Excluir",
    reply: "Responder",
    cancel: "Cancelar",
    send: "Enviar",
    response: "resposta",
    responses: "respostas",
  },
};

export const cookiesTranslation = {
  ar: {
    accept: "قبول",
    reject: "رفض",
    // [title, paragraph, link text]
    message: [
      "نستخدم ملفات تعريف الارتباط لتحسين تجربتك",
      "نستخدم ملفات تعريف الارتباط لتحليل حركة مرور الموقع وتحسين خدماتنا.",
      "مزيد من المعلومات حول سياسة الخصوصية وملفات تعريف الارتباط الخاصة بنا.",
    ],
  },
  de: {
    accept: "Akzeptieren",
    reject: "Ablehnen",
    message: [
      "Wir verwenden Cookies, um deine Erfahrung zu verbessern",
      "Wir verwenden Cookies, um den Website-Verkehr zu analysieren und unsere Dienste zu verbessern.",
      "Weitere Informationen zu unserer Datenschutz- und Cookie-Richtlinie.",
    ],
  },
  en: {
    accept: "Accept",
    reject: "Reject",
    message: [
      "We use cookies to improve your experience",
      "We use cookies to analyze site traffic and improve our services.",
      "More information about our privacy and cookie policy.",
    ],
  },
  es: {
    accept: "Aceptar",
    reject: "Rechazar",
    message: [
      "Usamos cookies para mejorar tu experiencia",
      "Utilizamos cookies para analizar el tráfico del sitio y mejorar nuestros servicios.",
      "Más información sobre nuestra política de privacidad y cookies.",
    ],
  },
  fr: {
    accept: "Accepter",
    reject: "Refuser",
    message: [
      "Nous utilisons des cookies pour améliorer votre expérience",
      "Nous utilisons des cookies pour analyser le trafic du site et améliorer nos services.",
      "Plus d'informations sur notre politique de confidentialité et de cookies.",
    ],
  },
  it: {
    accept: "Accetta",
    reject: "Rifiuta",
    message: [
      "Utilizziamo i cookie per migliorare la tua esperienza",
      "Utilizziamo i cookie per analizzare il traffico del sito e migliorare i nostri servizi.",
      "Maggiori informazioni sulla nostra politica sulla privacy e sui cookie.",
    ],
  },
  pt: {
    accept: "Aceitar",
    reject: "Rejeitar",
    message: [
      "Usamos cookies para melhorar sua experiência",
      "Usamos cookies para analisar o tráfego do site e melhorar nossos serviços.",
      "Mais informações sobre nossa política de privacidade e cookies.",
    ],
  },
};
