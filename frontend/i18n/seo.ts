import type { SEOTranslations } from "@/types/types";

// Función helper para generar SEO data por página
export function getSEOData(page: string, lang: string = "es") {
  const baseTitle = "Amanah Academia";

  const seoData: SEOTranslations = {
    es: {
      index: {
        title: `${baseTitle} - Aprende idiomas online`,
        description:
          "Academia de idiomas online especializada en árabe y otros idiomas. Clases con profesores nativos.",
        keywords:
          "academia idiomas, árabe online, clases idiomas, profesores nativos",
        structuredDataType: "organization" as const,
        structuredData: {
          name: "Amanah Academia",
          description:
            "Academia de idiomas online especializada en árabe y otros idiomas",
        },
      },
      courses: {
        title: `Cursos de Idiomas - ${baseTitle}`,
        description:
          "Descubre nuestros cursos de árabe, inglés, francés y más idiomas con profesores nativos certificados.",
        keywords:
          "cursos árabe, clases inglés, francés online, idiomas certificados",
        structuredDataType: "course" as const,
        structuredData: {
          name: "Cursos de Idiomas",
          description: "Cursos completos de idiomas online con certificación",
        },
      },
      pricing: {
        title: `Precios y Planes - ${baseTitle}`,
        description:
          "Consulta nuestros precios accesibles para clases de idiomas online. Planes flexibles para todos los presupuestos.",
        keywords:
          "precios clases idiomas, tarifas árabe, costos cursos online, planes idiomas",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Precios y Planes",
          description: "Precios accesibles para clases de idiomas online",
        },
      },
      contact: {
        title: `Contacto - ${baseTitle}`,
        description:
          "Ponte en contacto con Amanah Academia. Información sobre nuestros cursos de idiomas y profesores nativos.",
        keywords:
          "contacto amanah academia, información cursos, consultas idiomas",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Contacto",
          description: "Información de contacto y consultas",
        },
      },
      articles: {
        title: `Artículos y Blog - ${baseTitle}`,
        description:
          "Lee nuestros artículos sobre aprendizaje de idiomas, cultura árabe y consejos para estudiar online.",
        keywords:
          "blog idiomas, artículos árabe, consejos aprender idiomas, cultura árabe",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Blog y Artículos",
          description: "Artículos sobre aprendizaje de idiomas",
        },
      },
      comments: {
        title: `Testimonios - ${baseTitle}`,
        description:
          "Lee los testimonios de nuestros estudiantes sobre su experiencia aprendiendo idiomas con nosotros.",
        keywords:
          "testimonios amanah academia, opiniones estudiantes, reseñas cursos idiomas",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Testimonios",
          description: "Testimonios de nuestros estudiantes",
        },
      },
    },
    ar: {
      index: {
        title: `${baseTitle} - تعلم اللغات عبر الإنترنت`,
        description:
          "أكاديمية لغات عبر الإنترنت متخصصة في العربية وغيرها من اللغات. دروس مع معلمين ناطقين باللغة.",
        keywords:
          "أكاديمية لغات, العربية عبر الإنترنت, دروس لغات, معلمون ناطقون",
        structuredDataType: "organization" as const,
        structuredData: {
          name: "أكاديمية أمانة",
          description:
            "أكاديمية لغات عبر الإنترنت متخصصة في العربية وغيرها من اللغات",
        },
      },
      courses: {
        title: `دورات اللغات - ${baseTitle}`,
        description:
          "اكتشف دوراتنا في العربية والإنجليزية والفرنسية وغيرها من اللغات مع معلمين ناطقين معتمدين.",
        keywords:
          "دورات عربية, دروس إنجليزية, فرنسية عبر الإنترنت, لغات معتمدة",
        structuredDataType: "course" as const,
        structuredData: {
          name: "دورات اللغات",
          description: "دورات شاملة للغات عبر الإنترنت مع شهادة",
        },
      },
      pricing: {
        title: `الأسعار والخطط - ${baseTitle}`,
        description:
          "استعرض أسعارنا المعقولة لدروس اللغات عبر الإنترنت. خطط مرنة تناسب جميع الميزانيات.",
        keywords:
          "أسعار دروس اللغات, رسوم عربية, تكاليف دورات عبر الإنترنت, خطط لغات",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "الأسعار والخطط",
          description: "أسعار معقولة لدروس اللغات عبر الإنترنت",
        },
      },
      contact: {
        title: `اتصل بنا - ${baseTitle}`,
        description:
          "تواصل مع أكاديمية أمانة. معلومات حول دوراتنا في اللغات ومعلمينا الناطقين.",
        keywords: "اتصال أكاديمية أمانة, معلومات دورات, استفسارات لغات",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "اتصل بنا",
          description: "معلومات الاتصال والاستفسارات",
        },
      },
      articles: {
        title: `المقالات والمدونة - ${baseTitle}`,
        description:
          "اقرأ مقالاتنا حول تعلم اللغات، الثقافة العربية ونصائح للدراسة عبر الإنترنت.",
        keywords: "مدونة لغات, مقالات عربية, نصائح تعلم لغات, ثقافة عربية",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "المدونة والمقالات",
          description: "مقالات حول تعلم اللغات",
        },
      },
      comments: {
        title: `الشهادات - ${baseTitle}`,
        description: "اقرأ شهادات طلابنا حول تجربتهم في تعلم اللغات معنا.",
        keywords: "شهادات أكاديمية أمانة, آراء الطلاب, مراجعات دورات اللغات",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "الشهادات",
          description: "شهادات طلابنا",
        },
      },
    },
    de: {
      index: {
        title: `${baseTitle} - Online-Sprachkurse`,
        description:
          "Online-Sprachakademie spezialisiert auf Arabisch und andere Sprachen. Unterricht mit muttersprachlichen Lehrern.",
        keywords:
          "Sprachakademie, Arabisch online, Sprachunterricht, muttersprachliche Lehrer",
        structuredDataType: "organization" as const,
        structuredData: {
          name: "Amanah Akademie",
          description:
            "Online-Sprachakademie spezialisiert auf Arabisch und andere Sprachen",
        },
      },
      courses: {
        title: `Sprachkurse - ${baseTitle}`,
        description:
          "Entdecken Sie unsere Kurse in Arabisch, Englisch, Französisch und anderen Sprachen mit zertifizierten muttersprachlichen Lehrern.",
        keywords:
          "Arabischkurse, Englischunterricht, Französisch online, zertifizierte Lehrer",
        structuredDataType: "course" as const,
        structuredData: {
          name: "Sprachkurse",
          description: "Umfassende Online-Sprachkurse mit Zertifikat",
        },
      },
      pricing: {
        title: `Preise und Pläne - ${baseTitle}`,
        description:
          "Durchsuchen Sie unsere erschwinglichen Preise für Online-Sprachunterricht. Flexible Pläne für jedes Budget.",
        keywords:
          "Preise Sprachunterricht, Arabisch Gebühren, Kosten Online-Kurse, Sprachpläne",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Preise und Pläne",
          description: "Erschwingliche Preise für Online-Sprachunterricht",
        },
      },
      contact: {
        title: `Kontakt - ${baseTitle}`,
        description:
          "Kontaktieren Sie die Amanah Akademie. Informationen zu unseren Sprachkursen und muttersprachlichen Lehrern.",
        keywords: "Kontakt Amanah Akademie, Kursinformationen, Sprachanfragen",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Kontakt",
          description: "Kontaktinformationen und Anfragen",
        },
      },
      articles: {
        title: `Artikel und Blog - ${baseTitle}`,
        description:
          "Lesen Sie unsere Artikel über Sprachenlernen, arabische Kultur und Tipps für das Online-Studium.",
        keywords:
          "Blog Sprachen, Artikel Arabisch, Tipps Sprachenlernen, arabische Kultur",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Blog und Artikel",
          description: "Artikel über Sprachenlernen",
        },
      },
      comments: {
        title: `Testimonials - ${baseTitle}`,
        description:
          "Lesen Sie die Testimonials unserer Schüler über ihre Erfahrungen beim Sprachenlernen mit uns.",
        keywords:
          "Testimonials Amanah Akademie, Meinungen Schüler, Bewertungen Sprachkurse",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Testimonials",
          description: "Testimonials unserer Schüler",
        },
      },
    },
    en: {
      index: {
        title: `${baseTitle} - Online Language Courses`,
        description:
          "Online language academy specializing in Arabic and other languages. Classes with native-speaking teachers.",
        keywords:
          "language academy, Arabic online, language classes, native teachers",
        structuredDataType: "organization" as const,
        structuredData: {
          name: "Amanah Academy",
          description:
            "Online language academy specializing in Arabic and other languages",
        },
      },
      courses: {
        title: `Language Courses - ${baseTitle}`,
        description:
          "Explore our courses in Arabic, English, French, and other languages with certified native-speaking teachers.",
        keywords:
          "Arabic courses, English classes, French online, certified teachers",
        structuredDataType: "course" as const,
        structuredData: {
          name: "Language Courses",
          description:
            "Comprehensive online language courses with certification",
        },
      },
      pricing: {
        title: `Pricing and Plans - ${baseTitle}`,
        description:
          "Browse our affordable pricing for online language classes. Flexible plans for every budget.",
        keywords:
          "pricing language classes, Arabic fees, costs online courses, language plans",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Pricing and Plans",
          description: "Affordable pricing for online language classes",
        },
      },
      contact: {
        title: `Contact - ${baseTitle}`,
        description:
          "Get in touch with Amanah Academy. Information about our language courses and native-speaking teachers.",
        keywords:
          "contact Amanah Academy, course information, language inquiries",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Contact",
          description: "Contact information and inquiries",
        },
      },
      articles: {
        title: `Articles and Blog - ${baseTitle}`,
        description:
          "Read our articles on language learning, Arabic culture, and tips for online study.",
        keywords:
          "blog languages, articles Arabic, tips language learning, Arabic culture",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Blog and Articles",
          description: "Articles about language learning",
        },
      },
      comments: {
        title: `Testimonials - ${baseTitle}`,
        description:
          "Read the testimonials of our students about their experiences learning languages with us.",
        keywords:
          "testimonials Amanah Academy, student opinions, course reviews",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Testimonials",
          description: "Testimonials from our students",
        },
      },
    },
    fr: {
      index: {
        title: `${baseTitle} - Cours de Langue en Ligne`,
        description:
          "Académie de langue en ligne spécialisée dans l'arabe et d'autres langues. Cours avec des enseignants natifs.",
        keywords:
          "académie de langue, arabe en ligne, cours de langue, enseignants natifs",
        structuredDataType: "organization" as const,
        structuredData: {
          name: "Académie Amanah",
          description:
            "Académie de langue en ligne spécialisée dans l'arabe et d'autres langues",
        },
      },
      courses: {
        title: `Cours de Langue - ${baseTitle}`,
        description:
          "Découvrez nos cours d'arabe, d'anglais, de français et d'autres langues avec des enseignants natifs certifiés.",
        keywords:
          "cours d'arabe, cours d'anglais, français en ligne, enseignants certifiés",
        structuredDataType: "course" as const,
        structuredData: {
          name: "Cours de Langue",
          description: "Cours de langue en ligne complets avec certification",
        },
      },
      pricing: {
        title: `Tarification et Plans - ${baseTitle}`,
        description:
          "Parcourez nos tarifs abordables pour les cours de langue en ligne. Plans flexibles pour tous les budgets.",
        keywords:
          "tarification cours de langue, frais arabe, coûts des cours en ligne, plans de langue",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Tarification et Plans",
          description:
            "Tarification abordable pour les cours de langue en ligne",
        },
      },
      contact: {
        title: `Contact - ${baseTitle}`,
        description:
          "Contactez l'Académie Amanah. Informations sur nos cours de langue et nos enseignants natifs.",
        keywords:
          "contact Académie Amanah, informations sur les cours, demandes de langue",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Contact",
          description: "Informations de contact et demandes",
        },
      },
      articles: {
        title: `Articles et Blog - ${baseTitle}`,
        description:
          "Lisez nos articles sur l'apprentissage des langues, la culture arabe et des conseils pour l'étude en ligne.",
        keywords:
          "blog langues, articles arabe, conseils apprentissage langue, culture arabe",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Blog et Articles",
          description: "Articles sur l'apprentissage des langues",
        },
      },
      comments: {
        title: `Témoignages - ${baseTitle}`,
        description:
          "Lisez les témoignages de nos étudiants sur leurs expériences d'apprentissage des langues avec nous.",
        keywords:
          "témoignages Académie Amanah, opinions des étudiants, avis sur les cours",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Témoignages",
          description: "Témoignages de nos étudiants",
        },
      },
    },
    it: {
      index: {
        title: `${baseTitle} - Corsi di Lingua Online`,
        description:
          "Accademia di lingua online specializzata in arabo e altre lingue. Corsi con insegnanti madrelingua.",
        keywords:
          "accademia di lingua, arabo online, corsi di lingua, insegnanti madrelingua",
        structuredDataType: "organization" as const,
        structuredData: {
          name: "Accademia Amanah",
          description:
            "Accademia di lingua online specializzata in arabo e altre lingue",
        },
      },
      courses: {
        title: `Corsi di Lingua - ${baseTitle}`,
        description:
          "Scopri i nostri corsi di arabo, inglese, francese e altre lingue con insegnanti madrelingua certificati.",
        keywords:
          "corsi di arabo, corsi di inglese, francese online, insegnanti certificati",
        structuredDataType: "course" as const,
        structuredData: {
          name: "Corsi di Lingua",
          description: "Corsi di lingua online completi con certificazione",
        },
      },
      pricing: {
        title: `Tariffe e Piani - ${baseTitle}`,
        description:
          "Esplora le nostre tariffe accessibili per i corsi di lingua online. Piani flessibili per tutti i budget.",
        keywords:
          "tariffe corsi di lingua, costi arabo, costi corsi online, piani di lingua",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Tariffe e Piani",
          description: "Tariffe accessibili per i corsi di lingua online",
        },
      },
      contact: {
        title: `Contatto - ${baseTitle}`,
        description:
          "Contatta l'Accademia Amanah. Informazioni sui nostri corsi di lingua e sui nostri insegnanti madrelingua.",
        keywords:
          "contatto Accademia Amanah, informazioni sui corsi, richieste di lingua",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Contatto",
          description: "Informazioni di contatto e richieste",
        },
      },
      articles: {
        title: `Articoli e Blog - ${baseTitle}`,
        description:
          "Leggi i nostri articoli sull'apprendimento delle lingue, la cultura araba e consigli per lo studio online.",
        keywords:
          "blog lingue, articoli arabo, consigli apprendimento lingua, cultura araba",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Blog e Articoli",
          description: "Articoli sull'apprendimento delle lingue",
        },
      },
      comments: {
        title: `Testimonianze - ${baseTitle}`,
        description:
          "Leggi le testimonianze dei nostri studenti sulle loro esperienze di apprendimento delle lingue con noi.",
        keywords:
          "testimonianze Accademia Amanah, opinioni degli studenti, recensioni dei corsi",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Testimonianze",
          description: "Testimonianze dei nostri studenti",
        },
      },
    },
    pt: {
      index: {
        title: `${baseTitle} - Cursos de Idiomas Online`,
        description:
          "Academia de idiomas online especializada em árabe e outras línguas. Cursos com professores nativos.",
        keywords:
          "academia de idiomas, árabe online, cursos de idiomas, professores nativos",
        structuredDataType: "organization" as const,
        structuredData: {
          name: "Academia Amanah",
          description:
            "Academia de idiomas online especializada em árabe e outras línguas",
        },
      },
      courses: {
        title: `Cursos de Idiomas - ${baseTitle}`,
        description:
          "Descubra nossos cursos de árabe, inglês, francês e outras línguas com professores nativos certificados.",
        keywords:
          "cursos de árabe, cursos de inglês, francês online, professores certificados",
        structuredDataType: "course" as const,
        structuredData: {
          name: "Cursos de Idiomas",
          description: "Cursos de idiomas online completos com certificação",
        },
      },
      pricing: {
        title: `Preços e Planos - ${baseTitle}`,
        description:
          "Explore nossos preços acessíveis para cursos de idiomas online. Planos flexíveis para todos os orçamentos.",
        keywords:
          "preços cursos de idiomas, custos árabe, custos cursos online, planos de idiomas",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Preços e Planos",
          description: "Preços acessíveis para cursos de idiomas online",
        },
      },
      contact: {
        title: `Contato - ${baseTitle}`,
        description:
          "Entre em contato com a Academia Amanah. Informações sobre nossos cursos de idiomas e nossos professores nativos.",
        keywords:
          "contato Academia Amanah, informações sobre cursos, solicitações de idiomas",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Contato",
          description: "Informações de contato e solicitações",
        },
      },
      articles: {
        title: `Artigos e Blog - ${baseTitle}`,
        description:
          "Leia nossos artigos sobre aprendizado de idiomas, cultura árabe e dicas para estudo online.",
        keywords:
          "blog idiomas, artigos árabe, dicas aprendizado idioma, cultura árabe",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Blog e Artigos",
          description: "Artigos sobre aprendizado de idiomas",
        },
      },
      comments: {
        title: `Depoimentos - ${baseTitle}`,
        description:
          "Leia os depoimentos de nossos alunos sobre suas experiências de aprendizado de idiomas conosco.",
        keywords:
          "depoimentos Academia Amanah, opiniões dos alunos, avaliações dos cursos",
        structuredDataType: "webpage" as const,
        structuredData: {
          name: "Depoimentos",
          description: "Depoimentos de nossos alunos",
        },
      },
    },
  };

  // Devolvemos os dados de SEO para a página solicitada
  const langData = seoData[lang] || seoData.es;
  return langData[page] || langData.index;
}
