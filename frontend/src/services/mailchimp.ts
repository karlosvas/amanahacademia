import type { ContactMailchimp, UserRequest } from "@/types/bakend-types";
import { ApiService } from "./helper";
import { FrontendErrorCode, getErrorToast } from "@/enums/enums";
import toast from "solid-toast";

export async function suscribeToNewsletter(formData: FormData, userRequest: UserRequest) {
  const helper = new ApiService();
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
}
