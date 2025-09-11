import { signInWithEmailAndPassword, type User } from "firebase/auth";
import toast from "solid-toast";
// import { toastErrorFirebase } from "./toast";
import { showModalAnimation } from "./modals";
import { auth } from "@/lib/firebase";
import type { IdentificationI18n } from "@/types/types";

// Actualiza el botón de identificación
export function openIdentificationModalOrLoggout(
  user: User | null,
  authModalLogin: HTMLDialogElement,
  formLogin: HTMLFormElement,
  auth: any,
  headerData: IdentificationI18n
) {
  const identificationButton = getIdentificationButton();
  if (!identificationButton) return;
  if (user) {
    identificationButton.textContent = headerData.button.logout;
    identificationButton.onclick = async () => await auth.signOut();
  } else {
    identificationButton.textContent = headerData.button.login;
    identificationButton.onclick = () => {
      if (authModalLogin && !authModalLogin.classList.contains("hidden") && formLogin)
        showModalAnimation(authModalLogin, formLogin, true);
    };
  }
}

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
      .addField('[name="terms"]', [{ rule: "required", errorMessage: "Debes aceptar los términos y condiciones" }])
      .addField('[name="newsletter"]', [{ rule: "required", errorMessage: "Debes aceptar recibir novedades" }]);
  }

  // Si todo sale bien
  validation.onSuccess(async (event: Event) => {
    event.preventDefault();

    // Datos introducidos
    const formData = new FormData(event.target as HTMLFormElement);
    const credentials = Object.fromEntries(formData.entries());

    // Mostrar loading, ocultar errores
    loading.classList.remove("hidden");

    try {
      // Cloudlflare Turnstile
      const formHTML = document.getElementById(form.startsWith("#") ? form.slice(1) : form) as HTMLFormElement;
      const turnstileDiv = formHTML.querySelector(".cf-turnstile");
      if (typeof window.turnstile !== "undefined" && turnstileDiv) {
        window.turnstile.execute(turnstileDiv, {
          async callback(token: string) {
            console.log("Token Turnstile:", token);
          },
          "error-callback": function (error: any) {
            console.error("Error de Turnstile:", error);
            throw new Error("Error en la verificación, por favor recarga la página.");
          },
        });
      }

      // URL de la petición
      let url = import.meta.env.PUBLIC_BACKEND_URL;
      if (!url) throw new Error("PUBLIC_BACKEND_URL no definida");
      if (isRegister) url += "/user/register";
      else url += "/user/login";

      // Validar credenciales en backend, registrandonos o logeandonos segun corresponda
      const backendResponse = await fetch(url, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(credentials),
      });

      if (backendResponse.ok && credentials.email && credentials.password && form) {
        // Si todo salió bien, hacer login directo en Firebase, con email y contraseña
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
      } else {
        const errorData = await backendResponse.json();
        console.error(errorData);
        throw new Error("Credenciales inválidas");
      }
    } catch (error: Error | unknown) {
      console.error("Error en la autenticación:", error);
      errorMessage.classList.remove("hidden");
      let message = "*Error desconocido";
      if (error instanceof Error) {
        message = "*" + error.message;
      } else if (typeof error === "string") {
        message = "*" + error;
      } else if (typeof error === "object" && error !== null && "message" in error) {
        message = "*" + String((error as any).message);
      }
      errorMessage.textContent = message;
    } finally {
      // Finalmente quitamos la carga
      loading.classList.add("hidden");
    }
  });
}
