import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ request, cookies }) => {
  // Verificar el método HTTP
  if (request.method !== "POST") {
    return new Response("Método no permitido", {
      status: 405,
      headers: { Allow: "POST" },
    });
  }

  // Parsear y validar JSON
  const body = await request.json();
  const { theme } = body;

  if (!theme || (theme !== "light" && theme !== "dark")) {
    return new Response(
      JSON.stringify({
        error: 'Tema inválido. Use "light" o "dark"',
      }),
      {
        status: 400,
        headers: { "Content-Type": "application/json" },
      }
    );
  }

  // Configurar cookie con opciones de seguridad
  cookies.set("theme", theme, {
    path: "/",
    maxAge: 60 * 60 * 24 * 365, // 1 año en segundos
    sameSite: "lax",
    secure: import.meta.env.PROD, // HTTPS en producción
    httpOnly: true, // Protección contra XSS
  });

  return new Response(
    JSON.stringify({
      message: "Tema actualizado correctamente",
      theme: theme,
    }),
    {
      status: 200,
      headers: { "Content-Type": "application/json" },
    }
  );
};
