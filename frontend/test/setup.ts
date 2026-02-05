import { vi } from "vitest";

// Mock de window.matchMedia
Object.defineProperty(window, "matchMedia", {
  writable: true,
  value: vi.fn().mockImplementation((query) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

// Mock de import.meta.env
vi.mock("import.meta", () => ({
  env: {
    PUBLIC_FIREBASE_API_KEY: "test-api-key",
    PUBLIC_FIREBASE_AUTH_DOMAIN: "test.firebaseapp.com",
    PUBLIC_FIREBASE_PROJECT_ID: "test-project",
    PUBLIC_FIREBASE_STORAGE_BUCKET: "test.appspot.com",
    PUBLIC_FIREBASE_MESSAGING_SENDER_ID: "123456789",
    PUBLIC_FIREBASE_APP_ID: "test-app-id",
    PUBLIC_FIREBASE_MEASUREMENT_ID: "G-TEST",
    PUBLIC_BACKEND_URL: "http://localhost:3000",
    PUBLIC_STRIPE_PUBLIC_KEY: "pk_test_123",
  },
}));

// Mock de Firebase Auth
vi.mock("firebase/app", () => ({
  initializeApp: vi.fn(() => ({})),
}));

vi.mock("firebase/auth", () => ({
  getAuth: vi.fn(() => ({
    currentUser: null,
    signOut: vi.fn(),
  })),
  signInWithEmailAndPassword: vi.fn(),
  signInWithPopup: vi.fn(),
  GoogleAuthProvider: vi.fn(),
  onAuthStateChanged: vi.fn((auth, callback) => {
    callback(null);
    return vi.fn();
  }),
}));

// Mock de solid-toast
vi.mock("solid-toast", () => ({
  default: {
    success: vi.fn(),
    error: vi.fn(),
    loading: vi.fn(),
  },
}));

// Mock de JustValidate
global.window = global.window || {};
(global.window as any).JustValidate = vi.fn().mockImplementation(() => ({
  addField: vi.fn().mockReturnThis(),
  onSuccess: vi.fn().mockReturnThis(),
}));

// Mock de fetch global
global.fetch = vi.fn();

// Mock de location
Object.defineProperty(window, "location", {
  writable: true,
  value: {
    reload: vi.fn(),
    href: "",
    pathname: "/",
  },
});

// Mock de document methods comunes
Object.defineProperty(document, "getElementById", {
  writable: true,
  value: vi.fn(),
});

// Mock de localStorage
const localStorageMock = {
  getItem: vi.fn(),
  setItem: vi.fn(),
  removeItem: vi.fn(),
  clear: vi.fn(),
};

Object.defineProperty(window, "localStorage", {
  value: localStorageMock,
});

// Mock de Cloudflare Turnstile
(global.window as any).turnstile = {
  render: vi.fn(),
  getResponse: vi.fn(() => "test-token"),
  reset: vi.fn(),
};
