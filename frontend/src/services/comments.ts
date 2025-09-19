import type { Comment, Result, SessionData } from "@/types/bakend-types";
import { toast } from "solid-toast";
import { ApiService } from "./helper";

export async function submitLike(likeIcon: HTMLDivElement, likeCountSpan: HTMLSpanElement) {
  const helper = new ApiService();

  // Obtenemos el token de Astro server
  const sessionData: Result<SessionData> = await helper.getSession();

  if (!sessionData.success) {
    console.error("Error fetching session data:", sessionData.error);
    toast.error("Error has occurred, please log in again");
    return;
  }

  // Verificar que tenemos los datos de sesión válidos
  if (typeof sessionData.data.token !== "string") {
    console.error("Error: Invalid session data");
    toast.error("Error has occurred, please log in again");
    return;
  }

  // Obtenemos el commentId del atributo data-id del likeIcon
  let commentId = likeIcon.getAttribute("data-id");
  if (!commentId) {
    console.error("Error: commentId not found");
    return;
  }

  // Llamamos a la API para registrar el like
  const commentResponse: Result<Comment> = await helper.setLike(sessionData.data.token, commentId);

  if (commentResponse.success) {
    // Obtenemos el span dentro de likeIcon y actualizamos el conteo
    likeCountSpan.innerText = String(commentResponse.data.like || 0);
  } else {
    // La respuesta no es válida, puedes mostrar un error o ignorar
    console.error("Error: la respuesta no es un Comment válido");
  }
}
