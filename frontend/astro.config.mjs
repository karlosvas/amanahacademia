// @ts-check
import { defineConfig } from "astro/config";
import tailwind from "@astrojs/tailwind";
import sitemap from "@astrojs/sitemap";
import solidJs from "@astrojs/solid-js";
import vercel from "@astrojs/vercel";
import { loadEnv } from "vite";
import sentry from "@sentry/astro";

const { SENTRY_AUTH_TOKEN } = loadEnv(
  process.env.NODE_ENV || "production",
  process.cwd(),
  "",
);

// https://astro.build/config
export default defineConfig({
  site: "https://amanahacademia.com",
  output: "server",
  adapter: vercel(),
  i18n: {
    defaultLocale: "es",
    locales: ["es", "en", "fr", "de", "it", "pt", "ar"],
    routing: {
      prefixDefaultLocale: false,
    },
  },
  prefetch: true,
  integrations: [
    tailwind(),
    solidJs(),
    sitemap({
      i18n: {
        defaultLocale: "es",
        locales: {
          es: "es",
          en: "en",
          fr: "fr",
          de: "de",
          it: "it",
          pt: "pt",
          ar: "ar",
        },
      },
      filter: (page) => !page.includes("/admin/") && !page.includes("/404"),
    }),
    sentry({
      project: "amanahacademia",
      org: "karlosvas",
      authToken: process.env.SENTRY_AUTH_TOKEN,
    }),
  ],
  devToolbar: {
    enabled: false,
  },
});

// Antiguo adaptador de Cloudflare
// adapter: cloudflare({
//   imageService: 'passthrough',
//   routes: {
//     extend: {
//       exclude: [
//         { pattern: '/sitemap-index.xml' },
//         { pattern: '/sitemap-*.xml' }
//       ]
//     }
//   }
// }),
