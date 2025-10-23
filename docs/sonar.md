# SonarQube / SonarCloud — Guía rápida

Este documento explica cómo generar los informes de cobertura y ejecutar el análisis de Sonar para este repositorio (monorepo: `frontend` + `backend`). Está pensado como complemento al `sonar-project.properties` que ya tienes en la raíz.

## Resumen de archivos esperados

- `sonar-project.properties` — ya presente en la raíz; define `sonar.sources` y rutas de cobertura.
  - Frontend LCOV: `frontend/coverage/lcov.info`
  - Backend Cobertura (o XML aceptado): `backend/coverage/cobertura.xml`

## 1) Generar cobertura local

### Frontend (Astro / TypeScript) — usando Vitest

1. Entra en la carpeta `frontend`:

```powershell
cd frontend
```

2. Ejecuta los tests con cobertura:

```powershell
# Si usas npm
npm ci
npm run test:coverage

# o con pnpm
pnpm install
pnpm run test:coverage
```

Por defecto Vitest genera `coverage/lcov.info`. Asegúrate de que quede en `frontend/coverage/lcov.info`.

Si usas Jest u otro runner, configura la salida `lcov` y coloca `lcov.info` en la ruta anterior.

---

### Backend (Rust) — cargo-tarpaulin (Linux/CI) - recomendado para CI

En CI (Ubuntu) es sencillo con `cargo-tarpaulin`:

```bash
# instalar (en CI o máquina Linux)
cargo install cargo-tarpaulin || true
# ejecutar y generar xml en backend/coverage
cd backend
cargo tarpaulin --out Xml --output-dir ./coverage
# resultado típico: backend/coverage/cargo-tarpaulin-report.xml
# renombra o indica esa ruta como cobertura en sonar config
```

> Nota: `cargo-tarpaulin` funciona mejor en Linux. Si necesitas cobertura en Windows/macOS, considera `grcov` + llvm-cov (configuración más avanzada).

Si el scanner de Sonar necesita específicamente `cobertura.xml` puedes renombrar el archivo o convertirlo antes de subir:

```powershell
# ejemplo simple en CI para renombrar
mv backend/coverage/cargo-tarpaulin-report.xml backend/coverage/cobertura.xml
```

---

## 2) Variables/Secrets necesarias

- `SONAR_TOKEN` — token de SonarCloud o SonarQube con permisos de análisis. (Guardar como Secret en GitHub)
- `SONAR_HOST_URL` — sólo si usas SonarQube self-hosted; en SonarCloud no es necesario.

Añade estos secrets en Settings → Secrets and variables → Actions en GitHub.

## 3) CI (ya incluido) y cómo probar localmente antes de abrir PR

El repo ya incluye un workflow de GitHub Actions para ejecutar Sonar en CI (`.github/workflows/sonar.yml`).
No es necesario que lo crees: los mantenedores ya lo usan. Lo que sí suele querer hacer un contribuidor es **probar localmente** que su cambio no rompe las pruebas ni reduce la cobertura antes de abrir un PR.

Pasos recomendados para un contribuidor que clona el repo:

1. Ejecuta los tests y genera cobertura para el frontend:

```powershell
cd frontend
npm ci
npm run test:coverage
# verifica que exista frontend/coverage/lcov.info
```

1. Ejecuta cobertura para el backend (Linux recomendado / CI):

```bash
cd backend
sudo apt-get update && sudo apt-get install -y libssl-dev pkg-config
cargo install cargo-tarpaulin || true
cargo tarpaulin --out Xml --output-dir ./coverage
mv ./coverage/cargo-tarpaulin-report.xml ./coverage/cobertura.xml || true
# verifica backend/coverage/cobertura.xml
```

1. Ejecuta el scanner Sonar localmente (sin tocar CI) para ver el resultado antes de push:

- Opción rápida (Docker):

```powershell
docker run --rm -v ${PWD}:/usr/src sonarsource/sonar-scanner-cli \
  -Dsonar.host.url="https://sonarcloud.io" \
  -Dsonar.login="${{ env.SONAR_TOKEN }}" \
  -Dsonar.projectKey="amanahacademia" \
  -Dsonar.sources=frontend/src,backend/src \
  -Dsonar.javascript.lcov.reportPaths=frontend/coverage/lcov.info \
  -Dsonar.coverageReportPaths=backend/coverage/cobertura.xml
```

- Opción CLI (instalar sonar-scanner):

```bash
# instala sonar-scanner según tu OS y ejecuta
sonar-scanner \
  -Dsonar.projectKey=amanahacademia \
  -Dsonar.sources=frontend/src,backend/src \
  -Dsonar.javascript.lcov.reportPaths=frontend/coverage/lcov.info \
  -Dsonar.coverageReportPaths=backend/coverage/cobertura.xml \
  -Dsonar.login="$SONAR_TOKEN"
```

1. Cómo obtener `SONAR_TOKEN` para pruebas locales y consideraciones de seguridad

- Obtener un token en SonarCloud:

  1. Entra en https://sonarcloud.io y haz login con tu cuenta.
  2. Ve a tu organización → **My Account** (arriba a la derecha) → **Security** → **Tokens**.
  3. Crea un token nuevo, ponle un nombre descriptivo (por ejemplo `amanah-ci`) y copia el valor (no se mostrará otra vez).

- Alternativa: pide al mantenedor del repositorio que genere un token con permisos de análisis y lo añada a los Secrets del repositorio (recomendado para contributors que no tienen acceso a la organización SonarCloud).

- Añadir el token a GitHub (si eres mantenedor): Repository → Settings → Secrets and variables → Actions → New repository secret

  - Name: `SONAR_TOKEN`
  - Value: el token que obtuviste de SonarCloud

- Usarlo localmente (opcional, solo para pruebas):

```powershell
# PowerShell
$env:SONAR_TOKEN = "your_token_here"

# Bash
export SONAR_TOKEN=your_token_here
```

- Nota sobre forks y PRs de contribuyentes externos:
  - GitHub no expone los Secrets a workflows que se ejecutan desde forks por razones de seguridad. Si un contribuyente abre un PR desde un fork, el workflow en CI no podrá usar `SONAR_TOKEN` salvo que el maintainer lo re-ejecute desde la rama principal o el PR venga desde un branch del repositorio principal.
  - Recomendación: los contribuyentes deberían ejecutar el scanner localmente (no subir a SonarCloud) o pedir a los mantenedores que ejecuten el CI para validar el análisis.

1. Verifica que el scanner haya subido análisis (si usas SonarCloud) o que el reporte se genere localmente. Si prefieres no subir informes a SonarCloud durante pruebas locales, usa un SonarQube local o apunta a una instancia privada.

Con estos pasos puedes validar tus cambios antes de abrir un PR. El workflow en CI se encargará de ejecutar lo mismo automáticamente cuando empujes a la rama remota.

## 4) Ejecutar scanner localmente (opción Docker)

Para pruebas locales sin instalar el scanner, usa la imagen oficial:

```powershell
# PowerShell
docker run --rm -v ${PWD}:/usr/src sonarsource/sonar-scanner-cli \
  -Dsonar.host.url="https://sonarcloud.io" \
  -Dsonar.login="YOUR_TOKEN" \
  -Dsonar.projectKey="amanahacademia" \
  -Dsonar.sources=frontend/src,backend/src \
  -Dsonar.javascript.lcov.reportPaths=frontend/coverage/lcov.info \
  -Dsonar.coverageReportPaths=backend/coverage/cobertura.xml
```

En `cmd.exe` usa `%cd%` en lugar de `${PWD}`.

## 5) sonarqube / sonar-project.properties

Tu archivo `sonar-project.properties` ya incluye las claves esenciales. Recomendación:

- Mantén las exclusiones en `sonar.exclusions` (evita `node_modules`, `target`, `coverage`, `public`, etc.).
- Asegúrate de que las rutas en `sonar.javascript.lcov.reportPaths` y `sonar.coverageReportPaths` coincidan con las rutas que genera tu CI.

## 6) Consejos y troubleshooting

- Si Sonar no encuentra `lcov.info`, el análisis mostrará cobertura 0% para frontend; verifica path y que el archivo existe al ejecutar el scanner.
- Si `cargo-tarpaulin` falla en CI por permisos o dependencias nativas, intenta ejecutar en `ubuntu-latest` y asegura `libssl-dev` y `pkg-config` instalados.
- Para PRs: SonarCloud tiene integración automática para comentarios y calidad de gate si configuras `pull request analysis`.
