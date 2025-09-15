import type { APIRoute } from "astro";
import { adminAuth } from "@/config/firebase-admin"; // SDK Admin de Firebase

// Envía y crea una sesión segura con cookies HTTP only
export const POST: APIRoute = async ({ request, cookies }) => {
  const { token } = await request.json();

  try {
    // Valida el token con Firebase Admin
    const decoded = await adminAuth.verifyIdToken(token);

    // Crear objeto con toda la información necesaria
    const sessionData = {
      jwt: token, // Token JWT completo
      localId: decoded.uid, // Firebase UID del usuario
      email: decoded.email, // Email del usuario (opcional)
      name: decoded.name, // Nombre del usuario (opcional)
      exp: decoded.exp, // Expiración del token
      picture: decoded.picture, // Avatar del usuario
      emailVerified: decoded.email_verified, // Si el email está verificado
      provider: decoded.firebase?.sign_in_provider, // google.com, etc.
    };

    // Crea una cookie de sesión segura
    cookies.set("session", JSON.stringify(sessionData), {
      path: "/",
      httpOnly: true,
      secure: true,
      sameSite: "strict",
      maxAge: 60 * 60 * 24 * 7, // 1 semana
    });

    return new Response(JSON.stringify({ success: true }), { status: 200 });
  } catch (err) {
    console.error("Error verifying token:", err);
    return new Response(JSON.stringify({ error: "Invalid token" }), { status: 401 });
  }
};

// Obtiene la sesión (si existe)
export const GET: APIRoute = async ({ cookies }) => {
  const sessionCookie = cookies.get("session")?.value;
  if (!sessionCookie) {
    return new Response(JSON.stringify({ error: "No session" }), { status: 401 });
  }

  try {
    const sessionData = JSON.parse(sessionCookie);

    // Verificar que el JWT sigue siendo válido
    await adminAuth.verifyIdToken(sessionData.jwt);

    // Devolver el objeto completo
    return new Response(JSON.stringify(sessionData), {
      status: 200,
      headers: { "Content-Type": "application/json" },
    });
  } catch (err) {
    console.error("Error verifying session:", err);
    cookies.delete("session");
    return new Response(JSON.stringify({ error: "Invalid or expired session" }), {
      status: 401,
    });
  }
};
