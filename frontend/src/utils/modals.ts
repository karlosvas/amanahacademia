// Cerrar el modal con animación
export function closeModalAnimation(modal: HTMLDialogElement) {
  modal.setAttribute("closing", "");
  modal.addEventListener(
    "animationend",
    () => {
      modal.removeAttribute("closing");
      modal.close();
    },
    { once: true }
  );
}

// Abrir el modal
export function showModalAnimation(modal: HTMLDialogElement, form: HTMLFormElement | null, background: boolean) {
  // Añadir clase de apertura antes de mostrar el modal
  modal.classList.add("modal-opening");
  // Mostrar el modal
  if (background) modal.showModal();
  else modal.show();

  if (form) {
    // Hacer focus al primer elemento después de que termine la animación
    const firstInput = form.querySelector("input, select, textarea, button");
    if (firstInput && firstInput instanceof HTMLElement) firstInput.focus();
  } else {
    if (document.activeElement instanceof HTMLElement) {
      document.activeElement.blur();
    }
  }

  // Después de la animación, quitar la clase de apertura
  setTimeout(() => {
    modal.classList.remove("modal-opening");
  }, 350);
}
