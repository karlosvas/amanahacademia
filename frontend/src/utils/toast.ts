import { FirebaseError } from "firebase/app";
import toast from "solid-toast";

export async function toastErrorFirebase(errorData: any, loadingToastId: string, backendResponse: Response) {
  if (errorData instanceof FirebaseError) {
    const errorData = await backendResponse.json();
    toast.error(errorData.error.message, {
      id: loadingToastId,
      duration: 3000,
    });
  } else {
    toast.error("Error en la autenticación", {
      id: loadingToastId,
      duration: 3000,
    });
  }
}
