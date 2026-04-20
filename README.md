# 🎓 Amanah Academia - README

![Amanah Academia](https://img.shields.io/badge/Platform-Education-orange)
![Version](https://img.shields.io/badge/Version-1.0.0-blue)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

## 🎯 Descripción del Proyecto

**Amanah Academia** es una plataforma educativa innovadora especializada en la enseñanza del español mediante un enfoque culturalmente inmersivo.
Dirigida por Ana Ortega, conecta a estudiantes con profesores nativos que enseñan español en el idioma nativo del estudiante,
facilitando un aprendizaje más natural y efectivo.

**Valores clave:**

- ✨ Experiencia cultural inmersiva
- 👥 Profesores nativos especializados por idioma
- 🎯 Enfoque personalizado según objetivos del estudiante
- ⏰ Flexibilidad horaria completa

### 🎓 Modalidades de Enseñanza

- **Clases individuales estándar** (30€/clase)
- **Clases de conversación** (20€/clase)
- **Clases grupales** (10€/clase)
- **Clase de prueba gratuita** sin compromiso

### 📚 Recursos Adicionales

- **Artículos educativos** gratuitos
- **Podcast** con conversaciones reales y consejos
- **Cursos** compra cursos empaquetando todo lo necesario para aprender

### 🔐 Sistema de Seguridad

- **Security Hall of Fame** que reconoce contribuciones de seguridad
- Políticas de privacidad robustas
- Protección de datos del estudiante

## 🛠️ Tecnologías Utilizadas

### Frontend

![Astro](https://img.shields.io/badge/Astro-FF5D01?style=for-the-badge&logo=astro&logoColor=white)
![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-38B2AC?style=for-the-badge&logo=tailwind-css&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)

Astro - Framework web todo-en-uno para construir sitios rápidos y optimizados  
Tailwind CSS - Framework CSS utilitario para diseños responsivos y modernos  
TypeScript - Superset de JavaScript con tipado estático para mayor robustez

### Backend

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-000000?style=for-the-badge)
![Firebase](https://img.shields.io/badge/Firebase-FFCA28?style=for-the-badge&logo=firebase&logoColor=black)
![Stripe](https://img.shields.io/badge/Stripe-008CDD?style=for-the-badge&logo=stripe&logoColor=white)
![Cal.com](https://img.shields.io/badge/Cal.com-000000?style=for-the-badge)

Rust - Lenguaje de programación seguro y de alto rendimiento  
Axum - Framework web ergonómico y modular para Rust  
Firebase Realtime DB - Base de datos en tiempo real para sincronización instantánea  
Firebase Auth - Sistema de autenticación seguro y escalable  
Stripe - Pasarela de pagos para procesamiento de transacciones seguras  
Cal.com - Sistema de gestión de calendario y reservas integrado

### DevOps & Infraestructura

![Fly.io](https://img.shields.io/badge/Fly.io-8B5CF6?style=for-the-badge)
![Cloudflare](https://img.shields.io/badge/Cloudflare-F38020?style=for-the-badge&logo=Cloudflare&logoColor=white)
![GitHub Actions](https://img.shields.io/badge/GitHub_Actions-2088FF?style=for-the-badge&logo=github-actions&logoColor=white)
![SonarCloud](https://img.shields.io/badge/SonarCloud-000000?style=for-the-badge)
![Vercel](https://img.shields.io/badge/Vercel-000000?style=for-the-badge)
![Firebase](https://img.shields.io/badge/Firebase-FFCA28?style=for-the-badge&logo=firebase&logoColor=black)

Firebase Auth - Gestión de autenticación y usuarios  
Firebase Realtime DB - Base de datos en tiempo real para sincronización instantánea
SonarCloud - Análisis de calidad de código y cobertura de pruebas  
Vercel - Plataforma de despliegue para frontend con optimización global  
Fly.io - Plataforma de despliegue global para aplicaciones edge  
Cloudflare - CDN, DNS y servicios de seguridad para optimización  
GitHub Actions - CI/CD automatizado para integración y despliegue continuo

## ⚙️ Instalación y Configuración

### Pasos de instalación

1. **Clonar el repositorio**

```bash
git clone https://github.com/tu-usuario/amanah-academia.git
cd amanah-academia
```

#### Frontend

```
cd backend
cargo run
```

#### Backend

```bash
cd frontend
npm install
npm run dev
```

## 🤝 Contribución

¡Agradecemos las contribuciones a Amanah Academia!  
Para contribuir: Haz fork del proyecto , sigue los pasos y haz un PullRequest(PR)

```bash
git branch feature/foo  // Crear rama
git switch foo          // Cambiar de rama
git add .               // Añadir los cambios al stage
git commit -m "foo"     // Subir los cambios
git push origin foo     // Push a la rama remota
```

Reportar Vulnerabilidades
Si descubres una vulnerabilidad de seguridad, por favor repórtala de manera responsable a:  
Email: security@amanahacademia.com  
Política: Consulta nuestra Security Hall of Fame

## 🔒 Seguridad

- 🔒 Encriptación SSL/TLS para todas las comunicaciones
- 🔑 Autenticación JWT segura
- 🛡️ Protección contra ataques XSS y CSRF
- 🧩 Validación de datos en frontend y backend
- 🔍 Auditorías de seguridad regulares
- 💳 Integración segura con Stripe para pagos
- 📜 Cumplimiento con políticas de privacidad y GDPR
- 🎖️ Reconocimiento en nuestro Security Hall of Fame

## 🧰 Calidad / Sonar

Analizamos la calidad del código con SonarCloud. Si quieres ejecutar los mismos checks localmente antes de abrir un PR, sigue las instrucciones en `docs/sonar.md` (genera cobertura y ejecuta el scanner localmente).

El workflow de CI ya está incluido en `.github/workflows/sonar.yml` y se ejecutará automáticamente en pushes a `main` y en Pull Requests.

## 📄 Licencia

Este proyecto está bajo una Licencia GPL. Ver el archivo LICENSE para detalles.

## 📞 Contacto

Sitio web: [amanahacademia.com](amanahacademia.com)  
Email: suport@amanahacademia.com
