import { describe, it, expect, beforeEach, vi } from 'vitest';
import { closeModalAnimation, showModalAnimation } from '@/utils/modals';

describe('Modal Utilities', () => {
  let mockModal: HTMLDialogElement;
  let mockForm: HTMLFormElement;

  beforeEach(() => {
    // Mock modal element
    mockModal = {
      setAttribute: vi.fn(),
      removeAttribute: vi.fn(),
      close: vi.fn(),
      showModal: vi.fn(),
      show: vi.fn(),
      classList: {
        add: vi.fn(),
        remove: vi.fn(),
        contains: vi.fn(() => false),
      },
      addEventListener: vi.fn((event, callback) => {
        if (event === 'animationend') {
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

    // Mock document methods
    document.body.style.overflow = '';
    document.documentElement.style.overflow = '';
    document.body.style.paddingRight = '';

    document.querySelector = vi.fn((selector) => {
      if (selector === 'header') {
        return {
          style: { width: '' },
          setAttribute: vi.fn(),
          removeAttribute: vi.fn(),
          offsetWidth: 1200,
        } as any;
      }
      return null;
    });

    document.querySelectorAll = vi.fn(() => []);
    document.getElementById = vi.fn(() => null);

    // Mock window
    Object.defineProperty(window, 'innerWidth', {
      writable: true,
      value: 1024,
    });

    Object.defineProperty(document.documentElement, 'clientWidth', {
      writable: true,
      value: 1024,
    });

    Object.defineProperty(window, 'getComputedStyle', {
      writable: true,
      value: () => ({ paddingRight: '0px' }),
    });
  });

  describe('closeModalAnimation', () => {
    it('should set closing attribute on modal', () => {
      closeModalAnimation(mockModal, mockForm);
      expect(mockModal.setAttribute).toHaveBeenCalledWith('closing', '');
    });

    it('should reset form when provided', async () => {
      closeModalAnimation(mockModal, mockForm);

      // Wait for animation end
      await new Promise(resolve => setTimeout(resolve, 10));

      expect(mockForm.reset).toHaveBeenCalled();
    });

    it('should restore body overflow styles', async () => {
      document.body.style.overflow = 'hidden';

      closeModalAnimation(mockModal, mockForm);

      await new Promise(resolve => setTimeout(resolve, 10));

      expect(document.body.style.overflow).toBe('');
    });

    it('should remove closing attribute and close modal after animation', async () => {
      closeModalAnimation(mockModal, mockForm);

      await new Promise(resolve => setTimeout(resolve, 10));

      expect(mockModal.removeAttribute).toHaveBeenCalledWith('closing');
      expect(mockModal.close).toHaveBeenCalled();
    });

    it('should work without form parameter', async () => {
      closeModalAnimation(mockModal, null);

      await new Promise(resolve => setTimeout(resolve, 10));

      expect(mockModal.close).toHaveBeenCalled();
    });
  });

  describe('showModalAnimation', () => {
    it('should show modal with background when background=true', () => {
      showModalAnimation(mockModal, mockForm, true);
      expect(mockModal.showModal).toHaveBeenCalled();
    });

    it('should show modal without background when background=false', () => {
      showModalAnimation(mockModal, mockForm, false);
      expect(mockModal.show).toHaveBeenCalled();
    });

    it('should add modal-opening class', () => {
      showModalAnimation(mockModal, mockForm, true);
      expect(mockModal.classList.add).toHaveBeenCalledWith('modal-opening');
    });

    it('should block scroll when background=true', () => {
      showModalAnimation(mockModal, mockForm, true);
      expect(document.body.style.overflow).toBe('hidden');
      expect(document.documentElement.style.overflow).toBe('hidden');
    });

    it('should not block scroll when background=false', () => {
      showModalAnimation(mockModal, mockForm, false);
      expect(document.body.style.overflow).toBe('');
    });

    it('should compensate for scrollbar width', () => {
      // Simulate scrollbar width
      Object.defineProperty(window, 'innerWidth', {
        writable: true,
        value: 1024,
      });
      Object.defineProperty(document.documentElement, 'clientWidth', {
        writable: true,
        value: 1008, // 16px scrollbar
      });

      showModalAnimation(mockModal, mockForm, true);

      expect(document.body.style.paddingRight).toBe('16px');
    });

    it('should focus first input in form', () => {
      const mockInput = {
        focus: vi.fn(),
      };

      // Make it appear as HTMLElement
      Object.setPrototypeOf(mockInput, HTMLElement.prototype);

      mockForm.querySelector = vi.fn(() => mockInput as any);

      showModalAnimation(mockModal, mockForm, true);

      expect(mockInput.focus).toHaveBeenCalled();
    });

    it('should remove modal-opening class after animation', () => {
      vi.useFakeTimers();

      showModalAnimation(mockModal, mockForm, true);

      vi.advanceTimersByTime(350);

      expect(mockModal.classList.remove).toHaveBeenCalledWith('modal-opening');

      vi.useRealTimers();
    });
  });
});
