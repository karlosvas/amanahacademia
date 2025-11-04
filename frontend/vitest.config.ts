import { defineConfig } from 'vitest/config';
import { resolve } from 'path';

export default defineConfig({
  test: {
    // Entorno de pruebas
    environment: 'jsdom',

    // Archivos de configuración global
    setupFiles: ['./test/setup.ts'],

    // Cobertura de código
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html', 'lcov'],
      exclude: [
        'node_modules/',
        'test/',
        'dist/',
        '.astro/',
        '**/*.config.{js,ts}',
        '**/types/**',
        '**/enums/**',
        '**/*.d.ts',
        '**/assets/**',
        '**/styles/**',
      ],
      // Umbrales de cobertura para SonarQube
      lines: 60,
      functions: 60,
      branches: 60,
      statements: 60,
    },

    // Incluir archivos de test
    include: ['test/**/*.test.{ts,tsx}', 'src/**/*.test.{ts,tsx}'],

    // Excluir archivos que no son tests
    exclude: [
      'node_modules',
      'dist',
      '.astro',
      'coverage',
    ],

    // Configuración de globals para no tener que importar expect, describe, etc
    globals: true,

    // Timeout para tests (útil para tests con Firebase/Stripe)
    testTimeout: 10000,
  },

  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
    },
  },
});
