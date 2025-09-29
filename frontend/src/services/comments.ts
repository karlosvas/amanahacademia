import type { Comment, Result } from "@/types/bakend-types";
import { toast } from "solid-toast";
import { ApiService } from "./helper";
import { getAuth, type User } from "firebase/auth";
import { FrontendErrorCode, getErrorToast } from "@/enums/enums";

export async function submitLike(likeIcon: Element, likeCountSpan: HTMLSpanElement) {
  const helper = new ApiService();

  const auth = getAuth();
  const currentUser = auth.currentUser;

  if (!currentUser) {
    toast.error(getErrorToast(FrontendErrorCode.NEED_AUTHENTICATION));
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

function formatToSpanishDate(isoTimestamp: string): string {
  const date = new Date(isoTimestamp);

  const day = date.getDate().toString().padStart(2, "0");
  const month = (date.getMonth() + 1).toString().padStart(2, "0");
  const year = date.getFullYear();
  const hours = date.getHours().toString().padStart(2, "0");
  const minutes = date.getMinutes().toString().padStart(2, "0");

  return `${day}/${month}/${year} ${hours}:${minutes}`;
}

// Función para parsear fecha española a objeto Date
function parseSpanishDate(spanishDate: string): Date {
  // Formato: "20/09/2025 00:25"
  const [datePart, timePart] = spanishDate.split(" ");
  const [day, month, year] = datePart.split("/").map(Number);
  const [hours, minutes] = timePart.split(":").map(Number);

  return new Date(year, month - 1, day, hours, minutes);
}

function normalizeTimestamp(timestamp: string): string {
  // Si ya está en formato español, lo devuelve tal como está
  if (timestamp.includes("/")) {
    return timestamp;
  }

  // Si está en formato ISO, lo convierte
  if (timestamp.includes("T") || timestamp.includes("Z")) {
    return formatToSpanishDate(timestamp);
  }

  // Si no reconoce el formato, devuelve el original
  return timestamp;
}

// Función principal para procesar y ordenar comentarios
export function processComments(comments: Comment[]) {
  // Normalizar todos los timestamps
  const normalizedComments = comments.map((comment: Comment) => ({
    ...comment,
    timestamp: normalizeTimestamp(comment.timestamp),
  }));

  // Ordenar por fecha (más antiguos primero)
  const sortedComments = normalizedComments.toSorted((a, b) => {
    return parseSpanishDate(a.timestamp).getTime() - parseSpanishDate(b.timestamp).getTime();
  });

  // Obtener los 3 mejores comentarios por likes
  const bestComments = [...normalizedComments].sort((a, b) => (b.like ?? 0) - (a.like ?? 0)).slice(0, 3);

  return {
    comments: sortedComments,
    bestComments: bestComments,
  };
}

// Función para actualizar el estado visual del "like" según el usuario actual
export function updateLikeState(user: User | null, likeIcon: Element | null, comment: string | null) {
  if (!comment || !likeIcon) return;
  // Si el usuario ha dado like, añade la clase "liked"
  if (user && comment.includes(user.uid)) {
    likeIcon.classList.add("liked");
    const svg = likeIcon.querySelector(".like-svg");
    if (svg) svg.classList.add("liked");
  } else {
    // Si no ha dado like, quita la clase "liked"
    likeIcon.classList.remove("liked");
    const svg = likeIcon.querySelector(".like-svg");
    if (svg) svg.classList.remove("liked");
  }
}

export function updateModalUser(user: User | null) {
  const modal = document.getElementById("comment-modal");
  if (!modal) {
    console.error("Modal element not found.");
    return;
  }
  const avatarNameElem = modal.querySelector("#avatar-name") as HTMLSpanElement;
  const avatarContainer = modal.querySelector("#avatar-container") as HTMLDivElement;
  const avatarImg = modal.querySelector(".avatar-img") as HTMLImageElement;
  const avatarDefault = modal.querySelector("#avatar-default") as HTMLDivElement;

  if (!avatarNameElem || !avatarContainer || !avatarImg || !avatarDefault) {
    console.warn("One or more avatar elements not found.");
    return;
  }

  if (user) {
    if (avatarNameElem) {
      avatarNameElem.textContent = user.displayName || "Anonimo";
    }

    if (user.photoURL) {
      avatarImg.src = user.photoURL;
      avatarImg.style.display = "block";
      avatarDefault.style.display = "none";
    } else {
      avatarImg.style.display = "none";
      avatarDefault.style.display = "flex";
      // Oculta la imagen y muestra el SVG
      (avatarImg as HTMLImageElement).src = "";
      // Si el SVG ya está en el DOM, solo muéstralo
    }
  }
}
