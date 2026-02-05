import type { Comment, ReplyComment, ResponseAPI } from "@/types/bakend-types";
import { ApiService } from "./helper";
import { FrontendErrorCode, getErrorToast } from "@/enums/enums";
import { getFirebaseAuth } from "./firebase";
import { FrontendError, isFrontendError } from "@/types/types";
import toast from "solid-toast";
import { log } from "./logger";
type User = import("firebase/auth").User;

export async function submitLike(
  likeIcon: Element,
  likeCountSpan: HTMLSpanElement,
) {
  const helper = new ApiService();

  // Obtenemos el commentId del atributo data-id del likeIcon
  const el = likeIcon as HTMLElement;
  const commentId = el.dataset.id;
  if (!commentId) {
    log.error("No comment ID found on like icon");
    return;
  }

  // Llamamos a la API para registrar el like
  const commentResponse: ResponseAPI<Comment> = await helper.setLike(commentId);

  if (commentResponse.success) {
    // Obtenemos el span dentro de likeIcon y actualizamos el conteo
    likeCountSpan.innerText = String(commentResponse.data.like || 0);
  } else {
    // La respuesta no es válida, puedes mostrar un error o ignorar
    log.error("Error submitting like", commentResponse);
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
    return (
      parseSpanishDate(a.timestamp).getTime() -
      parseSpanishDate(b.timestamp).getTime()
    );
  });

  // Obtener los 3 mejores comentarios por likes
  const bestComments = [...normalizedComments]
    .sort((a, b) => (b.like ?? 0) - (a.like ?? 0))
    .slice(0, 3);

  return {
    comments: sortedComments,
    bestComments: bestComments,
  };
}

// Función para actualizar el estado visual del "like" según el usuario actual
export function updateLikeState(
  user: User | null,
  likeIcon: Element | null,
  comment: string | null,
) {
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

export async function verifyAuthorInComment(
  helper: ApiService,
  commentId: string,
) {
  const comment: ResponseAPI<Comment> = await helper.getCommentById(commentId);
  if (!comment.success)
    throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));

  const auth = getFirebaseAuth();
  if (auth.currentUser?.uid !== comment.data.author_uid)
    throw new FrontendError(getErrorToast(FrontendErrorCode.MUST_BE_OWNER));
}

export async function verifyAuthorInCommentReply(
  helper: ApiService,
  commentId: string,
  replyId: string,
) {
  const comment: ResponseAPI<Comment> = await helper.getCommentReplyById(
    commentId,
    replyId,
  );
  if (!comment.success)
    throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));

  const auth = getFirebaseAuth();
  if (auth.currentUser?.uid !== comment.data.author_uid)
    throw new FrontendError(getErrorToast(FrontendErrorCode.MUST_BE_OWNER));
}

// Función para manejar el envío de respuestas
export async function handleSubmitReply(
  helper: ApiService,
  commentEl: HTMLElement,
  traductions: any,
) {
  try {
    const commentDiv = commentEl.closest("[data-comment-id]") as HTMLElement;
    if (!commentDiv)
      throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));

    const commentId = commentDiv.dataset.commentId;
    if (!commentId)
      throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));

    const replyForm = commentDiv.querySelector<HTMLElement>(".reply-form");
    if (!replyForm)
      throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));

    const textarea =
      replyForm.querySelector<HTMLTextAreaElement>(".reply-textarea");
    if (!textarea)
      throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));

    const content = textarea.value.trim();
    if (!content)
      throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));

    const auth = getFirebaseAuth();
    if (!auth.currentUser)
      throw new FrontendError(
        getErrorToast(FrontendErrorCode.NEED_AUTHENTICATION),
      );

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

    const res: ResponseAPI<ReplyComment> = await helper.createReply(
      commentId,
      newReply,
    );

    if (!res.success) {
      log.error("Error creating reply", {
        error: res.error,
      });
      throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));
    }

    // Mostrar mensaje de éxito y recargar la página para renderizar desde SSR
    toast.success("Respuesta añadida correctamente");
    setTimeout(() => {
      location.reload();
    }, 1000);
  } catch (e) {
    isFrontendError(e)
      ? toast.error(e.message)
      : toast.error(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));
  }
}

// Función para manejar la eliminación de comentarios
export async function handleDeleteComment(
  helper: ApiService,
  commentId: string,
) {
  await verifyAuthorInComment(helper, commentId);
  const res: ResponseAPI<void> = await helper.deleteComment(commentId);

  if (!res.success) {
    throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));
  }

  return res;
}

// Función para manejar la edición de comentarios
export async function handleEditComment(helper: ApiService, commentId: string) {
  await verifyAuthorInComment(helper, commentId);
}

// Función para manejar la eliminación de respuestas
export async function handleDeleteReply(
  helper: ApiService,
  commentId: string,
  replyId: string,
) {
  await verifyAuthorInCommentReply(helper, commentId, replyId);
  const result: ResponseAPI<void> = await helper.deleteReply(
    commentId,
    replyId,
  );

  if (!result.success) {
    throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));
  }

  return result;
}

// Función para manejar la edición de respuestas
export async function handleEditReply(
  helper: ApiService,
  commentId: string,
  replyId: string,
  newContent: string,
  currentContent: string,
) {
  if (!newContent || newContent === currentContent) {
    return null;
  }

  const auth = getFirebaseAuth();
  if (!auth.currentUser)
    throw new FrontendError(
      getErrorToast(FrontendErrorCode.NEED_AUTHENTICATION),
    );

  const reply: ReplyComment = {
    id: replyId,
    content: newContent,
    timestamp: new Date().toISOString(),
    author_uid: auth.currentUser.uid,
    name: auth.currentUser.displayName || "Anónimo",
    url_img: auth.currentUser.photoURL || null,
    like: 0,
    users_liked: [],
  };

  const result: ResponseAPI<ReplyComment> = await helper.editReply(
    commentId,
    replyId,
    reply,
  );

  if (!result.success) {
    throw new FrontendError(getErrorToast(FrontendErrorCode.UNKNOWN_ERROR));
  }

  return result;
}
