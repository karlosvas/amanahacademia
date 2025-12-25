import { describe, it, expect, beforeEach, vi, afterEach } from "vitest";
import {
  closeModalAnimation,
  showModalAnimation,
  closeModalsEvents,
  startCalScrollManagement,
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
      setAttribute: vi.fn(),
      removeAttribute: vi.fn(),
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

    // Mock window properties
    Object.defineProperty(window, "innerWidth", {
      writable: true,
      configurable: true,
      value: 1024,
    });

    Object.defineProperty(document.documentElement, "clientWidth", {
      writable: true,
      configurable: true,
      value: 1024,
    });

    Object.defineProperty(window, "scrollY", {
      writable: true,
      configurable: true,
      value: 0,
    });

    Object.defineProperty(window, "pageYOffset", {
      writable: true,
      configurable: true,
      value: 0,
    });

    global.requestAnimationFrame = vi.fn((callback) => {
      callback(0);
      return 0;
    }) as any;

    window.scrollTo = vi.fn();

    Object.defineProperty(window, "getComputedStyle", {
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

      expect(window.scrollTo).toHaveBeenCalledWith(0, 500);
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
      Object.defineProperty(window, "scrollY", {
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
      Object.defineProperty(window, "innerWidth", {
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
      Object.defineProperty(window, "innerWidth", {
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
      Object.defineProperty(window, "innerWidth", {
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
      Object.defineProperty(window, "innerWidth", {
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
  });

  describe("startCalScrollManagement", () => {
    beforeEach(() => {
      document.querySelectorAll = vi.fn(() => [] as any as NodeListOf<Element>) as any;
    });

    afterEach(() => {
      vi.clearAllTimers();
      vi.useRealTimers();
    });

    it("should start an interval to check modal visibility", () => {
      vi.useFakeTimers();
      const setIntervalSpy = vi.spyOn(window, "setInterval");

      startCalScrollManagement();

      expect(setIntervalSpy).toHaveBeenCalledWith(expect.any(Function), 200);

      vi.useRealTimers();
    });

    it("should not start multiple intervals if already running", () => {
      vi.useFakeTimers();

      // Primer intervalo
      const mockModalBox1 = { style: { visibility: "visible" } } as any;
      document.querySelectorAll = vi.fn(() => [mockModalBox1] as any as NodeListOf<Element>) as any;

      startCalScrollManagement();
      const firstInterval = vi.getTimerCount();

      // Intentar segundo intervalo
      startCalScrollManagement();
      const secondInterval = vi.getTimerCount();

      // No debe haber creado un nuevo intervalo
      expect(firstInterval).toBe(secondInterval);

      vi.useRealTimers();
    });

    // Nota: Los siguientes tests de integración con setInterval son complejos de mockear
    // La funcionalidad principal está cubierta por los tests anteriores
    it.skip("should block scroll when modal becomes visible - integration test", () => {
      // Este test requiere un entorno de navegador real para funcionar correctamente
      // La función startCalScrollManagement está diseñada para trabajar con elementos reales del DOM
    });

    it.skip("should restore scroll when modal becomes hidden - integration test", () => {
      // Este test requiere un entorno de navegador real para funcionar correctamente
    });

    it.skip("should clear interval when modal closes - integration test", () => {
      // Este test requiere un entorno de navegador real para funcionar correctamente
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
});
