import type { Comment, ReplyComment, Result } from "@/types/bakend-types";
import { ApiService } from "./helper";
import { FrontendErrorCode, getErrorToast } from "@/enums/enums";
import { getFirebaseAuth } from "./firebase";
import { FrontendError, isFrontendError } from "@/types/types";
import toast from "solid-toast";
type User = import("firebase/auth").User;

export async function submitLike(likeIcon: Element, likeCountSpan: HTMLSpanElement) {
  const helper = new ApiService();

  // Obtenemos el commentId del atributo data-id del likeIcon
  let commentId = likeIcon.getAttribute("data-id");
  if (!commentId) {
    console.error("Error: commentId not found");
    return;
  }

  // Llamamos a la API para registrar el like
  const commentResponse: Result<Comment> = await helper.setLike(commentId);

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

export async function verifyAuthorInComment(helper: ApiService, commentId: string) {
  const comment: Result<Comment> = await helper.getCommentById(commentId);
  if (!comment.success) throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));

  const auth = await getFirebaseAuth();
  if (auth.currentUser?.uid !== comment.data.author_uid)
    throw new FrontendError(getErrorToast(FrontendErrorCode.MUST_BE_OWNER));
}

export async function verifyAuthorInCommentReply(helper: ApiService, commentId: string, replyId: string) {
  const comment: Result<Comment> = await helper.getCommentReplyById(commentId, replyId);
  if (!comment.success) throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));

  const auth = await getFirebaseAuth();
  if (auth.currentUser?.uid !== comment.data.author_uid)
    throw new FrontendError(getErrorToast(FrontendErrorCode.MUST_BE_OWNER));
}

// Función para manejar el envío de respuestas
export async function handleSubmitReply(helper: ApiService, commentEl: HTMLElement, traductions: any) {
  try {
    console.log("handleSubmitReply ejecutado");
    const commentDiv = commentEl.closest("[data-comment-id]");
    if (!commentDiv) throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));

    const commentId: string = commentDiv.getAttribute("data-comment-id") as string;
    if (!commentId) throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));

    const replyForm = commentDiv.querySelector<HTMLElement>(".reply-form");
    if (!replyForm) throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));

    const textarea = replyForm.querySelector<HTMLTextAreaElement>(".reply-textarea");
    if (!textarea) throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));

    const content = textarea.value.trim();
    if (!content) throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));

    const auth = await getFirebaseAuth();
    if (!auth.currentUser) throw new FrontendError(getErrorToast(FrontendErrorCode.NEED_AUTHENTICATION));

    const newReply: ReplyComment = {
      id: "", // El backend asignará el ID
      content,
      timestamp: new Date().toISOString(),
      author_uid: auth.currentUser.uid,
      name: auth.currentUser.displayName || "Anónimo",
      url_img: auth.currentUser.photoURL || null,
      like: 0,
      users_liked: [],
    };

    console.log("Enviando reply:", { commentId, newReply });
    const res: Result<ReplyComment> = await helper.createReply(commentId, newReply);
    console.log("Respuesta del servidor:", res);

    if (!res.success) {
      console.error("Error creando respuesta:", {
        error: res.error,
        message: res.error?.message,
        statusCode: res.error?.statusCode,
        type: res.error?.type,
      });
      throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));
    }

    // Actualizar el DOM en lugar de recargar la página
    const repliesSection = commentDiv.querySelector(".replies-section");
    const repliesContainer = commentDiv.querySelector(".replies-container");

    // Si no existe la sección de respuestas, crearla
    if (!repliesSection || !repliesContainer) {
      const newRepliesSection = document.createElement("div");
      newRepliesSection.className = "replies-section px-4 py-3 bg-[#fff5f2] border-t border-salmon";
      newRepliesSection.innerHTML = `
          <div class="text-xs font-medium mb-2 text-gray-600 replies-count">
            1 ${traductions.response}
          </div>
          <div class="space-y-3 replies-container"></div>
        `;
      commentDiv.appendChild(newRepliesSection);
    }

    // Añadir la nueva respuesta al DOM
    const container = commentDiv.querySelector(".replies-container");
    if (container) {
      const newReplyElement = document.createElement("div");
      newReplyElement.className = "reply-item flex space-x-2 p-2 bg-lightSalmon rounded-md shadow-sm relative";
      newReplyElement.setAttribute("data-reply-id", res.data.id);
      newReplyElement.setAttribute("data-reply-author", res.data.author_uid);
      newReplyElement.innerHTML = `
          <div class="w-8 h-8 rounded-full overflow-hidden flex-shrink-0">
            ${res.data.url_img ? `<img src="${res.data.url_img}" alt="${res.data.name}" class="w-full h-full object-cover" />` : `<div class="w-full h-full bg-salmon flex items-center justify-center text-xs font-bold">${res.data.name.charAt(0).toUpperCase()}</div>`}
          </div>
          <div class="flex-1 pr-4">
            <div class="flex items-center justify-between">
              <span class="text-xs font-medium">${res.data.name}</span>
              <div class="flex items-center space-x-2">
                <time class="text-xs text-gray-700">${res.data.timestamp}</time>
              </div>
            </div>
            <p class="text-xs mt-1 text-gray-700 reply-content">${res.data.content}</p>
          </div>
        `;
      container.appendChild(newReplyElement);

      // Actualizar contador
      const countElement = commentDiv.querySelector(".replies-count");
      if (countElement) {
        const newCount = container.children.length;
        countElement.textContent = `${newCount} ${newCount === 1 ? traductions.response : traductions.responses}`;
      }
    }

    // Limpiar y ocultar el formulario
    textarea.value = "";
    replyForm.classList.add("hidden");
    toast.success("Respuesta añadida correctamente");
  } catch (e) {
    isFrontendError(e) ? toast.error(e.message) : toast.error(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));
  }
}
