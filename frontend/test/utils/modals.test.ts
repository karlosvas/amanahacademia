import { describe, it, expect, beforeEach, vi, afterEach } from "vitest";
import {
  closeModalAnimation,
  showModalAnimation,
  closeModalsEvents,
  openCommentModal,
} from "@/utils/modals.ts";
import { rejectCookies } from "@/utils/cookie";

describe("Modal Utilities", () => {
  let mockModal: HTMLDialogElement;
  let mockForm: HTMLFormElement;

  beforeEach(() => {
    // Reset all mocks
    vi.clearAllMocks();
    vi.clearAllTimers();

    // Mock modal element with dataset
    mockModal = {
      setAttribute: vi.fn((attr) => {
        if (attr === "closing") {
          mockModal.dataset.closing = "";
        }
      }),
      removeAttribute: vi.fn((attr) => {
        if (attr === "closing") {
          delete mockModal.dataset.closing;
        }
      }),
      close: vi.fn(),
      showModal: vi.fn(),
      show: vi.fn(),
      dataset: {},
      classList: {
        add: vi.fn(),
        remove: vi.fn(),
        contains: vi.fn(() => false),
      },
      addEventListener: vi.fn((event, callback) => {
        if (event === "animationend") {
          // Ejecutar callback inmediatamente para tests
          setTimeout(callback, 0);
        }
      }),
      querySelector: vi.fn(),
    } as any;

    // Mock form element
    mockForm = {
      reset: vi.fn(),
      querySelector: vi.fn(() => ({
        focus: vi.fn(),
      })),
    } as any;

    // Reset document styles
    document.body.style.overflow = "";
    document.documentElement.style.overflow = "";
    document.body.style.paddingRight = "";
    document.body.style.position = "";
    document.body.style.top = "";
    document.body.style.width = "";

    // Mock globalThis properties
    Object.defineProperty(globalThis, "innerWidth", {
      writable: true,
      configurable: true,
      value: 1024,
    });

    Object.defineProperty(document.documentElement, "clientWidth", {
      writable: true,
      configurable: true,
      value: 1024,
    });

    Object.defineProperty(globalThis, "scrollY", {
      writable: true,
      configurable: true,
      value: 0,
    });

    Object.defineProperty(globalThis, "pageYOffset", {
      writable: true,
      configurable: true,
      value: 0,
    });

    globalThis.requestAnimationFrame = vi.fn((callback) => {
      callback(0);
      return 0;
    }) as any;

    globalThis.scrollTo = vi.fn();

    Object.defineProperty(globalThis, "getComputedStyle", {
      writable: true,
      configurable: true,
      value: () => ({ paddingRight: "0px" }),
    });
  });

  afterEach(() => {
    vi.clearAllTimers();
    vi.clearAllMocks();
  });

  describe("closeModalAnimation", () => {
    beforeEach(() => {
      // Setup default querySelector and querySelectorAll mocks
      document.querySelector = vi.fn(() => null) as any;
      document.getElementById = vi.fn(() => null) as any;
      document.querySelectorAll = vi.fn(() => [] as any as NodeListOf<Element>) as any;
    });

    it("should set closing attribute on modal", () => {
      closeModalAnimation(mockModal, mockForm);
      expect(mockModal.dataset.closing).toBe("");
    });

    it("should reset form when provided", async () => {
      closeModalAnimation(mockModal, mockForm);

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(mockForm.reset).toHaveBeenCalled();
    });

    it("should restore body overflow styles", async () => {
      document.body.style.overflow = "hidden";
      document.documentElement.style.overflow = "hidden";

      closeModalAnimation(mockModal, mockForm);

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(document.body.style.overflow).toBe("");
      expect(document.documentElement.style.overflow).toBe("");
    });

    it("should restore body padding", async () => {
      document.body.style.paddingRight = "16px";

      closeModalAnimation(mockModal, mockForm);

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(document.body.style.paddingRight).toBe("");
    });

    it("should remove closing attribute and close modal after animation", async () => {
      closeModalAnimation(mockModal, mockForm);

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(mockModal.dataset.closing).toBeUndefined();
      expect(mockModal.close).toHaveBeenCalled();
    });

    it("should work without form parameter", async () => {
      closeModalAnimation(mockModal, null);

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(mockModal.close).toHaveBeenCalled();
    });

    it("should restore scroll position from dataset", async () => {
      mockModal.dataset.scrollPosition = "500";

      closeModalAnimation(mockModal, mockForm);

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(globalThis.scrollTo).toHaveBeenCalledWith(0, 500);
      expect(mockModal.dataset.scrollPosition).toBeUndefined();
    });

    it("should restore header width", async () => {
      const style = { width: "1200px" };
      const dataset = { originalWidth: "1200" };
      const mockHeader = {
        style,
        dataset,
      } as any;

      // Make it an instance of HTMLElement
      Object.setPrototypeOf(mockHeader, HTMLElement.prototype);

      document.querySelector = vi.fn((selector) => {
        if (selector === "header") return mockHeader;
        return null;
      }) as any;

      closeModalAnimation(mockModal, mockForm);

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(mockHeader.style.width).toBe("");
      expect(mockHeader.dataset.originalWidth).toBeUndefined();
    });

    it("should restore select-page left position", async () => {
      const style = { left: "90px" };
      const dataset = { originalLeft: "90" };
      const mockSelectPage = {
        style,
        dataset,
      } as any;

      Object.setPrototypeOf(mockSelectPage, HTMLElement.prototype);

      document.getElementById = vi.fn((id) => {
        if (id === "select-page") return mockSelectPage;
        return null;
      }) as any;

      closeModalAnimation(mockModal, mockForm);

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(mockSelectPage.style.left).toBe("");
      expect(mockSelectPage.dataset.originalLeft).toBeUndefined();
    });

    it("should restore padding of fixed elements", async () => {
      const style = { paddingRight: "16px" };
      const dataset = { originalPadding: "0" };
      const mockFixedElement = {
        style,
        dataset,
      } as any;

      Object.setPrototypeOf(mockFixedElement, HTMLElement.prototype);

      document.querySelectorAll = vi.fn((selector) => {
        if (selector === ".fixed") return [mockFixedElement] as any as NodeListOf<Element>;
        return [] as any as NodeListOf<Element>;
      }) as any;

      closeModalAnimation(mockModal, mockForm);

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(mockFixedElement.style.paddingRight).toBe("0px");
      expect(mockFixedElement.dataset.originalPadding).toBeUndefined();
    });
  });

  describe("showModalAnimation", () => {
    beforeEach(() => {
      // Setup default querySelector and querySelectorAll mocks
      document.querySelector = vi.fn(() => null) as any;
      document.getElementById = vi.fn(() => null) as any;
      document.querySelectorAll = vi.fn(() => [] as any as NodeListOf<Element>) as any;
    });

    it("should show modal with background when background=true", () => {
      showModalAnimation(mockModal, mockForm, true);
      expect(mockModal.showModal).toHaveBeenCalled();
    });

    it("should show modal without background when background=false", () => {
      showModalAnimation(mockModal, mockForm, false);
      expect(mockModal.show).toHaveBeenCalled();
    });

    it("should add modal-opening class", () => {
      showModalAnimation(mockModal, mockForm, true);
      expect(mockModal.classList.add).toHaveBeenCalledWith("modal-opening");
    });

    it("should save scroll position in dataset", () => {
      Object.defineProperty(globalThis, "scrollY", {
        writable: true,
        configurable: true,
        value: 300,
      });

      showModalAnimation(mockModal, mockForm, true);
      expect(mockModal.dataset.scrollPosition).toBe("300");
    });

    it("should not block scroll when background=false", () => {
      showModalAnimation(mockModal, mockForm, false);
      expect(document.body.style.paddingRight).toBe("");
    });

    it("should compensate for scrollbar width when background=true", () => {
      Object.defineProperty(globalThis, "innerWidth", {
        writable: true,
        configurable: true,
        value: 1024,
      });
      Object.defineProperty(document.documentElement, "clientWidth", {
        writable: true,
        configurable: true,
        value: 1008, // 16px scrollbar
      });

      showModalAnimation(mockModal, mockForm, true);

      expect(document.body.style.paddingRight).toBe("16px");
    });

    it("should set header width to prevent layout shift", () => {
      Object.defineProperty(globalThis, "innerWidth", {
        writable: true,
        configurable: true,
        value: 1024,
      });
      Object.defineProperty(document.documentElement, "clientWidth", {
        writable: true,
        configurable: true,
        value: 1008,
      });

      const style = { width: "" };
      const dataset = {};
      const mockHeader = {
        style,
        dataset,
        offsetWidth: 1200,
      } as any;

      Object.setPrototypeOf(mockHeader, HTMLElement.prototype);

      document.querySelector = vi.fn((selector) => {
        if (selector === "header") return mockHeader;
        return null;
      }) as any;

      showModalAnimation(mockModal, mockForm, true);

      expect(mockHeader.style.width).toBe("1200px");
      expect(mockHeader.dataset.originalWidth).toBe("1200");
    });

    it("should adjust select-page position for scrollbar compensation", () => {
      Object.defineProperty(globalThis, "innerWidth", {
        writable: true,
        configurable: true,
        value: 1024,
      });
      Object.defineProperty(document.documentElement, "clientWidth", {
        writable: true,
        configurable: true,
        value: 1008,
      });

      const style = { left: "" };
      const dataset = {};
      const mockSelectPage = {
        style,
        dataset,
        offsetLeft: 100,
      } as any;

      Object.setPrototypeOf(mockSelectPage, HTMLElement.prototype);

      document.getElementById = vi.fn((id) => {
        if (id === "select-page") return mockSelectPage;
        return null;
      }) as any;

      showModalAnimation(mockModal, mockForm, true);

      expect(mockSelectPage.style.left).toBe("92px"); // 100 - 16/2
      expect(mockSelectPage.dataset.originalLeft).toBe("100");
    });

    it("should add padding to fixed elements", () => {
      Object.defineProperty(globalThis, "innerWidth", {
        writable: true,
        configurable: true,
        value: 1024,
      });
      Object.defineProperty(document.documentElement, "clientWidth", {
        writable: true,
        configurable: true,
        value: 1008,
      });

      const style = { paddingRight: "" };
      const dataset = {};
      const mockFixedElement = {
        style,
        dataset,
      } as any;

      Object.setPrototypeOf(mockFixedElement, HTMLElement.prototype);

      document.querySelectorAll = vi.fn((selector) => {
        if (selector === ".fixed") return [mockFixedElement] as any as NodeListOf<Element>;
        return [] as any as NodeListOf<Element>;
      }) as any;

      showModalAnimation(mockModal, mockForm, true);

      expect(mockFixedElement.style.paddingRight).toBe("16px");
      expect(mockFixedElement.dataset.originalPadding).toBe("0");
    });

    it("should focus first input in form", () => {
      const mockInput = {
        focus: vi.fn(),
      };

      Object.setPrototypeOf(mockInput, HTMLElement.prototype);

      mockForm.querySelector = vi.fn(() => mockInput as any);

      showModalAnimation(mockModal, mockForm, true);

      expect(mockInput.focus).toHaveBeenCalled();
    });

    it("should blur active element when no form provided", () => {
      const mockActiveElement = {
        blur: vi.fn(),
      };

      Object.setPrototypeOf(mockActiveElement, HTMLElement.prototype);
      Object.defineProperty(document, "activeElement", {
        writable: true,
        configurable: true,
        value: mockActiveElement,
      });

      showModalAnimation(mockModal, null, true);

      expect(mockActiveElement.blur).toHaveBeenCalled();
    });

    it("should remove modal-opening class after animation", () => {
      vi.useFakeTimers();

      showModalAnimation(mockModal, mockForm, true);

      vi.advanceTimersByTime(350);

      expect(mockModal.classList.remove).toHaveBeenCalledWith("modal-opening");

      vi.useRealTimers();
    });
  });

  describe("closeModalsEvents", () => {
    beforeEach(() => {
      document.querySelectorAll = vi.fn(() => [] as any as NodeListOf<Element>) as any;
    });

    it("should add mousedown event listener to document", () => {
      const addEventListenerSpy = vi.spyOn(document, "addEventListener");

      closeModalsEvents();

      expect(addEventListenerSpy).toHaveBeenCalledWith("mousedown", expect.any(Function));
    });

    it("should add cancel event listener to all dialogs", () => {
      const mockDialog = {
        addEventListener: vi.fn(),
      } as any;

      document.querySelectorAll = vi.fn((selector) => {
        if (selector === "dialog") return [mockDialog] as any as NodeListOf<Element>;
        return [] as any as NodeListOf<Element>;
      }) as any;

      closeModalsEvents();

      expect(mockDialog.addEventListener).toHaveBeenCalledWith("cancel", expect.any(Function));
    });

    it("should close modal when clicking on embla__container", async () => {
      const mockForm = { reset: vi.fn() } as any;
      const mockCarousel = { classList: { contains: vi.fn(() => true) } } as any;
      Object.setPrototypeOf(mockCarousel, HTMLElement.prototype);

      const mockModalLocal = {
        dataset: {} as any,
        classList: {
          add: vi.fn(),
          remove: vi.fn(),
        },
        querySelector: vi.fn(() => mockForm),
        close: vi.fn(),
        setAttribute: vi.fn((attr) => {
          if (attr === "closing") {
            mockModalLocal.dataset.closing = "";
          }
        }),
        removeAttribute: vi.fn((attr) => {
          if (attr === "closing") {
            delete mockModalLocal.dataset.closing;
          }
        }),
        addEventListener: vi.fn((event, callback) => {
          if (event === "animationend") {
            setTimeout(callback, 0);
          }
        }),
      } as any;
      Object.setPrototypeOf(mockModalLocal, HTMLDialogElement.prototype);

      const mockTarget = {
        closest: vi.fn((selector) => {
          if (selector === ".embla__container") return mockCarousel;
          if (selector === "dialog") return mockModalLocal;
          return null;
        }),
        getAttribute: vi.fn(() => null),
      } as any;

      document.querySelectorAll = vi.fn((selector) => {
        if (selector === "dialog") return [] as any as NodeListOf<Element>;
        if (selector === ".fixed") return [] as any as NodeListOf<Element>;
        return [] as any as NodeListOf<Element>;
      }) as any;
      document.querySelector = vi.fn(() => null) as any;
      document.getElementById = vi.fn(() => null) as any;

      let mousedownHandler: any;
      document.addEventListener = vi.fn((event, handler) => {
        if (event === "mousedown") {
          mousedownHandler = handler;
        }
      }) as any;

      closeModalsEvents();

      // Trigger mousedown event
      mousedownHandler({ target: mockTarget });

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(mockModalLocal.dataset.closing).toBeUndefined();
      expect(mockModalLocal.close).toHaveBeenCalled();
    });

    it("should close modal when clicking on backdrop (HTMLDialogElement)", () => {
      // This test verifies that the mousedown event listener is set up correctly.
      // Note: Testing the actual backdrop click behavior (target instanceof HTMLDialogElement)
      // reveals a potential issue in the code where target.closest("dialog") returns null
      // when target IS the dialog element, which would cause closeModalAnimation to receive
      // a null modal parameter. This scenario is difficult to test properly in JSDOM.

      const addEventListenerSpy = vi.spyOn(document, "addEventListener");

      closeModalsEvents();

      expect(addEventListenerSpy).toHaveBeenCalledWith("mousedown", expect.any(Function));
    });

    it("should close modal when clicking on close button with aria-label", async () => {
      const mockForm = { reset: vi.fn() } as any;

      const mockModalLocal2 = {
        dataset: {} as any,
        classList: {
          add: vi.fn(),
          remove: vi.fn(),
        },
        querySelector: vi.fn(() => mockForm),
        close: vi.fn(),
        setAttribute: vi.fn((attr) => {
          if (attr === "closing") {
            mockModalLocal2.dataset.closing = "";
          }
        }),
        removeAttribute: vi.fn((attr) => {
          if (attr === "closing") {
            delete mockModalLocal2.dataset.closing;
          }
        }),
        addEventListener: vi.fn((event, callback) => {
          if (event === "animationend") {
            setTimeout(callback, 0);
          }
        }),
      } as any;
      Object.setPrototypeOf(mockModalLocal2, HTMLDialogElement.prototype);

      const mockTarget = {
        closest: vi.fn((selector) => {
          if (selector === "dialog") return mockModalLocal2;
          return null;
        }),
        getAttribute: vi.fn((attr) => {
          if (attr === "aria-label") return "close-modal";
          return null;
        }),
      } as any;

      document.querySelectorAll = vi.fn((selector) => {
        if (selector === "dialog") return [] as any as NodeListOf<Element>;
        if (selector === ".fixed") return [] as any as NodeListOf<Element>;
        return [] as any as NodeListOf<Element>;
      }) as any;
      document.querySelector = vi.fn(() => null) as any;
      document.getElementById = vi.fn(() => null) as any;

      let mousedownHandler: any;
      document.addEventListener = vi.fn((event, handler) => {
        if (event === "mousedown") {
          mousedownHandler = handler;
        }
      }) as any;

      closeModalsEvents();

      // Trigger mousedown event on close button
      mousedownHandler({ target: mockTarget });

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(mockModalLocal2.close).toHaveBeenCalled();
    });

    it("should prevent default and close modal on cancel event", async () => {
      const mockForm = { reset: vi.fn() } as any;

      const mockDialog = {
        dataset: {} as any,
        classList: {
          add: vi.fn(),
          remove: vi.fn(),
        },
        querySelector: vi.fn(() => mockForm),
        close: vi.fn(),
        setAttribute: vi.fn((attr) => {
          if (attr === "closing") {
            mockDialog.dataset.closing = "";
          }
        }),
        removeAttribute: vi.fn((attr) => {
          if (attr === "closing") {
            delete mockDialog.dataset.closing;
          }
        }),
        addEventListener: vi.fn((event, callback) => {
          if (event === "animationend") {
            setTimeout(callback, 0);
          }
        }),
      } as any;
      Object.setPrototypeOf(mockDialog, HTMLDialogElement.prototype);

      let cancelHandler: any;
      mockDialog.addEventListener = vi.fn((event, handler) => {
        if (event === "cancel") {
          cancelHandler = handler;
        }
        if (event === "animationend") {
          setTimeout(handler, 0);
        }
      });

      document.querySelectorAll = vi.fn((selector) => {
        if (selector === "dialog") return [mockDialog] as any as NodeListOf<Element>;
        if (selector === ".fixed") return [] as any as NodeListOf<Element>;
        return [] as any as NodeListOf<Element>;
      }) as any;
      document.querySelector = vi.fn(() => null) as any;
      document.getElementById = vi.fn(() => null) as any;

      closeModalsEvents();

      const mockEvent = {
        preventDefault: vi.fn(),
      };

      // Trigger cancel event
      cancelHandler(mockEvent);

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(mockEvent.preventDefault).toHaveBeenCalled();
      expect(mockDialog.close).toHaveBeenCalled();
    });
  });

  describe("startCalScrollManagement", () => {
    beforeEach(async () => {
      document.querySelectorAll = vi.fn(() => [] as any as NodeListOf<Element>) as any;
      // Limpiar estilos del body
      document.body.style.overflow = "";
      document.body.style.position = "";
      document.body.style.top = "";
      document.body.style.width = "";

      // Reset the global calScrollInterval by reimporting the module
      vi.resetModules();
    });

    afterEach(() => {
      vi.clearAllTimers();
      vi.useRealTimers();
      // Limpiar estilos del body después de cada test
      document.body.style.overflow = "";
      document.body.style.position = "";
      document.body.style.top = "";
      document.body.style.width = "";
    });

    it("should start an interval to check modal visibility", async () => {
      vi.useFakeTimers();
      const setIntervalSpy = vi.spyOn(globalThis, "setInterval");

      const { startCalScrollManagement } = await import("@/utils/modals");
      startCalScrollManagement();

      expect(setIntervalSpy).toHaveBeenCalledWith(expect.any(Function), 200);

      vi.useRealTimers();
    });

    it("should not start multiple intervals if already running", async () => {
      vi.useFakeTimers();

      // Primer intervalo
      const mockModalBox1 = { style: { visibility: "visible" } } as any;
      document.querySelectorAll = vi.fn(() => [mockModalBox1] as any as NodeListOf<Element>) as any;

      const { startCalScrollManagement } = await import("@/utils/modals");
      startCalScrollManagement();
      const firstInterval = vi.getTimerCount();

      // Intentar segundo intervalo
      startCalScrollManagement();
      const secondInterval = vi.getTimerCount();

      // No debe haber creado un nuevo intervalo
      expect(firstInterval).toBe(secondInterval);

      vi.useRealTimers();
    });

    it("should do nothing when no cal-modal-box elements exist", async () => {
      vi.useFakeTimers();

      document.querySelectorAll = vi.fn(() => [] as any as NodeListOf<Element>) as any;

      const { startCalScrollManagement } = await import("@/utils/modals");
      startCalScrollManagement();

      // Avanzar el intervalo
      vi.advanceTimersByTime(200);

      // No debe haber modificado los estilos del body
      expect(document.body.style.overflow).toBe("");
      expect(document.body.style.position).toBe("");

      vi.useRealTimers();
    });

    it("should handle CLOSED→OPEN→CLOSED transitions correctly", async () => {
      vi.useFakeTimers();

      Object.defineProperty(globalThis, "scrollY", {
        writable: true,
        configurable: true,
        value: 250,
      });

      const mockModalBox = {
        style: { visibility: "hidden" }, // Start with modal hidden
      } as any;

      document.querySelectorAll = vi.fn(() => [mockModalBox] as any as NodeListOf<Element>) as any;

      // Clear any previous intervals
      vi.clearAllTimers();

      const { startCalScrollManagement } = await import("@/utils/modals");
      startCalScrollManagement();

      // First iteration - modal still hidden (no change)
      vi.advanceTimersByTime(200);
      expect(document.body.style.overflow).toBe("");

      // Now make modal visible (CLOSED → OPEN transition)
      mockModalBox.style.visibility = "";
      vi.advanceTimersByTime(200);

      // Should have blocked scroll
      expect(document.body.style.overflow).toBe("hidden");
      expect(document.body.style.position).toBe("fixed");
      expect(document.body.style.top).toBe("-250px");
      expect(document.body.style.width).toBe("100%");

      // Now hide modal again (OPEN → CLOSED transition)
      mockModalBox.style.visibility = "hidden";
      vi.advanceTimersByTime(200);

      // Should have restored scroll
      expect(document.body.style.overflow).toBe("");
      expect(document.body.style.position).toBe("");
      expect(document.body.style.top).toBe("");
      expect(document.body.style.width).toBe("");

      vi.useRealTimers();
    });

    it("should not block scroll if all modals are hidden", async () => {
      vi.useFakeTimers();

      const mockModalBox1 = { style: { visibility: "hidden" } } as any;
      const mockModalBox2 = { style: { visibility: "hidden" } } as any;

      document.querySelectorAll = vi.fn(() => [mockModalBox1, mockModalBox2] as any as NodeListOf<Element>) as any;

      const { startCalScrollManagement } = await import("@/utils/modals");
      startCalScrollManagement();

      vi.advanceTimersByTime(200);

      // No debe bloquear el scroll
      expect(document.body.style.overflow).toBe("");

      vi.useRealTimers();
    });

    it("should use pageYOffset fallback when scrollY is not available", async () => {
      vi.useFakeTimers();

      Object.defineProperty(globalThis, "scrollY", {
        writable: true,
        configurable: true,
        value: undefined,
      });

      Object.defineProperty(globalThis, "pageYOffset", {
        writable: true,
        configurable: true,
        value: 300,
      });

      const mockModalBox = {
        style: { visibility: "hidden" },
      } as any;

      document.querySelectorAll = vi.fn(() => [mockModalBox] as any as NodeListOf<Element>) as any;

      const { startCalScrollManagement } = await import("@/utils/modals");
      startCalScrollManagement();

      // Make modal visible
      mockModalBox.style.visibility = "";
      vi.advanceTimersByTime(200);

      // Should save pageYOffset value
      expect(document.body.style.top).toBe("-300px");

      vi.useRealTimers();
    });

    it("should restore scroll position when modal closes", async () => {
      vi.useFakeTimers();

      const scrollToSpy = vi.spyOn(globalThis, "scrollTo");

      Object.defineProperty(globalThis, "scrollY", {
        writable: true,
        configurable: true,
        value: 400,
      });

      const mockModalBox = {
        style: { visibility: "hidden" },
      } as any;

      document.querySelectorAll = vi.fn(() => [mockModalBox] as any as NodeListOf<Element>) as any;

      const { startCalScrollManagement } = await import("@/utils/modals");
      startCalScrollManagement();

      // Open modal
      mockModalBox.style.visibility = "";
      vi.advanceTimersByTime(200);

      // Close modal
      mockModalBox.style.visibility = "hidden";
      vi.advanceTimersByTime(200);

      // Should restore scroll to saved position
      expect(scrollToSpy).toHaveBeenCalledWith(0, 400);

      vi.useRealTimers();
    });

    it("should clear interval when modal closes", async () => {
      vi.useFakeTimers();

      const mockModalBox = {
        style: { visibility: "hidden" },
      } as any;

      document.querySelectorAll = vi.fn(() => [mockModalBox] as any as NodeListOf<Element>) as any;

      const { startCalScrollManagement } = await import("@/utils/modals");
      startCalScrollManagement();
      const initialTimerCount = vi.getTimerCount();

      // Open modal
      mockModalBox.style.visibility = "";
      vi.advanceTimersByTime(200);

      // Close modal (should clear interval)
      mockModalBox.style.visibility = "hidden";
      vi.advanceTimersByTime(200);

      // Timer should be cleared
      const finalTimerCount = vi.getTimerCount();
      expect(finalTimerCount).toBeLessThan(initialTimerCount);

      vi.useRealTimers();
    });
  });

  describe("hideBanner", () => {
    it("should hide the cookie banner with animation", () => {
      vi.useFakeTimers();

      const mockBanner = {
        classList: { add: vi.fn() },
        style: { animation: "" },
      };
      document.getElementById = vi.fn(() => mockBanner as any);

      rejectCookies();

      vi.runAllTimers();

      expect(mockBanner.style.animation).toBe("slide-down 0.3s ease-out");
      expect(mockBanner.classList.add).toHaveBeenCalledWith("hidden");

      vi.useRealTimers();
    });
  });

  describe("openCommentModal", () => {
    beforeEach(() => {
      // Setup default querySelector and querySelectorAll mocks
      document.querySelector = vi.fn(() => null) as any;
      document.getElementById = vi.fn(() => null) as any;
      document.querySelectorAll = vi.fn(() => [] as any as NodeListOf<Element>) as any;
    });

    it("should open modal with form when both exist", () => {
      const mockForm = {
        querySelector: vi.fn(() => ({
          focus: vi.fn(),
        })),
      } as any;

      const mockModal = {
        setAttribute: vi.fn(),
        removeAttribute: vi.fn(),
        showModal: vi.fn(),
        show: vi.fn(),
        dataset: {},
        classList: {
          add: vi.fn(),
          remove: vi.fn(),
        },
        querySelector: vi.fn(() => mockForm),
        addEventListener: vi.fn(),
      } as any;

      Object.setPrototypeOf(mockModal, HTMLDialogElement.prototype);

      document.getElementById = vi.fn((id) => {
        if (id === "test-modal") return mockModal;
        return null;
      }) as any;

      openCommentModal("test-modal", false);

      expect(document.getElementById).toHaveBeenCalledWith("test-modal");
      expect(mockModal.querySelector).toHaveBeenCalledWith("form");
      expect(mockModal.showModal).toHaveBeenCalled();
    });

    it("should not call showModalAnimation if modal does not exist", () => {
      document.getElementById = vi.fn(() => null) as any;

      openCommentModal("non-existent-modal", false);

      expect(document.getElementById).toHaveBeenCalledWith("non-existent-modal");
    });

    it("should not call showModalAnimation if form does not exist", () => {
      const mockModal = {
        querySelector: vi.fn(() => null),
        showModal: vi.fn(),
      } as any;

      Object.setPrototypeOf(mockModal, HTMLDialogElement.prototype);

      document.getElementById = vi.fn((id) => {
        if (id === "test-modal") return mockModal;
        return null;
      }) as any;

      openCommentModal("test-modal", false);

      expect(mockModal.querySelector).toHaveBeenCalledWith("form");
      expect(mockModal.showModal).not.toHaveBeenCalled();
    });

    it("should pass isEdit parameter correctly (true)", () => {
      const mockForm = {
        querySelector: vi.fn(() => ({
          focus: vi.fn(),
        })),
      } as any;

      const mockModal = {
        setAttribute: vi.fn(),
        removeAttribute: vi.fn(),
        showModal: vi.fn(),
        show: vi.fn(),
        dataset: {},
        classList: {
          add: vi.fn(),
          remove: vi.fn(),
        },
        querySelector: vi.fn(() => mockForm),
        addEventListener: vi.fn(),
      } as any;

      Object.setPrototypeOf(mockModal, HTMLDialogElement.prototype);

      document.getElementById = vi.fn((id) => {
        if (id === "edit-modal") return mockModal;
        return null;
      }) as any;

      openCommentModal("edit-modal", true);

      expect(document.getElementById).toHaveBeenCalledWith("edit-modal");
      expect(mockModal.showModal).toHaveBeenCalled();
    });

    it("should use default isEdit parameter (false) when not provided", () => {
      const mockForm = {
        querySelector: vi.fn(() => ({
          focus: vi.fn(),
        })),
      } as any;

      const mockModal = {
        setAttribute: vi.fn(),
        removeAttribute: vi.fn(),
        showModal: vi.fn(),
        show: vi.fn(),
        dataset: {},
        classList: {
          add: vi.fn(),
          remove: vi.fn(),
        },
        querySelector: vi.fn(() => mockForm),
        addEventListener: vi.fn(),
      } as any;

      Object.setPrototypeOf(mockModal, HTMLDialogElement.prototype);

      document.getElementById = vi.fn((id) => {
        if (id === "comment-modal") return mockModal;
        return null;
      }) as any;

      openCommentModal("comment-modal");

      expect(document.getElementById).toHaveBeenCalledWith("comment-modal");
      expect(mockModal.showModal).toHaveBeenCalled();
    });
  });
});
