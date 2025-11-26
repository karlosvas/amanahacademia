import toast from "solid-toast";
import { closeModalAnimation, showModalAnimation } from "../utils/modals";
import type { ResponseAPI, UserMerged, UserRequest } from "@/types/bakend-types";
import { ApiService } from "./helper";
import {
  FrontendErrorCode,
  getErrorToast,
  AuthSuccessCode,
  getAuthSuccessMessage,
  ValidationCode,
  getValidationMessage,
} from "@/enums/enums";
import type { Auth, User } from "firebase/auth";

// Import estático de Firebase
import { initializeApp } from "firebase/app";
import {
  getAuth,
  signInWithEmailAndPassword,
  GoogleAuthProvider,
  signInWithPopup,
  onAuthStateChanged as firebaseOnAuthStateChanged,
} from "firebase/auth";
import { executeTurnstileIfPresent } from "./claudflare";
import { suscribeToNewsletter } from "./mailchimp";
import { log } from "./logger";

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
export function getFirebaseAuth(): Auth {
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
export function submitFormToRegisterOrLogin(
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
      { rule: "required", errorMessage: getValidationMessage(ValidationCode.EMAIL_REQUIRED) },
      { rule: "email", errorMessage: getValidationMessage(ValidationCode.EMAIL_INVALID) },
    ])
    .addField('[name="password"]', [
      { rule: "required", errorMessage: getValidationMessage(ValidationCode.PASSWORD_REQUIRED) },
      { rule: "minLength", value: 6, errorMessage: getValidationMessage(ValidationCode.PASSWORD_MIN) },
    ]);

  if (isRegister) {
    validation
      .addField('[name="name"]', [
        { rule: "required", errorMessage: getValidationMessage(ValidationCode.NAME_REQUIRED) },
      ])
      .addField('[name="privacy"]', [
        { rule: "required", errorMessage: getValidationMessage(ValidationCode.PRIVACY_REQUIRED) },
      ])
      .addField('[name="terms"]', [
        { rule: "required", errorMessage: getValidationMessage(ValidationCode.TERMS_REQUIRED) },
      ]);
  }

  validation.onSuccess(async (event: Event) => {
    event.preventDefault();

    // Obtenemos los datos del formulario
    const formData = new FormData(event.target as HTMLFormElement);
    const userRequest: UserRequest = {
      name: toStringFormValue(formData.get("name") || ""),
      email: toStringFormValue(formData.get("email") || ""),
      password: toStringFormValue(formData.get("password") || ""),
      provider: "email",
      first_free_class: false,
    };

    loading.classList.remove("hidden");

    try {
      // Obtenemos datos del formulario
      const id = form.startsWith("#") ? form.slice(1) : form;
      const formHTML = document.getElementById(id) as HTMLFormElement | null;
      if (!formHTML) throw new Error("Formulario no encontrado");

      // Ejecutamos Turnstile
      await executeTurnstileIfPresent(formHTML);

      const helper = new ApiService();
      let response: ResponseAPI<UserMerged | string> = isRegister
        ? await helper.registerUser(userRequest)
        : await helper.loginUser(userRequest);

      if (response.success) {
        // Si el usuario se ha registrado y ha marcado la opción de newsletter, lo añadimos
        if (isRegister) await suscribeToNewsletter(formData, userRequest);

        // Una vez creado el usuario desde el backend lo logeamos desde el frontend
        const cred = await signInWithEmailAndPassword(firebaseAuth, userRequest.email, userRequest.password);

        // Lo guardamos en las cookies
        const token = await cred.user.getIdToken();

        modal.close();
        setTimeout(() => {
          toast.success(
            getAuthSuccessMessage(isRegister ? AuthSuccessCode.REGISTER_SUCCESS : AuthSuccessCode.LOGIN_SUCCESS)
          );
          formHTML.reset();
          errorMessage.classList.add("hidden");
        }, 300);
      } else {
        const respError: any = response.error;
        throw new Error(typeof respError === "string" ? respError : respError?.message || "Error desconocido");
      }
    } catch (error: unknown) {
      console.error(error);
      errorMessage.classList.remove("hidden");
      errorMessage.textContent =
        error instanceof DOMException
          ? "*" + getErrorToast(FrontendErrorCode.NETWORK_ERROR)
          : "*" + getErrorToast(FrontendErrorCode.UNKNOWN_ERROR);
    } finally {
      loading.classList.add("hidden");
    }
  });
}

function toStringFormValue(v: FormDataEntryValue | undefined): string {
  return typeof v === "string" ? v : "";
}

// Logout
export async function handleLogout(): Promise<void> {
  try {
    await firebaseAuth.signOut();
    // Limpiamos la cookie de auth
    location.reload();
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
  const identificationButton = matchMedia("(min-width: 1024px)").matches
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
    const googleProvider: GoogleAuthProvider = getGoogleProvider();
    await signInWithPopup(firebaseAuth, googleProvider);

    // Get the ID token from Firebase
    const idToken = await firebaseAuth.currentUser?.getIdToken();
    if (!idToken) {
      throw new Error("No se pudo obtener el token de autenticación");
    }

    // Registramos o logeamos al usuario según corresponda
    const helper = new ApiService();
    const userRequest: UserRequest = {
      name: firebaseAuth.currentUser?.displayName || "",
      email: firebaseAuth.currentUser?.email || "",
      password: "",
      provider: "google",
      id_token: idToken,
      first_free_class: false,
    };

    // Intentamos registrar o logear al usuario
    let response: ResponseAPI<string> = isRegister
      ? await helper.registerUser(userRequest)
      : await helper.loginUser(userRequest);

    if (!response.success) {
      // Deslogeamos al usuario de Firebase auth
      await firebaseAuth.signOut();

      // Manejo específico si el usuario no existe
      loginError.textContent = getErrorToast(FrontendErrorCode.USER_NOT_EXISTS);
      loginError.classList.remove("hidden");
      log.error(FrontendErrorCode.USER_NOT_EXISTS);
    }

    // Al ser el registro con google el formulario actual no tiene que ver con el registro asique creamos un form data custom
    // para poder registrarlo
    const formData = new FormData();
    formData.append("newsletter", "on");

    //  Si el usuario se está registrando lo añadimos al newsletter
    if (isRegister) await suscribeToNewsletter(formData, userRequest);

    // Comprobamos si es un usuario nuevo
    closeModalAnimation(modal, formHTML);

    // Devolbemos el scroll
    setTimeout(() => {
      toast.success(
        isRegister
          ? getAuthSuccessMessage(AuthSuccessCode.REGISTER_SUCCESS)
          : getAuthSuccessMessage(AuthSuccessCode.LOGIN_SUCCESS)
      );
      formHTML.reset();
      loginError.classList.add("hidden");
    }, 300);
  } catch (error) {
    log.error("Error during Google sign-in:", error);
    loginError.textContent = getErrorToast(FrontendErrorCode.GOOGLE_LOGIN_ERROR);
    loginError.classList.remove("hidden");
  }
}

// Token actual
export async function getCurrentUserToken(): Promise<string | null> {
  try {
    const currentUser = firebaseAuth.currentUser;
    return currentUser ? await currentUser.getIdToken() : null;
  } catch (error) {
    log.error("Error getting token:", error);
    return null;
  }
}

// Auth state listener
export function onAuthStateChanged(callback: (user: User | null) => void) {
  return firebaseOnAuthStateChanged(firebaseAuth, callback);
}
