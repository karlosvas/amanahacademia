// @ts-check
import { defineConfig } from 'astro/config';
import tailwind from '@astrojs/tailwind';

// https://astro.build/config
export default defineConfig({
  i18n: {
    defaultLocale: 'es',
    locales: ['es', 'en', 'fr', 'de', 'it', 'pt', 'ar'],
    routing: {
      prefixDefaultLocale: false,
    },
  },
  prefetch: true,
  integrations: [tailwind()],
});