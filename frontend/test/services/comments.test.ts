import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import {
  processComments,
  updateLikeState,
  verifyAuthorInComment,
  verifyAuthorInCommentReply,
  handleSubmitReply,
  handleDeleteComment,
  handleEditComment,
  handleDeleteReply,
  handleEditReply,
  submitLike,
} from "@/services/comments";
import { getFirebaseAuth } from "@/services/firebase";
import type { Comment } from "@/types/bakend-types";
import toast from "solid-toast";

// Mocks
vi.mock("@/services/firebase", () => ({
  getFirebaseAuth: vi.fn(),
  getCurrentUserToken: vi.fn(() => Promise.resolve("mock-token")),
}));
vi.mock("solid-toast", () => ({
  default: {
    success: vi.fn(),
    error: vi.fn(),
  },
}));
vi.mock("@/services/logger", () => ({
  log: {
    debug: vi.fn(),
    info: vi.fn(),
    warn: vi.fn(),
    error: vi.fn(),
  },
}));

const mockFetchResponse = (status: number, body: any) => {
  const textPayload = JSON.stringify(body);
  return {
    status,
    text: vi.fn().mockResolvedValue(textPayload),
    json: vi.fn().mockResolvedValue(body),
  } as any;
};

describe("comments.ts", () => {
  let mockApiService: any;
  let consoleErrorSpy: any;

  beforeEach(() => {
    vi.clearAllMocks();
    consoleErrorSpy = vi.spyOn(console, "error").mockImplementation(() => {});

    // Mock fetch
    globalThis.fetch = vi.fn();

    // Mock Firebase Auth
    vi.mocked(getFirebaseAuth).mockReturnValue({
      currentUser: {
        uid: "user-123",
        displayName: "Test User",
        photoURL: "https://example.com/photo.jpg",
        email: "test@test.com",
      },
    } as any);

    // Create mock ApiService for functions that accept it as parameter
    mockApiService = {
      setLike: vi.fn(),
      getCommentById: vi.fn(),
      getCommentReplyById: vi.fn(),
      createReply: vi.fn(),
      deleteComment: vi.fn(),
      deleteReply: vi.fn(),
      editReply: vi.fn(),
    };
  });

  afterEach(() => {
    consoleErrorSpy.mockRestore();
    vi.restoreAllMocks();
  });

  //////////////////// submitLike ////////////////////
  describe("submitLike", () => {
    beforeEach(() => {
      document.body.innerHTML = `
        <div class="like-icon" data-id="comment-123"></div>
        <span class="like-count">5</span>
      `;
    });

    it("should submit like and update count", async () => {
      const likeIcon = document.querySelector(".like-icon")!;
      const likeCountSpan = document.querySelector(".like-count") as HTMLSpanElement;

      (globalThis.fetch as any).mockResolvedValue(
        mockFetchResponse(200, {
          success: true,
          data: { like: 6 },
        })
      );

      await submitLike(likeIcon, likeCountSpan);

      expect(likeCountSpan.innerText).toBe("6");
    });

    it("should handle missing commentId", async () => {
      const likeIcon = document.createElement("div");
      const likeCountSpan = document.querySelector(".like-count") as HTMLSpanElement;

      await submitLike(likeIcon, likeCountSpan);

      expect(consoleErrorSpy).toHaveBeenCalledWith("Error: commentId not found");
    });

    it("should handle unsuccessful response", async () => {
      const likeIcon = document.querySelector(".like-icon")!;
      const likeCountSpan = document.querySelector(".like-count") as HTMLSpanElement;

      (globalThis.fetch as any).mockResolvedValue(
        mockFetchResponse(200, {
          success: false,
        })
      );

      await submitLike(likeIcon, likeCountSpan);

      expect(consoleErrorSpy).toHaveBeenCalledWith("Error: la respuesta no es un Comment válido");
    });
  });

  //////////////////// processComments ////////////////////
  describe("processComments", () => {
    it("should normalize timestamps and sort comments by date", () => {
      const comments: Comment[] = [
        {
          id: "1",
          content: "First",
          timestamp: "2025-01-15T10:00:00Z",
          like: 5,
        } as Comment,
        {
          id: "2",
          content: "Second",
          timestamp: "2025-01-10T10:00:00Z",
          like: 3,
        } as Comment,
      ];

      const result = processComments(comments);

      expect(result.comments).toHaveLength(2);
      expect(result.comments[0].id).toBe("2"); // Más antiguo primero
      expect(result.comments[1].id).toBe("1");
      expect(result.comments[0].timestamp).toMatch(/\/01\/2025/); // Formato español
    });

    it("should return top 3 best comments by likes", () => {
      const comments: Comment[] = [
        { id: "1", like: 10, timestamp: "2025-01-01T10:00:00Z" } as Comment,
        { id: "2", like: 5, timestamp: "2025-01-02T10:00:00Z" } as Comment,
        { id: "3", like: 15, timestamp: "2025-01-03T10:00:00Z" } as Comment,
        { id: "4", like: 8, timestamp: "2025-01-04T10:00:00Z" } as Comment,
      ];

      const result = processComments(comments);

      expect(result.bestComments).toHaveLength(3);
      expect(result.bestComments[0].id).toBe("3"); // 15 likes
      expect(result.bestComments[1].id).toBe("1"); // 10 likes
      expect(result.bestComments[2].id).toBe("4"); // 8 likes
    });

    it("should handle comments with null likes", () => {
      const comments: Comment[] = [
        { id: "1", like: 10, timestamp: "2025-01-01T10:00:00Z" } as Comment,
        { id: "2", timestamp: "2025-01-02T10:00:00Z" } as Comment,
        { id: "3", like: 5, timestamp: "2025-01-03T10:00:00Z" } as Comment,
      ];

      const result = processComments(comments);

      expect(result.bestComments[0].id).toBe("1");
      expect(result.bestComments[1].id).toBe("3");
      expect(result.bestComments[2].id).toBe("2");
    });

    it("should preserve already normalized timestamps", () => {
      const comments: Comment[] = [
        {
          id: "1",
          content: "Test",
          timestamp: "15/01/2025 10:30",
          like: 0,
        } as Comment,
      ];

      const result = processComments(comments);

      expect(result.comments[0].timestamp).toBe("15/01/2025 10:30");
    });
  });

  //////////////////// updateLikeState ////////////////////
  describe("updateLikeState", () => {
    beforeEach(() => {
      document.body.innerHTML = `
        <div class="like-icon">
          <svg class="like-svg"></svg>
        </div>
      `;
    });

    it("should add 'liked' class when user has liked", () => {
      const likeIcon = document.querySelector(".like-icon")!;
      const user = { uid: "user-123" } as any;
      const comment = "user-123,user-456";

      updateLikeState(user, likeIcon, comment);

      expect(likeIcon.classList.contains("liked")).toBe(true);
      const svg = likeIcon.querySelector(".like-svg");
      expect(svg?.classList.contains("liked")).toBe(true);
    });

    it("should remove 'liked' class when user has not liked", () => {
      const likeIcon = document.querySelector(".like-icon")!;
      likeIcon.classList.add("liked");
      const user = { uid: "user-789" } as any;
      const comment = "user-123,user-456";

      updateLikeState(user, likeIcon, comment);

      expect(likeIcon.classList.contains("liked")).toBe(false);
    });

    it("should handle null user", () => {
      const likeIcon = document.querySelector(".like-icon")!;
      likeIcon.classList.add("liked");

      updateLikeState(null, likeIcon, "user-123");

      expect(likeIcon.classList.contains("liked")).toBe(false);
    });

    it("should handle null likeIcon", () => {
      const user = { uid: "user-123" } as any;

      expect(() => updateLikeState(user, null, "user-123")).not.toThrow();
    });

    it("should handle null comment", () => {
      const likeIcon = document.querySelector(".like-icon")!;
      const user = { uid: "user-123" } as any;

      expect(() => updateLikeState(user, likeIcon, null)).not.toThrow();
    });
  });

  //////////////////// verifyAuthorInComment ////////////////////
  describe("verifyAuthorInComment", () => {
    it("should verify author successfully", async () => {
      mockApiService.getCommentById.mockResolvedValue({
        success: true,
        data: { author_uid: "user-123" },
      });

      await verifyAuthorInComment(mockApiService, "comment-123");

      expect(mockApiService.getCommentById).toHaveBeenCalledWith("comment-123");
      expect(toast.error).not.toHaveBeenCalled();
    });

    it("should show error when request fails", async () => {
      mockApiService.getCommentById.mockResolvedValue({
        success: false,
      });

      await expect(verifyAuthorInComment(mockApiService, "comment-123")).rejects.toThrow();
    });

    it("should show error when user is not the author", async () => {
      mockApiService.getCommentById.mockResolvedValue({
        success: true,
        data: { author_uid: "other-user" },
      });

      await expect(verifyAuthorInComment(mockApiService, "comment-123")).rejects.toThrow();
    });

    it("should handle exceptions", async () => {
      mockApiService.getCommentById.mockRejectedValue(new Error("Network error"));

      await expect(verifyAuthorInComment(mockApiService, "comment-123")).rejects.toThrow("Network error");
    });
  });

  //////////////////// verifyAuthorInCommentReply ////////////////////
  describe("verifyAuthorInCommentReply", () => {
    it("should verify reply author successfully", async () => {
      mockApiService.getCommentReplyById.mockResolvedValue({
        success: true,
        data: { author_uid: "user-123" },
      });

      await verifyAuthorInCommentReply(mockApiService, "comment-123", "reply-456");

      expect(mockApiService.getCommentReplyById).toHaveBeenCalledWith("comment-123", "reply-456");
      expect(toast.error).not.toHaveBeenCalled();
    });

    it("should show error when user is not the reply author", async () => {
      mockApiService.getCommentReplyById.mockResolvedValue({
        success: true,
        data: { author_uid: "other-user" },
      });

      await expect(verifyAuthorInCommentReply(mockApiService, "comment-123", "reply-456")).rejects.toThrow();
    });
  });

  //////////////////// handleSubmitReply ////////////////////
  describe("handleSubmitReply", () => {
    let mockLocation: any;

    beforeEach(() => {
      document.body.innerHTML = `
        <div data-comment-id="comment-123">
          <div class="reply-form">
            <textarea class="reply-textarea">This is a reply</textarea>
          </div>
        </div>
      `;

      mockLocation = { reload: vi.fn() };
      Object.defineProperty(globalThis, "location", {
        value: mockLocation,
        writable: true,
        configurable: true,
      });

      vi.spyOn(globalThis, "setTimeout").mockImplementation((fn: any) => {
        fn();
        return 0 as any;
      });
    });

    it("should submit reply successfully", async () => {
      mockApiService.createReply.mockResolvedValue({
        success: true,
        data: { id: "reply-123" },
      });

      const commentEl = document.querySelector("div[data-comment-id]") as HTMLElement;

      await handleSubmitReply(mockApiService, commentEl, {});

      expect(mockApiService.createReply).toHaveBeenCalledWith(
        "comment-123",
        expect.objectContaining({
          content: "This is a reply",
          author_uid: "user-123",
          name: "Test User",
          url_img: "https://example.com/photo.jpg",
        })
      );
      expect(toast.success).toHaveBeenCalledWith("Respuesta añadida correctamente");
      expect(mockLocation.reload).toHaveBeenCalled();
    });

    it("should show error when reply creation fails", async () => {
      mockApiService.createReply.mockResolvedValue({
        success: false,
        error: "Error creating reply",
      });

      const commentEl = document.querySelector("div[data-comment-id]") as HTMLElement;

      await handleSubmitReply(mockApiService, commentEl, {});

      expect(toast.error).toHaveBeenCalled();
      expect(mockLocation.reload).not.toHaveBeenCalled();
    });

    it("should show error when user is not authenticated", async () => {
      vi.mocked(getFirebaseAuth).mockReturnValue({
        currentUser: null,
      } as any);

      const commentEl = document.querySelector("div[data-comment-id]") as HTMLElement;

      await handleSubmitReply(mockApiService, commentEl, {});

      expect(toast.error).toHaveBeenCalled();
      expect(mockApiService.createReply).not.toHaveBeenCalled();
    });

    it("should show error when textarea is empty", async () => {
      const textarea = document.querySelector(".reply-textarea") as HTMLTextAreaElement;
      textarea.value = "   ";

      const commentEl = document.querySelector("div[data-comment-id]") as HTMLElement;

      await handleSubmitReply(mockApiService, commentEl, {});

      expect(toast.error).toHaveBeenCalled();
      expect(mockApiService.createReply).not.toHaveBeenCalled();
    });

    it("should handle missing comment div", async () => {
      const commentEl = document.createElement("div");

      await handleSubmitReply(mockApiService, commentEl, {});

      expect(toast.error).toHaveBeenCalled();
    });
  });

  //////////////////// handleDeleteComment ////////////////////
  describe("handleDeleteComment", () => {
    it("should delete comment successfully", async () => {
      mockApiService.getCommentById.mockResolvedValue({
        success: true,
        data: { author_uid: "user-123" },
      });
      mockApiService.deleteComment.mockResolvedValue({
        success: true,
      });

      const result = await handleDeleteComment(mockApiService, "comment-123");

      expect(mockApiService.deleteComment).toHaveBeenCalledWith("comment-123");
      expect(result.success).toBe(true);
    });

    it("should throw error when deletion fails", async () => {
      mockApiService.getCommentById.mockResolvedValue({
        success: true,
        data: { author_uid: "user-123" },
      });
      mockApiService.deleteComment.mockResolvedValue({
        success: false,
        error: "Comment not found",
      });

      await expect(handleDeleteComment(mockApiService, "comment-123")).rejects.toThrow();
    });

    it("should throw error when user is not authorized", async () => {
      mockApiService.getCommentById.mockResolvedValue({
        success: true,
        data: { author_uid: "other-user" },
      });

      await expect(handleDeleteComment(mockApiService, "comment-123")).rejects.toThrow();
    });
  });

  //////////////////// handleEditComment ////////////////////
  describe("handleEditComment", () => {
    it("should verify author for editing", async () => {
      mockApiService.getCommentById.mockResolvedValue({
        success: true,
        data: { author_uid: "user-123" },
      });

      const result = await handleEditComment(mockApiService, "comment-123");

      expect(mockApiService.getCommentById).toHaveBeenCalledWith("comment-123");
      expect(result).toBeUndefined();
    });

    it("should throw error when user is not author", async () => {
      mockApiService.getCommentById.mockResolvedValue({
        success: true,
        data: { author_uid: "other-user" },
      });

      await expect(handleEditComment(mockApiService, "comment-123")).rejects.toThrow();
    });
  });

  //////////////////// handleDeleteReply ////////////////////
  describe("handleDeleteReply", () => {
    it("should delete reply successfully", async () => {
      mockApiService.getCommentReplyById.mockResolvedValue({
        success: true,
        data: { author_uid: "user-123" },
      });
      mockApiService.deleteReply.mockResolvedValue({
        success: true,
      });

      const result = await handleDeleteReply(mockApiService, "comment-123", "reply-456");

      expect(mockApiService.deleteReply).toHaveBeenCalledWith("comment-123", "reply-456");
      expect(result.success).toBe(true);
    });

    it("should throw error when deletion fails", async () => {
      mockApiService.getCommentReplyById.mockResolvedValue({
        success: true,
        data: { author_uid: "user-123" },
      });
      mockApiService.deleteReply.mockResolvedValue({
        success: false,
        error: "Reply not found",
      });

      await expect(handleDeleteReply(mockApiService, "comment-123", "reply-456")).rejects.toThrow();
    });
  });

  //////////////////// handleEditReply ////////////////////
  describe("handleEditReply", () => {
    it("should edit reply successfully", async () => {
      mockApiService.editReply.mockResolvedValue({
        success: true,
        data: { id: "reply-456", content: "Updated content" },
      });

      const result = await handleEditReply(
        mockApiService,
        "comment-123",
        "reply-456",
        "Updated content",
        "Old content"
      );

      expect(mockApiService.editReply).toHaveBeenCalledWith(
        "comment-123",
        "reply-456",
        expect.objectContaining({
          id: "reply-456",
          content: "Updated content",
          author_uid: "user-123",
        })
      );
      expect(result?.success).toBe(true);
    });

    it("should return null when content is empty", async () => {
      const result = await handleEditReply(mockApiService, "comment-123", "reply-456", "", "Old content");

      expect(result).toBeNull();
      expect(mockApiService.editReply).not.toHaveBeenCalled();
    });

    it("should return null when content hasn't changed", async () => {
      const result = await handleEditReply(mockApiService, "comment-123", "reply-456", "Same content", "Same content");

      expect(result).toBeNull();
      expect(mockApiService.editReply).not.toHaveBeenCalled();
    });

    it("should throw error when user is not authenticated", async () => {
      vi.mocked(getFirebaseAuth).mockReturnValue({
        currentUser: null,
      } as any);

      await expect(
        handleEditReply(mockApiService, "comment-123", "reply-456", "New content", "Old content")
      ).rejects.toThrow();
    });

    it("should throw error when edit fails", async () => {
      mockApiService.editReply.mockResolvedValue({
        success: false,
        error: "Failed to update reply",
      });

      await expect(
        handleEditReply(mockApiService, "comment-123", "reply-456", "New content", "Old content")
      ).rejects.toThrow();
    });
  });
});
