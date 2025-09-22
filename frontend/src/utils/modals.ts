// Cerrar el modal con animación
export function closeModalAnimation(modal: HTMLDialogElement, form: HTMLFormElement | null = null) {
  modal.setAttribute("closing", "");
  modal.addEventListener(
    "animationend",
    () => {
      modal.removeAttribute("closing");
      modal.close();
    },
    { once: true }
  );
  form?.reset();
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

export function closeModalsEvents() {
  // Eventos para cerrar los modales
  document.addEventListener("mousedown", (e) => {
    // Obtenemos el elementos al que le hemos echo click y obtenemos el modal
    const target = e.target as HTMLElement;
    const modal = target.closest("dialog") as HTMLDialogElement;
    const formModal = modal ? modal.querySelector("form") : null;
    // Cerrar al hacer clic en cualquier parte del carrusel (incluida la imagen), al hacer clic en el backdrop, el botón de cancelar
    if (
      (target.closest(".embla__container") && modal) ||
      (target instanceof HTMLDialogElement && target.open) ||
      (target.getAttribute("aria-label") === "close-modal" && modal)
    )
      closeModalAnimation(modal, formModal);
  });

  // Cerrar con el botón de cancelar
  document.querySelectorAll("dialog").forEach((modal) => {
    modal.addEventListener("cancel", (e) => {
      e.preventDefault();
      const formModal = modal.querySelector("form");
      closeModalAnimation(modal, formModal);
    });
  });
}
