import toast from "solid-toast";
import { closeModalAnimation, showModalAnimation } from "@/utils/modals";
import { ApiService } from "@/services/helper";
import {
  FrontendErrorCode,
  getErrorToast,
  AuthSuccessCode,
  getAuthSuccessMessage,
  ValidationCode,
  getValidationMessage,
} from "@/enums/enums";
import { initializeApp } from "firebase/app";
import {
  getAuth,
  signInWithEmailAndPassword,
  GoogleAuthProvider,
  signInWithPopup,
  onAuthStateChanged as firebaseOnAuthStateChanged,
} from "firebase/auth";
import { executeTurnstileIfPresent } from "@/services/claudflare";
import { log } from "@/services/logger";

// Tipos
import type { ResponseAPI, UserMerged, UserRequest } from "@/types/bakend-types";
import type { Auth, User } from "firebase/auth";

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

// Proveedor de Google
export function getGoogleProvider() {
  return new GoogleAuthProvider();
}

// Cambiar de login a register
export function toggleLoginToRegister(
  authModalLogin: HTMLDialogElement,
  authModalRegister: HTMLDialogElement,
  formLogin: HTMLFormElement,
  formRegister: HTMLFormElement,
  isRegister: boolean,
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

// Login o Register con email y password
export function submitFormToRegisterOrLogin(
  modal: HTMLDialogElement,
  loading: HTMLElement,
  form: string,
  isRegister: boolean,
  errorMessage: HTMLElement,
): Promise<{ success: boolean; userRequest?: UserRequest; formData?: FormData }> {
  return new Promise<{ success: boolean; formData?: FormData; userRequest?: UserRequest }>(async (resolve) => {
    const validation = new globalThis.JustValidate(form, {
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

      const formData = new FormData(event.target as HTMLFormElement);
      const getFormValue = (name: string): string => {
        const value = formData.get(name);
        return typeof value === "string" ? value : "";
      };

      const userRequest: UserRequest = {
        name: getFormValue("name"),
        email: getFormValue("email"),
        password: getFormValue("password"),
        provider: "email",
        first_free_class: false,
      };

      // Mostramos el loading
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
          // Una vez creado el usuario desde el backend lo logeamos desde el frontend
          await signInWithEmailAndPassword(firebaseAuth, userRequest.email, userRequest.password);

          modal.close();
          setTimeout(() => {
            toast.success(
              getAuthSuccessMessage(isRegister ? AuthSuccessCode.REGISTER_SUCCESS : AuthSuccessCode.LOGIN_SUCCESS),
            );
            formHTML.reset();
            errorMessage.classList.add("hidden");
          }, 300);
          resolve({ success: true, formData, userRequest });
        } else {
          const respError: any = response.error;
          throw new Error(typeof respError === "string" ? respError : respError?.message || "Error desconocido");
        }
      } catch (error: unknown) {
        log.error("Error during authentication", error);
        errorMessage.classList.remove("hidden");
        errorMessage.textContent =
          error instanceof DOMException
            ? "*" + getErrorToast(FrontendErrorCode.NETWORK_ERROR)
            : "*" + getErrorToast(FrontendErrorCode.UNKNOWN_ERROR);
        resolve({ success: false });
      } finally {
        loading.classList.add("hidden");
      }
    });

    validation.onFail(() => {
      resolve({ success: false });
    });
  });
}

// Logout
export async function handleLogout(): Promise<void> {
  try {
    // Limpiamos la cookie de auth
    await firebaseAuth.signOut();
    location.reload();
  } catch (error) {
    log.error("Error during logout:", error);
  }
}

// Setup Auth
export function setupAuth(
  user: User | null,
  authModalLogin: HTMLDialogElement,
  formLogin: HTMLFormElement,
  headerData: { button: { login: string; logout: string } },
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

// Google login y register
export async function handleLogGoogleProvider(
  modal: HTMLDialogElement,
  formHTML: HTMLFormElement,
  isRegister: boolean,
  loginError: HTMLDivElement,
): Promise<{ success: boolean; userRequest?: UserRequest; formData?: FormData }> {
  try {
    log.info(`[handleLogGoogleProvider] Iniciando ${isRegister ? "registro" : "login"} con Google`);
    const googleProvider: GoogleAuthProvider = getGoogleProvider();
    log.info("[handleLogGoogleProvider] Abriendo popup de Google");
    await signInWithPopup(firebaseAuth, googleProvider);
    log.info("[handleLogGoogleProvider] Popup de Google cerrado exitosamente");

    // Get the ID token from Firebase
    const idToken = await firebaseAuth.currentUser?.getIdToken();
    log.info(`[handleLogGoogleProvider] Token obtenido: ${idToken ? "Sí" : "No"}`);
    if (!idToken) throw new Error("No se pudo obtener el token de autenticación");

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
      await handleLogout();

      // Manejo específico si el usuario no existe
      loginError.textContent = getErrorToast(FrontendErrorCode.USER_NOT_EXISTS);
      loginError.classList.remove("hidden");
      log.error(FrontendErrorCode.USER_NOT_EXISTS);
      return { success: false };
    }

    // Al ser el registro con google el formulario actual no tiene que ver con el registro asique creamos un form data custom
    // para poder registrarlo
    const formData = new FormData();
    formData.append("newsletter", "on");

    // Cerramos el modal
    closeModalAnimation(modal, formHTML);

    // Mostramos el toast de éxito
    setTimeout(() => {
      toast.success(
        isRegister
          ? getAuthSuccessMessage(AuthSuccessCode.REGISTER_SUCCESS)
          : getAuthSuccessMessage(AuthSuccessCode.LOGIN_SUCCESS),
      );
      formHTML.reset();
      loginError.classList.add("hidden");
    }, 300);
    return { success: true, formData, userRequest };
  } catch (error) {
    log.error("Error during Google login/register", error);
    loginError.textContent = getErrorToast(FrontendErrorCode.GOOGLE_LOGIN_ERROR);
    loginError.classList.remove("hidden");
    return { success: false };
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
