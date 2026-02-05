import { log } from "@/services/logger";
import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ request, cookies }) => {
  try {
    const { token } = await request.json();

    if (!token || typeof token !== "string") {
      log.error("Invalid token received in /api/auth");
      return new Response(JSON.stringify({ error: "Invalid token" }), {
        status: 400,
      });
    }

    // Guardar en cookie httpOnly
    cookies.set("session", token, {
      path: "/",
      httpOnly: true,
      secure: import.meta.env.PROD,
      sameSite: "lax",
      maxAge: 60 * 60 * 24 * 7,
    });

    return new Response(JSON.stringify({ success: true }), {
      status: 200,
      headers: { "Content-Type": "application/json" },
    });
  } catch (error) {
    log.error("Error en POST /api/auth:", error);
    return new Response(
      JSON.stringify({ error: "Error interno del servidor" }),
      { status: 500 },
    );
  }
};

export const DELETE: APIRoute = async ({ cookies }) => {
  cookies.delete("session", { path: "/" });

  return new Response(JSON.stringify({ success: true }), {
    status: 200,
    headers: { "Content-Type": "application/json" },
  });
};
