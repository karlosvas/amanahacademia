import toast from "solid-toast";
import { showModalAnimation } from "../utils/modals";
import type { ContactMailchimp, UserRequest } from "@/types/bakend-types";
import { ApiService } from "./helper";
import { FrontendErrorCode, getErrorToast } from "@/enums/enums";
import type { User } from "firebase/auth";

// Import estático de Firebase
import { initializeApp } from "firebase/app";
import {
  getAuth,
  signInWithEmailAndPassword,
  GoogleAuthProvider,
  signInWithPopup,
  onAuthStateChanged as firebaseOnAuthStateChanged,
} from "firebase/auth";

const firebaseConfig = {
  apiKey: import.meta.env.PUBLIC_FIREBASE_API_KEY,
  authDomain: import.meta.env.PUBLIC_FIREBASE_AUTH_DOMAIN,
  projectId: import.meta.env.PUBLIC_FIREBASE_PROJECT_ID,
  storageBucket: import.meta.env.PUBLIC_FIREBASE_STORAGE_BUCKET,
  messagingSenderId: import.meta.env.PUBLIC_FIREBASE_MESSAGING_SENDER_ID,
  appId: import.meta.env.PUBLIC_FIREBASE_APP_ID,
  measurementId: import.meta.env.PUBLIC_FIREBASE_MEASUREMENT_ID,
};

// Inicializamos Firebase y Auth
const firebaseApp = initializeApp(firebaseConfig);
const firebaseAuth = getAuth(firebaseApp);

// Funciones helper
export function getFirebaseAuth() {
  return firebaseAuth;
}

export function getGoogleProvider() {
  return new GoogleAuthProvider();
}

// Cambiar de login a register
export function toggleLoginToRegister(
  authModalLogin: HTMLDialogElement,
  authModalRegister: HTMLDialogElement,
  formLogin: HTMLFormElement,
  formRegister: HTMLFormElement,
  isRegister: boolean
) {
  isRegister = !isRegister;
  const showModal = isRegister ? authModalRegister : authModalLogin;
  const hideModal = isRegister ? authModalLogin : authModalRegister;
  const form = isRegister ? formRegister : formLogin;

  form.reset();
  hideModal.close();
  showModalAnimation(showModal, form, true);

  return isRegister;
}

// Enviar formularios
export function submitForm(
  modal: HTMLDialogElement,
  loading: HTMLElement,
  form: string,
  isRegister: boolean,
  errorMessage: HTMLElement
) {
  const validation = new window.JustValidate(form, {
    errorFieldCssClass: "border-red",
    errorLabelStyle: { color: "#e53e3e", fontSize: "0.875rem" },
  });

  validation
    .addField('[name="email"]', [
      { rule: "required", errorMessage: "El email es obligatorio" },
      { rule: "email", errorMessage: "Introduce un email válido" },
    ])
    .addField('[name="password"]', [
      { rule: "required", errorMessage: "La contraseña es obligatoria" },
      { rule: "minLength", value: 6, errorMessage: "Mínimo 6 caracteres" },
    ]);

  if (isRegister) {
    validation
      .addField('[name="name"]', [{ rule: "required", errorMessage: "El nombre es obligatorio" }])
      .addField('[name="privacy"]', [{ rule: "required", errorMessage: "Debes aceptar la política de privacidad" }])
      .addField('[name="terms"]', [{ rule: "required", errorMessage: "Debes aceptar los términos y condiciones" }]);
  }

  validation.onSuccess(async (event: Event) => {
    event.preventDefault();
    const formData = new FormData(event.target as HTMLFormElement);
    const credentials = Object.fromEntries(formData.entries());
    const userRequest: UserRequest = {
      name: (credentials.name as string) || "",
      email: credentials.email as string,
      password: credentials.password as string,
    };

    loading.classList.remove("hidden");

    try {
      const formHTML = document.getElementById(form.startsWith("#") ? form.slice(1) : form) as HTMLFormElement;
      const turnstileDiv = formHTML.querySelector(".cf-turnstile");
      if (typeof window.turnstile !== "undefined" && turnstileDiv) {
        window.turnstile.execute(turnstileDiv, {
          async callback() {},
          "error-callback": (error) => {
            console.error("Error de Turnstile:", error);
            throw new Error("Error en la verificación, por favor recarga la página.");
          },
        });
      }

      const helper = new ApiService();
      let response;
      if (isRegister) response = await helper.registerUser(userRequest);
      else response = await helper.loginUser(userRequest);

      if (response.success) {
        if (formData.get("newsletter") === "on") {
          const newUserNewsletter: ContactMailchimp = {
            email_address: userRequest.email,
            status: "subscribed",
          };
          const newsletterResponse = await helper.addContactToNewsletter(newUserNewsletter);
          if (!newsletterResponse.success) {
            console.error("Error adding user to newsletter");
            toast.error(getErrorToast(FrontendErrorCode.NEWSLETTER_ERROR));
          }
        }

        await signInWithEmailAndPassword(firebaseAuth, credentials.email as string, credentials.password as string);
        modal.close();
        setTimeout(() => {
          toast.success(
            isRegister ? "¡Registro exitoso! Vamos a empezar con tu primer curso." : "¡Bienvenido de vuelta!"
          );
          formHTML.reset();
          errorMessage.classList.add("hidden");
        }, 300);
      } else if (response.error) {
        throw new Error(
          typeof response.error === "string" ? response.error : response.error?.message || "Error desconocido"
        );
      } else {
        throw new Error("Error desconocido");
      }
    } catch (error: unknown) {
      console.error("Error en la autenticación:", error);
      errorMessage.classList.remove("hidden");
      let message;
      if (error instanceof DOMException) message = "*Error de red. Por favor, inténtalo de nuevo más tarde.";
      else if (typeof error === "string") message = "*" + error;
      else if (typeof error === "object" && error !== null && "message" in error)
        message = "*" + String((error as any).message);
      else message = "*Error desconocido";
      errorMessage.textContent = message;
    } finally {
      loading.classList.add("hidden");
    }
  });
}

// Logout
export async function handleLogout(): Promise<void> {
  try {
    await firebaseAuth.signOut();
    toast.success("Sesión cerrada correctamente");
  } catch (error) {
    console.error("Error during logout:", error);
  }
}

// Setup Auth
export function setupAuth(
  user: User | null,
  authModalLogin: HTMLDialogElement,
  formLogin: HTMLFormElement,
  headerData: { button: { login: string; logout: string } }
) {
  const identificationButton = window.matchMedia("(min-width: 1024px)").matches
    ? document.getElementById("identification")
    : document.getElementById("identification-menu");

  if (!identificationButton || !headerData?.button) return;

  if (user) {
    identificationButton.textContent = headerData.button.logout;
    identificationButton.onclick = handleLogout;
  } else {
    identificationButton.textContent = headerData.button.login;
    identificationButton.onclick = () => {
      if (authModalLogin && !authModalLogin.classList.contains("hidden") && formLogin)
        showModalAnimation(authModalLogin, formLogin, true);
    };
  }
}

// Google login
export async function handleLogGoogleProvider(
  modal: HTMLDialogElement,
  formHTML: HTMLFormElement,
  isRegister: boolean,
  loginError: HTMLDivElement
) {
  try {
    const provider = getGoogleProvider();
    await signInWithPopup(firebaseAuth, provider);
    modal.close();
    setTimeout(() => {
      toast.success(isRegister ? "¡Registro exitoso! Vamos a empezar con tu primer curso." : "¡Bienvenido de vuelta!");
      formHTML.reset();
      loginError.classList.add("hidden");
    }, 300);
  } catch (error) {
    console.error("Error during Google sign-in:", error);
    loginError.textContent = "Error al iniciar sesión con Google. Por favor, inténtalo de nuevo.";
    loginError.classList.remove("hidden");
  }
}

// Token actual
export async function getCurrentUserToken(): Promise<string | null> {
  const currentUser = firebaseAuth.currentUser;
  return currentUser ? await currentUser.getIdToken() : null;
}

// Auth state listener
export function onAuthStateChanged(callback: (user: User | null) => void) {
  return firebaseOnAuthStateChanged(firebaseAuth, callback);
}
