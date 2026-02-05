import { ApiErrorType } from "@/enums/enums";

export class ApiError extends Error {
  constructor(
    public type: ApiErrorType,
    public message: string,
    public statusCode?: number,
    public originalError?: Error,
  ) {
    super(message);
    this.name = "ApiError";
  }
}
