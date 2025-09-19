import type { Comment, Result } from "@/types/bakend-types";
import { toast } from "solid-toast";
import { ApiService } from "./helper";
import { getAuth } from "firebase/auth";

export async function submitLike(likeIcon: Element, likeCountSpan: HTMLSpanElement) {
  const helper = new ApiService();

  const auth = getAuth();
  const currentUser = auth.currentUser;

  if (!currentUser) {
    toast.error("You must be logged in to like comments");
    return;
  }

  const tokenID = await currentUser?.getIdToken();

  // Obtenemos el commentId del atributo data-id del likeIcon
  let commentId = likeIcon.getAttribute("data-id");
  if (!commentId) {
    console.error("Error: commentId not found");
    return;
  }

  // Llamamos a la API para registrar el like
  const commentResponse: Result<Comment> = await helper.setLike(tokenID, commentId);

  if (commentResponse.success) {
    // Obtenemos el span dentro de likeIcon y actualizamos el conteo
    likeCountSpan.innerText = String(commentResponse.data.like || 0);
  } else {
    // La respuesta no es válida, puedes mostrar un error o ignorar
    console.error("Error: la respuesta no es un Comment válido");
  }
}
