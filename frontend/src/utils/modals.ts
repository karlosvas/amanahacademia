// Cerrar el modal con animación
export function closeModalAnimation(modal: HTMLDialogElement, form: HTMLFormElement | null = null) {
  modal.setAttribute("closing", "");

  modal.addEventListener(
    "animationend",
    () => {
      modal.removeAttribute("closing");
      modal.close();

      // Guardar la posición del scroll antes de restaurar los estilos
      const scrollY = modal.dataset.scrollPosition;

      // Restaurar el scroll del body y html
      document.body.style.overflow = "";
      document.documentElement.style.overflow = "";
      document.body.style.paddingRight = "";

      // Restaurar ancho del header
      const header = document.querySelector("header");
      if (header instanceof HTMLElement) {
        header.style.width = "";
        delete header.dataset.originalWidth;
      }

      // Restaurar posición del menú de navegación
      const selectPage = document.getElementById("select-page");
      if (selectPage instanceof HTMLElement) {
        selectPage.style.left = "";
        delete selectPage.dataset.originalLeft;
      }

      // Restaurar padding original de otros elementos fixed
      const fixedElements = document.querySelectorAll(".fixed");
      fixedElements.forEach((el) => {
        if (el instanceof HTMLElement) {
          const originalPadding = el.dataset.originalPadding;
          if (originalPadding) {
            el.style.paddingRight = `${originalPadding}px`;
            delete el.dataset.originalPadding;
          } else {
            el.style.paddingRight = "";
          }
        }
      });

      // Restaurar el scroll después de que el navegador haya procesado los cambios de estilo
      if (scrollY !== undefined) {
        requestAnimationFrame(() => {
          globalThis.scrollTo(0, Number.parseInt(scrollY));
          delete modal.dataset.scrollPosition;
        });
      }
    },
    { once: true }
  );
  form?.reset();
}

// Abrir el modal
export function showModalAnimation(modal: HTMLDialogElement, form: HTMLFormElement | null, background: boolean) {
  // Guardar la posición actual del scroll antes de bloquear el scroll
  const scrollY = globalThis.scrollY || globalThis.pageYOffset;
  modal.dataset.scrollPosition = scrollY.toString();

  // Calcular el ancho de la scrollbar para evitar el salto de contenido
  const scrollbarWidth = globalThis.innerWidth - document.documentElement.clientWidth;

  // Compensar el ancho de la scrollbar para evitar el salto de contenido
  if (scrollbarWidth > 0 && background) {
    // Guardar y sumar el padding original del body
    const bodyPaddingRight = Number.parseInt(globalThis.getComputedStyle(document.body).paddingRight) || 0;
    document.body.style.paddingRight = `${bodyPaddingRight + scrollbarWidth}px`;

    // Para el header: fijar su ancho ANTES de que el viewport cambie
    const header = document.querySelector("header");
    if (header instanceof HTMLElement) {
      // Guardamos el ancho actual del header (incluyendo la scrollbar)
      const currentWidth = header.offsetWidth;
      header.style.width = `${currentWidth}px`;
      header.dataset.originalWidth = currentWidth.toString();
    }

    // Aplicar compensación al menú de navegación para evitar desplazamiento
    const selectPage = document.getElementById("select-page");
    if (selectPage instanceof HTMLElement) {
      const currentLeft = selectPage.offsetLeft;
      selectPage.style.left = `${currentLeft - scrollbarWidth / 2}px`;
      selectPage.dataset.originalLeft = currentLeft.toString();
    }

    // Aplicar padding a otros elementos fixed
    const fixedElements = document.querySelectorAll(".fixed");
    fixedElements.forEach((el) => {
      if (el instanceof HTMLElement) {
        const currentPadding = Number.parseInt(globalThis.getComputedStyle(el).paddingRight) || 0;
        el.style.paddingRight = `${currentPadding + scrollbarWidth}px`;
        // Guardar el padding original como data attribute
        el.dataset.originalPadding = currentPadding.toString();
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

// Variable global para almacenar la referencia del intervalo
let calScrollInterval: ReturnType<typeof setInterval> | null = null;
export function startCalScrollManagement(): void {
  let savedScroll = 0;
  let wasOpen = false;

  if (calScrollInterval !== null) return;

  calScrollInterval = setInterval(() => {
    const modalBoxes = document.querySelectorAll("cal-modal-box");

    // Si no hay modales, no hacer nada
    if (modalBoxes.length === 0) return;

    // Determinar si HAY al menos un modal visible
    let isOpen = false;
    for (const box of modalBoxes) {
      const el = box as HTMLElement;
      const vis = el.style.visibility;
      // Visible si visibility NO es "hidden"
      if (vis !== "hidden") {
        isOpen = true;
        break;
      }
    }

    // ==========================================
    // TRANSICIÓN: CERRADO → ABIERTO
    // ==========================================
    if (isOpen && !wasOpen) {
      wasOpen = true;
      savedScroll = globalThis.scrollY || globalThis.pageYOffset;
      // Bloquear scroll
      document.body.style.overflow = "hidden";
      document.body.style.position = "fixed";
      document.body.style.top = `-${savedScroll}px`;
      document.body.style.width = "100%";
    }

    // ==========================================
    // TRANSICIÓN: ABIERTO → CERRADO
    // ==========================================
    else if (!isOpen && wasOpen) {
      wasOpen = false;
      document.body.style.overflow = "";
      document.body.style.position = "";
      document.body.style.top = "";
      document.body.style.width = "";
      // Cerramnos el intervalo de scroll
      globalThis.scrollTo(0, savedScroll);

      if (calScrollInterval !== null) {
        clearInterval(calScrollInterval);
        calScrollInterval = null;
      }
    }
  }, 200);
}

// Para ocultar el banner de cookies
export function hideBanner() {
  const banner = document.getElementById("cookie-banner");
  if (banner) {
    banner.style.animation = "slide-down 0.3s ease-out";
    setTimeout(() => {
      banner.classList.add("hidden");
    }, 300);
  }
}

// Para abrir el modal
export function openCommentModal(idCommentShared: string, isEdit: boolean = false) {
  const modal = document.getElementById(idCommentShared) as HTMLDialogElement;
  const form = modal?.querySelector("form") as HTMLFormElement;

  if (modal && form) showModalAnimation(modal, form, true); // true para showModal() en lugar de show()
}
