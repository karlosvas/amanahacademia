import type { Comment, ResponseAPI } from "@/types/bakend-types";
import { toast } from "solid-toast";

export async function submitLike(likeIcon: HTMLDivElement, likeCountSpan: HTMLSpanElement) {
  // Obtenemos el token de Astro server
  const response = await fetch("/api/session", {
    method: "GET",
    credentials: "include",
  });

  const sessionData = await response.json();
  // Verificar que tenemos los datos de sesión válidos
  if (typeof sessionData?.jwt !== "string") {
    console.error("Error: Invalid session data");
    toast.error("Error has occurred, please log in again");
    return;
  }

  const url_backend = import.meta.env.PUBLIC_BACKEND_URL;
  if (!url_backend) {
    console.error("Error: BACKEND_URL not defined");
    toast.error("Error has occurred, please contact support");
    return;
  }

  let commentId = likeIcon.getAttribute("data-id");
  if (!commentId) {
    console.error("Error: commentId not found");
    return;
  }

  const commentResponse = await fetch(url_backend + "/comments/like/" + commentId, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + sessionData.jwt,
    },
  });

  if (commentResponse.ok) {
    const data: ResponseAPI<Comment> = await commentResponse.json();
    if (data.success && isComment(data.data)) {
      // Obtenemos el span dentro de likeIcon y actualizamos el conteo
      likeCountSpan.innerText = String(data.data.like || 0);
    } else {
      // La respuesta no es válida, puedes mostrar un error o ignorar
      console.error("Error: la respuesta no es un Comment válido");
    }
  }
}

function isComment(obj: any): obj is Comment {
  return (
    obj &&
    typeof obj === "object" &&
    typeof obj.name === "string" &&
    typeof obj.timestamp === "string" &&
    typeof obj.content === "string" &&
    typeof obj.url_img === "string" &&
    (typeof obj.author_uid === "undefined" || typeof obj.author_uid === "string") &&
    (typeof obj.like === "undefined" || typeof obj.like === "number") &&
    (typeof obj.reply === "undefined" || Array.isArray(obj.reply)) &&
    (typeof obj.users_liked === "undefined" || Array.isArray(obj.users_liked))
  );
}
