// @ts-check
import { defineConfig } from 'astro/config';
import tailwind from '@astrojs/tailwind';
import sitemap from '@astrojs/sitemap';
import solidJs from '@astrojs/solid-js';
// import cloudflare from '@astrojs/cloudflare';
import vercel from '@astrojs/vercel';

// https://astro.build/config
export default defineConfig({
  site: 'https://amanahacademia.com',
  output: 'server',
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
  adapter: vercel(),
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
  devToolbar: {
    enabled: false
  }
});