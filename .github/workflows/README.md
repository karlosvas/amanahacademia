# GitHub Actions Workflows

Este directorio contiene los workflows de CI/CD para Amanah Academia.

## Workflows Disponibles

### 📝 workflow.yml - Format & Build Check
**Trigger**: Push a `main`

Ejecuta:
- ✅ Verificación de formato con Prettier
- ✅ Build del proyecto frontend

### 🧪 frontend-tests.yml - Frontend Quick Tests
**Trigger**:
- Push a branches de desarrollo (`develop`, `feature/*`, `fix/*`, `claude/*`)
- Pull Requests a `main` o `develop`

Ejecuta:
- ✅ Tests unitarios del frontend
- ✅ Generación de coverage
- ✅ Upload a Codecov
- ✅ Comentario en PR con métricas

**Propósito**: Proporcionar feedback rápido durante el desarrollo.

### 🔍 sonar.yml - SonarCloud Analysis
**Trigger**:
- Push a `main`
- Pull Requests (opened, synchronize, reopened)
- Manual (workflow_dispatch)

Ejecuta:
- ✅ Tests frontend con coverage
- ✅ Tests backend con tarpaulin
- ✅ Análisis de calidad en SonarCloud
- ✅ Quality Gate check

**Propósito**: Análisis completo de calidad de código para producción.

## Flujo de Trabajo Recomendado

```
1. Desarrollador crea branch feature/nueva-funcionalidad
   ↓
2. Push al branch → frontend-tests.yml se ejecuta (feedback rápido)
   ↓
3. Crea Pull Request → frontend-tests.yml + sonar.yml se ejecutan
   ↓
4. Merge a main → workflow.yml + sonar.yml se ejecutan
```

## Estrategia de Optimización

### ¿Por qué múltiples workflows?

**frontend-tests.yml** (rápido):
- Se ejecuta en cada push a branches de desarrollo
- Solo tests del frontend
- Feedback en ~2-3 minutos
- Ideal para desarrollo iterativo

**sonar.yml** (completo):
- Se ejecuta en main y PRs
- Tests frontend + backend + análisis SonarCloud
- ~5-8 minutos
- Análisis completo de calidad

### Ventajas:
- ⚡ Feedback rápido durante desarrollo
- 🔍 Análisis profundo antes de merge
- 💰 Ahorro de minutos de CI/CD
- 🚀 No bloquea el desarrollo con análisis lentos

## Mantenimiento

### Agregar nuevo workflow:
1. Crear archivo `.yml` en esta carpeta
2. Actualizar este README con descripción
3. Documentar triggers y propósito

### Modificar workflow existente:
1. Editar archivo correspondiente
2. Actualizar documentación si cambian triggers
3. Probar en branch de desarrollo primero

## Dependencias

### Frontend
```bash
pnpm install --frozen-lockfile
```
**Nota**: el gestor es pnpm, igual que en Vercel, para que CI y el deploy resuelvan el mismo arbol de dependencias. El `ignore-scripts=true` de `frontend/.npmrc` se aplica solo; `sharp`, `esbuild` y `@sentry/cli` traen binarios precompilados y no necesitan postinstall.

### Backend
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Xml --output-dir coverage
```

## Troubleshooting

### Error: "Failed to install dependencies"
- Verifica que uses `--legacy-peer-deps --ignore-scripts`
- Revisa conflictos de peer dependencies

### Error: "Tests failing in CI but passing locally"
- Verifica mocks en `test/setup.ts`
- Revisa variables de entorno
- Comprueba timeouts (algunos tests pueden ser lentos en CI)

### Error: "SonarCloud analysis failed"
- Verifica que `SONAR_TOKEN` esté configurado en GitHub Secrets
- Revisa que `coverage/lcov.info` se genere correctamente
- Comprueba configuración en `sonar-project.properties`

## Recursos

- [GitHub Actions Docs](https://docs.github.com/en/actions)
- [Vitest CI Guide](https://vitest.dev/guide/ci.html)
- [SonarCloud GitHub Action](https://github.com/SonarSource/sonarcloud-github-action)
