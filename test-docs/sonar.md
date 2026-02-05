# SonarQube / SonarCloud — Guía de análisis y cobertura

Guía para generar informes de cobertura y ejecutar análisis de SonarQube/SonarCloud en este monorepo (`frontend` + `backend`).

---

## Arquitectura del proyecto

**Archivos de configuración:**

- `sonar-project.properties` (raíz del repositorio)
  - Define `sonar.sources` y rutas de cobertura
  - Frontend: `frontend/coverage/lcov.info`
  - Backend: `backend/coverage/cobertura.xml`

**Estructura esperada:**

```
proyecto/
├── sonar-project.properties
├── frontend/
│   ├── coverage/
│   │   └── lcov.info
│   └── ...
└── backend/
    ├── coverage/
    │   └── cobertura.xml
    └── ...
```

---

## Generación de cobertura local

### Frontend (Astro / TypeScript con Vitest)

```bash
cd frontend

# npm
npm ci
npm run test:coverage

# pnpm
pnpm install
pnpm run test:coverage
```

**Resultado esperado:** `frontend/coverage/lcov.info`

> **Nota:** Si se usa Jest u otro test runner, configurar la salida en formato LCOV en la misma ruta.

---

### Backend (Rust con cargo-llvm-cov)

**Instalación inicial (ejecutar una vez):**

```bash
# Instalar componente llvm-tools
rustup component add llvm-tools-preview

# Instalar cargo-llvm-cov
cargo install cargo-llvm-cov
```

**Generar cobertura:**

```bash
cd backend
mkdir -p coverage
cargo llvm-cov --cobertura --output-path coverage/cobertura.xml
```

**Resultado esperado:** `backend/coverage/cobertura.xml`

> **Ventajas de llvm-cov:**
>
> - Más rápido que tarpaulin
> - Mejor soporte para Rust moderno
> - Compatible con todos los sistemas operativos (Linux, macOS, Windows)

**Dependencias del sistema (Ubuntu/Debian):**

```bash
sudo apt-get update
sudo apt-get install -y libssl-dev pkg-config
```

**Para excluir archivos de test:**

```bash
cargo llvm-cov --cobertura \
  --output-path coverage/cobertura.xml \
  --ignore-filename-regex "src/test.rs"
```

---

## Configuración de secretos

### Variables requeridas

| Variable         | Descripción                                        | Obligatorio        |
| ---------------- | -------------------------------------------------- | ------------------ |
| `SONAR_TOKEN`    | Token de análisis de SonarCloud/SonarQube          | ✅ Sí              |
| `SONAR_HOST_URL` | URL del servidor (solo para SonarQube self-hosted) | ❌ No (SonarCloud) |

### Obtener token de SonarCloud

1. Acceder a [https://sonarcloud.io](https://sonarcloud.io)
2. Navegar a **My Account** → **Security** → **Tokens**
3. Crear nuevo token con nombre descriptivo (ej: `amanah-ci`)
4. Copiar el valor (no se mostrará nuevamente)

### Configurar secretos en GitHub

**Para mantenedores:**

1. Ir a **Settings** → **Secrets and variables** → **Actions**
2. Crear nuevo secret:
   - **Name:** `SONAR_TOKEN`
   - **Value:** (pegar token de SonarCloud)

**Para contribuyentes externos:**

Solicitar al mantenedor que configure el token. Por seguridad, GitHub no expone secrets a workflows ejecutados desde forks.

---

## Flujo de trabajo para contribuyentes

### Validación local antes de PR

**1. Generar cobertura frontend:**

```bash
cd frontend
npm ci
npm run test:coverage
# Verificar: frontend/coverage/lcov.info existe
```

**2. Generar cobertura backend:**

```bash
cd backend
mkdir -p coverage
cargo llvm-cov --cobertura --output-path coverage/cobertura.xml
# Verificar: backend/coverage/cobertura.xml existe
```

**Nota:** Requiere `llvm-tools-preview` instalado:

```bash
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov
```

**3. Ejecutar análisis Sonar localmente (opcional):**

**Opción A: Docker (sin instalación)**

```bash
docker run --rm \
  -v $(pwd):/usr/src \
  -e SONAR_TOKEN="$SONAR_TOKEN" \
  sonarsource/sonar-scanner-cli \
  -Dsonar.host.url=https://sonarcloud.io \
  -Dsonar.projectKey=amanahacademia \
  -Dsonar.sources=frontend/src,backend/src \
  -Dsonar.javascript.lcov.reportPaths=frontend/coverage/lcov.info \
  -Dsonar.coverageReportPaths=backend/coverage/cobertura.xml
```

**PowerShell (Windows):**

```powershell
docker run --rm `
  -v ${PWD}:/usr/src `
  -e SONAR_TOKEN="$env:SONAR_TOKEN" `
  sonarsource/sonar-scanner-cli `
  -Dsonar.host.url=https://sonarcloud.io `
  -Dsonar.projectKey=amanahacademia `
  -Dsonar.sources=frontend/src,backend/src `
  -Dsonar.javascript.lcov.reportPaths=frontend/coverage/lcov.info `
  -Dsonar.coverageReportPaths=backend/coverage/cobertura.xml
```

**Opción B: CLI nativo**

```bash
# Instalar sonar-scanner según el sistema operativo
sonar-scanner \
  -Dsonar.projectKey=amanahacademia \
  -Dsonar.sources=frontend/src,backend/src \
  -Dsonar.javascript.lcov.reportPaths=frontend/coverage/lcov.info \
  -Dsonar.coverageReportPaths=backend/coverage/cobertura.xml \
  -Dsonar.host.url=https://sonarcloud.io \
  -Dsonar.login="$SONAR_TOKEN"
```

**Configurar token localmente (temporal):**

```bash
# Bash/Linux/macOS
export SONAR_TOKEN="tu_token_aqui"

# PowerShell
$env:SONAR_TOKEN = "tu_token_aqui"
```

> **Recomendación:** No subir análisis a SonarCloud desde entorno local si solo se quiere validar. Usar una instancia local de SonarQube o revisar el output del scanner sin el parámetro `-Dsonar.host.url`.

---

## Integración continua (CI)

### Workflows automáticos

El repositorio incluye tres workflows especializados:

1. **`frontend-tests.yml`** — Validación rápida de frontend
   - Tests, formato, build y cobertura
   - Se ejecuta en branches de desarrollo y PRs

2. **`backend-tests.yml`** — Validación rápida de backend
   - Tests, formato, lint y build
   - Se ejecuta en branches de desarrollo y PRs

3. **`sonar.yml`** — Análisis de calidad completo
   - Genera cobertura de frontend y backend
   - Envía resultados a SonarCloud
   - Se ejecuta en push a `main` y PRs

**Ejecución automática:**

- Push a `main`/`develop`/`feature/**`/`fix/**`
- Pull requests a `main`/`develop`

### Limitaciones en PRs desde forks

Por seguridad, GitHub no expone `SONAR_TOKEN` a workflows desde forks externos.

**Alternativas:**

- Ejecutar análisis localmente antes de abrir PR
- Solicitar al mantenedor que re-ejecute el workflow manualmente
- Contribuir desde branches del repositorio principal (requiere permisos)

---

## Configuración avanzada

### sonar-project.properties

Parámetros clave ya configurados:

```properties
sonar.projectKey=amanahacademia
sonar.sources=frontend/src,backend/src
sonar.exclusions=**/node_modules/**,**/target/**,**/coverage/**,**/public/**

# Cobertura
sonar.javascript.lcov.reportPaths=frontend/coverage/lcov.info
sonar.coverageReportPaths=backend/coverage/cobertura.xml
```

**Recomendaciones:**

- Mantener exclusiones para evitar análisis de código generado
- Verificar que las rutas de cobertura coincidan con la salida de los test runners
- Ajustar `sonar.sources` si se añaden nuevos módulos

---

## Troubleshooting

### Cobertura 0% en frontend

**Causa:** SonarCloud no encuentra `lcov.info`

**Solución:**

```bash
# Verificar existencia del archivo
ls -la frontend/coverage/lcov.info

# Verificar ruta en sonar-project.properties
grep "lcov.reportPaths" sonar-project.properties
```

### Error "No coverage information" en backend

**Causa:** Archivo no generado o formato inválido

**Solución:**

```bash
# Verificar existencia del archivo
ls -la backend/coverage/cobertura.xml

# Regenerar cobertura
cd backend
cargo llvm-cov --cobertura --output-path coverage/cobertura.xml
```

### cargo-llvm-cov no encontrado

**Causa:** Herramienta no instalada

**Solución:**

```bash
# Instalar componentes necesarios
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov
```

### Pull request analysis no disponible

**Causa:** Configuración de SonarCloud incompleta

**Solución:**

1. Acceder a SonarCloud → proyecto → **Administration** → **Pull Requests**
2. Habilitar análisis de PRs
3. Configurar decoración automática en GitHub

---

## Referencias

- [SonarCloud Documentation](https://docs.sonarcloud.io/)
- [cargo-llvm-cov GitHub](https://github.com/taiki-e/cargo-llvm-cov)
- [Vitest Coverage](https://vitest.dev/guide/coverage.html)
- [Sonar Scanner CLI](https://docs.sonarcloud.io/advanced-setup/ci-based-analysis/sonarscanner-cli/)
