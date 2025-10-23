# Amanah Academia - Cloud Deployment

## Objetivo principal

Amanah Academia es una plataforma educativa online especializada en la enseñanza de idiomas, con énfasis en árabe y español, ofreciendo clases con profesores nativos, cursos, artículos y recursos interactivos.

## Tecnologías utilizadas

### Backend

- **Rust**: Lenguaje principal para el backend.
- **Axum**: Framework web para Rust, utilizado para construir la API REST.
- **Firebase Auth & Realtime DB**: Autenticación y base de datos en tiempo real.
- **Stripe**: Procesamiento de pagos.
- **Cal.com**: Gestión de reservas y calendario.
- **Mailchimp & Resend**: Gestión de emails y newsletters.

### Frontend

- **Astro**: Framework moderno para construir sitios web rápidos y optimizados.
- **Tailwind CSS**: Framework CSS utilitario para estilos responsivos.
- **TypeScript**: Tipado estático para mayor robustez.
- **SolidJS**: Integración para componentes reactivos.

### DevOps & Infraestructura

- **Cloudflare**: Hosting del frontend con CDN y seguridad.
- **Fly.io**: Hosting del backend con despliegue en contenedores.
- **GitHub Actions**: CI/CD para automatizar pruebas y despliegues.
- **Docker**: Contenerización de la aplicación backend.

## Despliegue en Cloudflare y Fly.io

El frontend se despliega en Cloudflare utilizando el adapter oficial de Astro (`@astrojs/cloudflare`). El archivo de configuración principal es [`frontend/astro.config.mjs`](frontend/astro.config.mjs).

El backend se despliega en Fly.io usando Docker, con configuración en [`backend/Dockerfile`](backend/Dockerfile) y [`backend/fly.toml`](backend/fly.toml).

## Variables de entorno

- **Frontend**: Configura las variables en [`frontend/.env`](frontend/.env).
- **Backend**: Configura las variables en [`backend/.env`](backend/.env).

## Seguridad y buenas prácticas

- SSL/TLS en todas las comunicaciones.
- Autenticación JWT segura.
- Protección contra XSS y CSRF.
- Validación de datos en frontend y backend.
- Integración segura con Stripe.
- Cumplimiento GDPR.
