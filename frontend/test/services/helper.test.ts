// We recommend installing an extension to run vitest tests.
import {
  describe,
  it,
  expect,
  beforeEach,
  afterEach,
  vi,
  type Mock,
} from "vitest";
import { ApiService } from "@/services/helper.ts";
import { getCurrentUserToken } from "@/services/firebase.ts";

// Mock de dependencias
vi.mock("@/services/firebase", () => ({
  getCurrentUserToken: vi.fn(() => Promise.resolve("mock-token")),
}));

vi.mock("@/services/logger", () => ({
  log: {
    debug: vi.fn(),
    info: vi.fn(),
    warn: vi.fn(),
    error: vi.fn(),
  },
}));

const mockFetchResponse = (status: number, body: any, asJson = true) => {
  const textPayload = asJson ? JSON.stringify(body) : body;
  return {
    status,
    text: vi.fn().mockResolvedValue(textPayload),
    json: vi.fn().mockResolvedValue(body),
  } as any;
};

const mockedGetCurrentUserToken = getCurrentUserToken as unknown as Mock;
let mockedFetch: Mock;
let consoleErrorSpy: any;

describe("ApiService", () => {
  let apiService: ApiService;

  beforeEach(() => {
    vi.clearAllMocks();
    consoleErrorSpy = vi.spyOn(console, "error").mockImplementation(() => {});
    (globalThis as any).fetch = vi.fn();
    mockedFetch = globalThis.fetch as unknown as Mock;
    apiService = new ApiService();
  });

  afterEach(() => {
    consoleErrorSpy.mockRestore();
  });

  //////////////////// COMENTARIOS ////////////////////
  it("should return data when fetch returns valid JSON", async () => {
    const payload = { success: true, data: [{ id: "1" }] };
    mockedFetch.mockResolvedValueOnce(mockFetchResponse(200, payload));

    const result = await apiService.getAllComments();

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/comments/all",
      { method: "GET" },
    );
    expect(result).toEqual(payload);
  });

  it("should handle 204 responses with undefined data", async () => {
    mockedFetch.mockResolvedValueOnce({ status: 204 } as any);

    const result = await apiService.getAllComments();

    expect(result).toEqual({ success: true, data: undefined });
  });

  it("should surface invalid JSON as an error", async () => {
    mockedFetch.mockResolvedValueOnce({
      status: 200,
      text: vi.fn().mockResolvedValue("not-json"),
    } as any);

    const result = await apiService.getAllComments();

    expect(result.success).toBe(false);
    if (!result.success) {
      expect(result.error).toContain(
        "Network error: Invalid response object from fetch",
      );
    }
  });

  it("should map network failures to a ResponseAPI error", async () => {
    mockedFetch.mockRejectedValueOnce(new Error("boom"));

    const result = await apiService.getAllComments();

    expect(result).toEqual({
      success: false,
      error: "Network error: Invalid response object from fetch",
    });
  });

  it("should include auth token when posting comments", async () => {
    const comment = { content: "hola" } as any;
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, {
        success: true,
        data: comment,
      }),
    );

    await apiService.postComment(comment);

    expect(mockedGetCurrentUserToken).toHaveBeenCalledTimes(1);
    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/comments/add",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer mock-token",
        },
        body: JSON.stringify(comment),
      },
    );
  });

  it("should call setLike with PUT and auth header", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: {} }),
    );

    await apiService.setLike("123");

    expect(mockedGetCurrentUserToken).toHaveBeenCalled();
    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/comments/like/123",
      {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer mock-token",
        },
      },
    );
  });

  it("should delete a comment with DELETE and auth header", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true }),
    );

    await apiService.deleteComment("abc");

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/comments/del/abc",
      {
        method: "DELETE",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer mock-token",
        },
      },
    );
  });

  it("should edit a comment with body and auth header", async () => {
    const payload = { message: "hi" };
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true }),
    );

    await apiService.editComment("abc", payload as any);

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/comments/edit/abc",
      {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer mock-token",
        },
        body: JSON.stringify(payload),
      },
    );
  });

  //////////////////// PROFESORES ////////////////////

  //////////////////// CONNECTIONS ////////////////////
  it("should prefer provided cookie token over firebase token", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: [] }),
    );

    await apiService.getAllConnections("cookie-token");

    expect(mockedGetCurrentUserToken).not.toHaveBeenCalled();
    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/payment/cal/connection/all",
      {
        method: "GET",
        headers: {
          Authorization: "Bearer cookie-token",
        },
      },
    );
  });

  //////////////////// BOOKINGS ////////////////////
  it("should create bookings using the base URL and return parsed payload", async () => {
    const bookingResponse = { success: true, data: { id: "bk-1" } };
    mockedFetch.mockResolvedValueOnce({
      status: 200,
      json: vi.fn().mockResolvedValue(bookingResponse),
    } as any);

    const result = await apiService.createBooking({} as any, "cookie-token");

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/cal/bookings",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer cookie-token",
        },
        body: JSON.stringify({}),
      },
    );
    expect(result).toEqual(bookingResponse);
  });

  it("should use cookie token for paid reservations when provided", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: [] }),
    );

    await apiService.getPaidReservations("cookie-token");

    expect(mockedGetCurrentUserToken).not.toHaveBeenCalled();
    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/payment/history",
      {
        method: "GET",
        headers: {
          Authorization: "Bearer cookie-token",
        },
      },
    );
  });

  it("should fetch booking by id with provided cookie token", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: { id: "bk" } }),
    );

    await apiService.getBookingById("bk", "cookie-token");

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/cal/bookings/bk",
      {
        method: "GET",
        headers: { Authorization: "Bearer cookie-token" },
      },
    );
  });

  it("should confirm booking with auth header", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true }),
    );

    await apiService.confirmBooking("bk-123");

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/cal/bookings/bk-123/confirm",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer mock-token",
        },
      },
    );
  });

  //////////////////// COMENTARIOS - Métodos faltantes ////////////////////
  it("should get comment by id with auth", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: { id: "123" } }),
    );

    await apiService.getCommentById("123");

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/comments/123",
      {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer mock-token",
        },
      },
    );
  });

  it("should create reply to comment", async () => {
    const reply = { content: "respuesta" } as any;
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: reply }),
    );

    await apiService.createReply("parent-123", reply);

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/comments/reply/parent-123",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer mock-token",
        },
        body: JSON.stringify(reply),
      },
    );
  });

  it("should edit reply", async () => {
    const reply = { content: "editado" } as any;
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: reply }),
    );

    await apiService.editReply("comment-123", "reply-456", reply);

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/comments/reply/comment-123/reply-456/edit",
      {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer mock-token",
        },
        body: JSON.stringify(reply),
      },
    );
  });

  it("should delete reply", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true }),
    );

    await apiService.deleteReply("comment-123", "reply-456");

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/comments/del/comment-123/reply/reply-456",
      {
        method: "DELETE",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer mock-token",
        },
      },
    );
  });

  it("should get comment reply by id", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: {} }),
    );

    await apiService.getCommentReplyById("comment-123", "reply-456");

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/comments/comment-123/reply/reply-456",
      {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer mock-token",
        },
      },
    );
  });

  //////////////////// PROFESORES ////////////////////
  it("should get teacher by id", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: { id: "teacher-1" } }),
    );

    await apiService.getTeacher("teacher-1");

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/teachers/teacher-1",
      {
        method: "GET",
        headers: { "Content-Type": "application/json" },
      },
    );
  });

  it("should get all teachers", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: [] }),
    );

    await apiService.getTeachers();

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/teachers/all",
      {
        method: "GET",
        headers: { "Content-Type": "application/json" },
      },
    );
  });

  it("should get available schedule", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: {} }),
    );

    await apiService.getAvailableTimeSchedule("schedule-1");

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/cal/schedule/schedule-1",
      {
        method: "GET",
        headers: { "Content-Type": "application/json" },
      },
    );
  });

  //////////////////// RESEND & USUARIOS ////////////////////
  it("should send contact email", async () => {
    const email = { to: "test@test.com", subject: "Test" } as any;
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: {} }),
    );

    await apiService.sendContact(email);

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/email/contact",
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(email),
      },
    );
  });

  it("should register user", async () => {
    const user = { email: "user@test.com", password: "pass" } as any;
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: "user-id" }),
    );

    await apiService.registerUser(user);

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/users/register",
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(user),
      },
    );
  });

  it("should login user", async () => {
    const user = { email: "user@test.com", password: "pass" } as any;
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: "token" }),
    );

    await apiService.loginUser(user);

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/users/login",
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(user),
      },
    );
  });

  it("should get current user", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: {} }),
    );

    await apiService.getUser();

    expect(mockedFetch).toHaveBeenCalledWith("http://localhost:3000/users/me", {
      method: "GET",
      headers: { Authorization: "Bearer mock-token" },
    });
  });

  it("should check if user is admin", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: true }),
    );

    await apiService.isAdminUser("cookie-token");

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/users/admin_check",
      {
        method: "GET",
        headers: { Authorization: "Bearer mock-token" },
      },
    );
  });

  //////////////////// MAILCHIMP & STRIPE ////////////////////
  it("should add contact to newsletter", async () => {
    const contact = { email: "test@test.com" } as any;
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: {} }),
    );

    await apiService.addContactToNewsletter(contact);

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/mailchimp/add_contact",
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(contact),
      },
    );
  });

  it("should create checkout payment intent", async () => {
    const payload = { amount: 100 } as any;
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: {} }),
    );

    await apiService.checkout(payload);

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/payment/intent",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer mock-token",
        },
        body: JSON.stringify(payload),
      },
    );
  });

  it("should save cal-stripe connection", async () => {
    const payload = { calId: "cal-1", stripeId: "stripe-1" } as any;
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true }),
    );

    await apiService.saveCalStripeConnection(payload);

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/payment/cal/connection",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer mock-token",
        },
        body: JSON.stringify(payload),
      },
    );
  });

  it("should get group bookings", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: [] }),
    );

    await apiService.getGroupBookings("cookie-token");

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/cal/bookings/all",
      {
        method: "GET",
        headers: { Authorization: "Bearer cookie-token" },
      },
    );
  });

  //////////////////// ANALYTICS ////////////////////
  it("should get user metrics", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: {} }),
    );

    await apiService.getUserMetrics("cookie-token");

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/metrics/users",
      {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer cookie-token",
        },
      },
    );
  });

  it("should get articles metrics", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: {} }),
    );

    await apiService.getArticlesMetrics();

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/metrics/articles",
      {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer mock-token",
        },
      },
    );
  });

  it("should get class metrics", async () => {
    mockedFetch.mockResolvedValueOnce(
      mockFetchResponse(200, { success: true, data: {} }),
    );

    await apiService.getClassMetrics();

    expect(mockedFetch).toHaveBeenCalledWith(
      "http://localhost:3000/metrics/class",
      {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer mock-token",
        },
      },
    );
  });
});
