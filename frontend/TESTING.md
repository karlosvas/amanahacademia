# Testing Documentation - Amanah Academia Frontend

## Índice

- [Introducción](#introducción)
- [Configuración](#configuración)
- [Ejecutar Tests](#ejecutar-tests)
- [Estructura de Tests](#estructura-de-tests)
- [Cobertura Actual](#cobertura-actual)
- [Escribir Nuevos Tests](#escribir-nuevos-tests)
- [Mejores Prácticas](#mejores-prácticas)
- [CI/CD](#cicd)

## Introducción

Este proyecto utiliza **Vitest** como framework de testing, que es rápido, compatible con ESM y está diseñado para proyectos Vite/Astro.

### Tecnologías de Testing

- **Vitest**: Framework de testing (v4.0.2)
- **@vitest/coverage-v8**: Generación de reportes de cobertura
- **jsdom**: Entorno DOM para tests unitarios

## Configuración

La configuración de Vitest se encuentra en `vitest.config.ts`:

```typescript
export default defineConfig({
  test: {
    environment: "jsdom",
    setupFiles: ["./test/setup.ts"],
    coverage: {
      provider: "v8",
      lines: 60,
      functions: 60,
      branches: 60,
      statements: 60,
    },
    globals: true,
  },
});
```

### Setup Global

El archivo `test/setup.ts` contiene los mocks globales:

- Mock de Firebase Auth
- Mock de globalThis.matchMedia
- Mock de solid-toast
- Mock de Cloudflare Turnstile
- Mock de localStorage
- Mock de import.meta.env

## Ejecutar Tests

### Comandos Disponibles

```bash
# Ejecutar tests en modo watch
npm test

# Ejecutar tests una vez
npm run test:run

# Ejecutar tests con coverage
npm run test:coverage

# Ejecutar tests en modo watch con coverage
npm run test:coverage:watch
```

### Ejemplo de Ejecución

```bash
cd frontend
npm test -- --run
```

## Estructura de Tests

```
frontend/
├── test/
│   ├── setup.ts                    # Configuración global y mocks
│   ├── utils.test.ts               # Tests de ejemplo
│   ├── utils/
│   │   ├── cookie.test.ts          # Tests de gestión de cookies
│   │   ├── metrics.test.ts         # Tests de métricas
│   │   └── modals.test.ts          # Tests de modales
│   └── services/
│       └── helper.test.ts          # Tests de ApiService y ResultUtils
├── vitest.config.ts                # Configuración de Vitest
└── sonar-project.properties        # Configuración de SonarQube
```

## Cobertura Actual

### Resumen General

| Métrica    | Cobertura |
| ---------- | --------- |
| Statements | 53.47%    |
| Branches   | 56.48%    |
| Functions  | 62.5%     |
| Lines      | 52.15%    |

### Detalles por Módulo

#### Utilities (65.03%)

- ✅ **metrics.ts**: 100% - Completamente testeado
- ⚠️ **cookie.ts**: 69.62% - Buen coverage
- ⚠️ **modals.ts**: 52.85% - Mejorable

#### Services (38.4%)

- ⚠️ **helper.ts**: 37.16% - Necesita más tests
- ⚠️ **globalHandler.ts**: 50% - Mejorable

## Tests Implementados

### 1. Tests de Utilidades (utils/)

#### cookie.test.ts (14 tests)

- ✅ Obtención de tema desde cookies
- ✅ Obtención de idioma desde cookies
- ✅ Aceptación de cookies con gtag
- ✅ Rechazo de cookies
- ✅ Inicialización de consentimiento

#### metrics.test.ts (8 tests)

- ✅ Parseo de métricas de usuarios
- ✅ Parseo de métricas de artículos
- ✅ Parseo de métricas de clases
- ✅ Mapeo de datos a meses

#### modals.test.ts (13 tests)

- ✅ Animación de cierre de modales
- ✅ Animación de apertura de modales
- ✅ Bloqueo de scroll
- ✅ Compensación de scrollbar
- ✅ Focus en inputs

### 2. Tests de Servicios (services/)

#### helper.test.ts (18 tests)

- ✅ ResultUtils.ok()
- ✅ ResultUtils.error()
- ✅ ResultUtils.getErrorType()
- ✅ ApiService.getAllComments()
- ✅ ApiService.getTeachers()
- ✅ ApiService.registerUser()
- ✅ ApiService.sendContact()
- ✅ Manejo de errores HTTP (401, 404, 422, 500)

## Escribir Nuevos Tests

### Estructura Básica

```typescript
import { describe, it, expect, beforeEach, vi } from "vitest";

describe("MiComponente", () => {
  beforeEach(() => {
    // Setup antes de cada test
    vi.clearAllMocks();
  });

  it("debería hacer algo específico", () => {
    // Arrange
    const input = "test";

    // Act
    const result = miFunction(input);

    // Assert
    expect(result).toBe("expected");
  });
});
```

### Mocking

#### Mock de Firebase

```typescript
vi.mock("@/services/firebase", () => ({
  getCurrentUserToken: vi.fn(() => Promise.resolve("mock-token")),
}));
```

#### Mock de fetch

```typescript
global.fetch = vi.fn(() =>
  Promise.resolve({
    ok: true,
    json: async () => ({ success: true, data: mockData }),
  })
) as any;
```

#### Mock de timers

```typescript
beforeEach(() => {
  vi.useFakeTimers();
});

afterEach(() => {
  vi.useRealTimers();
});

it("debería ejecutar después de timeout", () => {
  myFunction();
  vi.advanceTimersByTime(1000);
  expect(callback).toHaveBeenCalled();
});
```

## Mejores Prácticas

### 1. Nombres Descriptivos

```typescript
// ❌ Mal
it('test 1', () => { ... });

// ✅ Bien
it('should return user data when authentication is successful', () => { ... });
```

### 2. Arrange-Act-Assert

```typescript
it("should calculate total correctly", () => {
  // Arrange
  const items = [{ price: 10 }, { price: 20 }];

  // Act
  const total = calculateTotal(items);

  // Assert
  expect(total).toBe(30);
});
```

### 3. Tests Independientes

```typescript
// ❌ Mal - Tests dependientes entre sí
let sharedState = null;

it("test 1", () => {
  sharedState = { value: 1 };
});

it("test 2", () => {
  expect(sharedState.value).toBe(1); // Depende de test 1
});

// ✅ Bien - Tests independientes
beforeEach(() => {
  sharedState = { value: 1 };
});
```

### 4. Mock Solo lo Necesario

```typescript
// ❌ Mal - Mock de todo
vi.mock("../../entire-module");

// ✅ Bien - Mock específico
vi.mock("@/services/firebase", () => ({
  getCurrentUserToken: vi.fn(),
}));
```

## Áreas Prioritarias para Mejorar Cobertura

### 1. Services (Prioridad Alta)

```typescript
// frontend/test/services/firebase.test.ts
describe('Firebase Service', () => {
  it('should login with email and password', async () => { ... });
  it('should handle Google login', async () => { ... });
  it('should logout user', async () => { ... });
});
```

### 2. Services Payment (Prioridad Alta)

```typescript
// frontend/test/services/payment.test.ts
describe('Payment Service', () => {
  it('should create payment intent', async () => { ... });
  it('should handle payment errors', async () => { ... });
});
```

### 3. Services Comments (Prioridad Media)

```typescript
// frontend/test/services/comments.test.ts
describe('Comments Service', () => {
  it('should post comment', async () => { ... });
  it('should delete comment', async () => { ... });
  it('should edit comment', async () => { ... });
});
```

### 4. Modals Coverage (Prioridad Media)

Completar tests para casos edge en modals.ts

## CI/CD

### GitHub Actions

El workflow `.github/workflows/frontend-tests.yml` ejecuta automáticamente:

1. Tests en cada push a `main` o `develop`
2. Tests en cada Pull Request
3. Generación de reporte de cobertura
4. Upload de coverage a Codecov
5. Comentario en PR con métricas de cobertura

### SonarQube

Configuración en `sonar-project.properties`:

- Análisis de código TypeScript
- Exclusión de node_modules, dist, tests
- Reporte de cobertura desde `coverage/lcov.info`

Para ejecutar análisis de SonarQube:

```bash
# Con sonar-scanner instalado
sonar-scanner
```

## Comandos Útiles

```bash
# Ver coverage en el navegador
npm run test:coverage
open coverage/index.html

# Ejecutar tests específicos
npm test -- cookie.test.ts

# Ejecutar tests en modo watch con filter
npm test -- --watch cookie

# Ejecutar con UI (requiere @vitest/ui)
npm run test:ui
```

## Recursos

- [Vitest Documentation](https://vitest.dev/)
- [Testing Best Practices](https://github.com/goldbergyoni/javascript-testing-best-practices)
- [Astro Testing Guide](https://docs.astro.build/en/guides/testing/)

## Contribuir

Al agregar nuevas funcionalidades, por favor:

1. Escribe tests para el nuevo código
2. Mantén la cobertura mínima del 60%
3. Ejecuta `npm run test:coverage` antes de hacer commit
4. Documenta casos edge y limitaciones conocidas

---

**Última actualización**: 2025-11-04
**Versión de Vitest**: 4.0.2
**Cobertura actual**: 53.47%
