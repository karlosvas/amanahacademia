// Cerrar el modal con animación
export function closeModalAnimation(modal: HTMLDialogElement, form: HTMLFormElement | null = null) {
  modal.setAttribute("closing", "");
  modal.addEventListener(
    "animationend",
    () => {
      modal.removeAttribute("closing");
      modal.close();
      // Restaurar el scroll del body y html
      document.body.style.overflow = "";
      document.documentElement.style.overflow = "";
      document.body.style.paddingRight = "";

      // Restaurar ancho del header
      const header = document.querySelector("header");
      if (header instanceof HTMLElement) {
        header.style.width = "";
        header.removeAttribute("data-original-width");
      }

      // Restaurar posición del menú de navegación
      const selectPage = document.getElementById("select-page");
      if (selectPage instanceof HTMLElement) {
        selectPage.style.left = "";
        selectPage.removeAttribute("data-original-left");
      }

      // Restaurar padding original de otros elementos fixed
      const fixedElements = document.querySelectorAll(".fixed");
      fixedElements.forEach((el) => {
        if (el instanceof HTMLElement) {
          const originalPadding = el.getAttribute("data-original-padding");
          if (originalPadding) {
            el.style.paddingRight = `${originalPadding}px`;
            el.removeAttribute("data-original-padding");
          } else {
            el.style.paddingRight = "";
          }
        }
      });
    },
    { once: true }
  );
  form?.reset();
}

// Abrir el modal
export function showModalAnimation(modal: HTMLDialogElement, form: HTMLFormElement | null, background: boolean) {
  // Calcular el ancho de la scrollbar para evitar el salto de contenido
  const scrollbarWidth = window.innerWidth - document.documentElement.clientWidth;

  // Bloquear el scroll del body y html
  if (background) {
    document.body.style.overflow = "hidden";
    document.documentElement.style.overflow = "hidden";
  }

  // Compensar el ancho de la scrollbar para evitar el salto de contenido
  if (scrollbarWidth > 0 && background) {
    // Guardar y sumar el padding original del body
    const bodyPaddingRight = parseInt(window.getComputedStyle(document.body).paddingRight) || 0;
    document.body.style.paddingRight = `${bodyPaddingRight + scrollbarWidth}px`;

    // Para el header: fijar su ancho ANTES de que el viewport cambie
    const header = document.querySelector("header");
    if (header instanceof HTMLElement) {
      // Guardamos el ancho actual del header (incluyendo la scrollbar)
      const currentWidth = header.offsetWidth;
      header.style.width = `${currentWidth}px`;
      header.setAttribute("data-original-width", currentWidth.toString());
    }

    // Aplicar compensación al menú de navegación para evitar desplazamiento
    const selectPage = document.getElementById("select-page");
    if (selectPage instanceof HTMLElement) {
      const currentLeft = selectPage.offsetLeft;
      selectPage.style.left = `${currentLeft - scrollbarWidth / 2}px`;
      selectPage.setAttribute("data-original-left", currentLeft.toString());
    }

    // Aplicar padding a otros elementos fixed
    const fixedElements = document.querySelectorAll(".fixed");
    fixedElements.forEach((el) => {
      if (el instanceof HTMLElement) {
        const currentPadding = parseInt(window.getComputedStyle(el).paddingRight) || 0;
        el.style.paddingRight = `${currentPadding + scrollbarWidth}px`;
        // Guardar el padding original como data attribute
        el.setAttribute("data-original-padding", currentPadding.toString());
      }
    });
  }

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
    if (document.activeElement instanceof HTMLElement) document.activeElement.blur();
  }

  // Después de la animación, quitar la clase de apertura
  setTimeout(() => {
    modal.classList.remove("modal-opening");
  }, 350);
}

// Eliminar eventos de cerrar modales
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
