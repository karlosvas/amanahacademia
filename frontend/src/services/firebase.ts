import { signInWithEmailAndPassword, signInWithPopup, type User } from "firebase/auth";
import toast from "solid-toast";
import { showModalAnimation } from "../utils/modals";
import { auth, googleProvider } from "@/config/firebase";
import type { ContactMailchimp, UserRequest } from "@/types/bakend-types";
import { ApiService } from "./helper";
import { FrontendErrorCode, getErrorMessage } from "@/types/enums";

// Obtener el botón de identificación segun el tamaño de la pantalla
export function getIdentificationButton() {
  if (window.matchMedia("(min-width: 1024px)").matches) return document.getElementById("identification");
  else return document.getElementById("identification-menu");
}

// Cambiar de login a register
export function toggleLoginToRegister(
  authModalLogin: HTMLDialogElement,
  authModalRegister: HTMLDialogElement,
  formLogin: HTMLFormElement,
  formRegister: HTMLFormElement,
  isRegister: boolean
) {
  // Cambiamos el estado a su contrario
  isRegister = !isRegister;

  // Determinamos qué modal vamos a cerrar y cuál a abrir
  const showModal = isRegister ? authModalRegister : authModalLogin;
  const hideModal = isRegister ? authModalLogin : authModalRegister;
  const form = isRegister ? formRegister : formLogin;

  // Reseteamos el formulario
  form.reset();
  // Cierra el modal anterior, sin ocultarlo con hidden
  hideModal.close();
  // Elimina animaciones de cierre previas y muestra el nuevo modal
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
  // Inicialización de la validación
  const validation = new window.JustValidate(form, {
    errorFieldCssClass: "border-red",
    errorLabelStyle: {
      color: "#e53e3e",
      fontSize: "0.875rem",
    },
  });

  // Evento de enviar el formulario
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

  // Si sale bien
  validation.onSuccess(async (event: Event) => {
    event.preventDefault();

    // Datos introducidos
    const formData = new FormData(event.target as HTMLFormElement);
    const credentials = Object.fromEntries(formData.entries());
    const userRequest: UserRequest = {
      name: (credentials.name as string) || "",
      email: credentials.email as string,
      password: credentials.password as string,
    };

    // Mostrar loading, ocultar errores
    loading.classList.remove("hidden");

    try {
      // Cloudlflare Turnstile
      const formHTML = document.getElementById(form.startsWith("#") ? form.slice(1) : form) as HTMLFormElement;
      const turnstileDiv = formHTML.querySelector(".cf-turnstile");
      if (typeof window.turnstile !== "undefined" && turnstileDiv) {
        window.turnstile.execute(turnstileDiv, {
          async callback() {},
          "error-callback": function (error: any) {
            console.error("Error de Turnstile:", error);
            throw new Error("Error en la verificación, por favor recarga la página.");
          },
        });
      }

      const helper = new ApiService();
      let response;
      if (isRegister) response = await helper.registerUser(userRequest);
      else response = await helper.loginUser(userRequest);

      // Validar credenciales en backend, registrandonos o logeandonos segun corresponda
      if (response.success) {
        // Si selecionó la opcion de la newsletter le añadimos a nuesra lista de mailchimp
        if (formData.get("newsletter") === "on") {
          const newUserNewsletter: ContactMailchimp = {
            email_address: userRequest.email,
            status: "subscribed",
          };
          const response = await helper.addContactToNewsletter(newUserNewsletter);

          if (!response.success) {
            console.error("Error adding user to newsletter");
            toast.error(getErrorMessage(FrontendErrorCode.NEWSLETTER_ERROR));
          }
        }

        // Si salió bien, hacer login directo en Firebase, con email y contraseña
        await signInWithEmailAndPassword(auth, credentials.email as string, credentials.password as string);
        modal.close();
        // Esperamos a que se cierre para mostrar el mensaje de exito
        setTimeout(() => {
          toast.success(
            isRegister ? "¡Registro exitoso! Vamos a empezar con tu primer curso." : "¡Bienvenido de vuelta!"
          );
          formHTML.reset();
          errorMessage.classList.add("hidden");
        }, 300);
      } else if (response.error) {
        throw new Error(
          typeof response.error === "string"
            ? response.error
            : response.error?.message || "Error desconocido, por favor inténtalo de nuevo."
        );
      } else {
        throw new Error("Error desconocido, por favor inténtalo de nuevo.");
      }
    } catch (error: unknown) {
      console.error("Error en la autenticación:", error);
      errorMessage.classList.remove("hidden");
      let message;
      switch (error) {
        case error instanceof DOMException:
          message = "*Error de red. Por favor, inténtalo de nuevo más tarde.";
          break;
        case typeof error === "string":
          message = "*" + error;
          break;
        case typeof error === "object" && error !== null && "message" in error:
          message = "*" + String((error as any).message);
          break;
        default:
          message = "*Error desconocido";
      }
      errorMessage.textContent = message;
    } finally {
      // Finalmente quitamos la carga
      loading.classList.add("hidden");
    }
  });
}

// Función mejorada para logout
export async function handleLogout(): Promise<void> {
  try {
    await auth.signOut();
    toast.success("Sesión cerrada correctamente");
  } catch (error) {
    console.error("❌ Error during logout:", error);
    try {
      await auth.signOut();
    } catch (firebaseError) {
      console.error("❌ Firebase logout also failed:", firebaseError);
    }
  }
}

// Actualizar setupAuth para usar handleLogout
export function setupAuth(
  user: User | null,
  authModalLogin: HTMLDialogElement,
  formLogin: HTMLFormElement,
  headerData: { button: { login: string; logout: string } }
) {
  const identificationButton = getIdentificationButton();
  if (!identificationButton) return;

  // Validación defensiva
  if (!headerData?.button) {
    console.error("❌ headerData or headerData.button is undefined", headerData);
    return;
  }

  if (user) {
    identificationButton.textContent = headerData.button.logout;
    // Usar la nueva función de logout
    identificationButton.onclick = async () => {
      await handleLogout();
    };
  } else {
    identificationButton.textContent = headerData.button.login;
    identificationButton.onclick = () => {
      if (authModalLogin && !authModalLogin.classList.contains("hidden") && formLogin)
        showModalAnimation(authModalLogin, formLogin, true);
    };
  }
}

// Función mejorada para Google login
export async function handleLogGoogleProvider(
  modal: HTMLDialogElement,
  formHTML: HTMLFormElement,
  isRegister: boolean,
  loginError: HTMLDivElement
) {
  try {
    await signInWithPopup(auth, googleProvider);
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
