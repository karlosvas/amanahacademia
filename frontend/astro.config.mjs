// @ts-check
import { defineConfig } from 'astro/config';
import tailwind from '@astrojs/tailwind';
import sitemap from '@astrojs/sitemap';
import solidJs from '@astrojs/solid-js';
import cloudflare from '@astrojs/cloudflare';

// https://astro.build/config
export default defineConfig({
  site: 'https://amanahacademia.com',
  output: 'server',
  adapter: cloudflare(),
  i18n: {
    defaultLocale: 'es',
    locales: ['es', 'en', 'fr', 'de', 'it', 'pt', 'ar'],
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
        defaultLocale: 'es',
        locales: {
          es: 'es',
          en: 'en',
          fr: 'fr',
          de: 'de',
          it: 'it',
          pt: 'pt',
          ar: 'ar'
        }
      },
      filter: (page) => !page.includes('/admin/') && !page.includes('/404'),
    })
  ],
  image: {
    service: {
      entrypoint: "astro/assets/services/sharp",
      config: {
        imageService: "compile"
      }
    }
  }
});