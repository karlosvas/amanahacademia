// import { describe, it, expect, beforeEach, vi } from 'vitest';
// import { ResultUtils, ApiService } from '@/services/helper';
// import { ApiError } from '@/services/globalHandler';
// import { ApiErrorType } from '@/enums/enums';

// // Mock de getCurrentUserToken
// vi.mock('@/services/firebase', () => ({
//   getCurrentUserToken: vi.fn(() => Promise.resolve('mock-token')),
// }));

// describe('ResultUtils', () => {
//   describe('ok', () => {
//     it('should create a successful result with data', () => {
//       const data = { id: 1, name: 'Test' };
//       const result = ResultUtils.ok(data);

//       expect(result.success).toBe(true);
//       expect(result.data).toEqual(data);
//     });

//     it('should handle null data', () => {
//       const result = ResultUtils.ok(null);

//       expect(result.success).toBe(true);
//       expect(result.data).toBe(null);
//     });

//     it('should handle undefined data', () => {
//       const result = ResultUtils.ok(undefined);

//       expect(result.success).toBe(true);
//       expect(result.data).toBe(undefined);
//     });
//   });

//   describe('error', () => {
//     it('should create an error result', () => {
//       const error = new ApiError(ApiErrorType.SERVER_ERROR, 'Test error');
//       const result = ResultUtils.error(error);

//       expect(result.success).toBe(false);
//       expect(result.error).toBe(error);
//     });

//     it('should handle string errors', () => {
//       const result = ResultUtils.error('Simple error');

//       expect(result.success).toBe(false);
//       expect(result.error).toBe('Simple error');
//     });
//   });

//   describe('getErrorType', () => {
//     it('should return error type from ApiError', () => {
//       const error = new ApiError(ApiErrorType.AUTHENTICATION_ERROR, 'Auth failed');
//       const result = ResultUtils.error(error);

//       const errorType = ResultUtils.getErrorType(result);

//       expect(errorType).toBe(ApiErrorType.AUTHENTICATION_ERROR);
//     });

//     it('should return null for successful results', () => {
//       const result = ResultUtils.ok({ data: 'test' });

//       const errorType = ResultUtils.getErrorType(result);

//       expect(errorType).toBe(null);
//     });

//     it('should return null for non-ApiError errors', () => {
//       const result = ResultUtils.error('Simple error');

//       const errorType = ResultUtils.getErrorType(result as any);

//       expect(errorType).toBe(null);
//     });
//   });
// });

// describe('ApiService', () => {
//   let apiService: ApiService;
//   let mockFetch: any;

//   beforeEach(() => {
//     apiService = new ApiService();
//     mockFetch = vi.fn();
//     global.fetch = mockFetch;
//   });

//   describe('getAllComments', () => {
//     it('should fetch all comments successfully', async () => {
//       const mockComments = [
//         { id: '1', content: 'Test comment 1' },
//         { id: '2', content: 'Test comment 2' },
//       ];

//       mockFetch.mockResolvedValue({
//         ok: true,
//         status: 200,
//         json: async () => ({ success: true, data: mockComments }),
//       });

//       const result = await apiService.getAllComments();

//       expect(result.success).toBe(true);
//       if (result.success) {
//         expect(result.data).toEqual(mockComments);
//       }
//       expect(mockFetch).toHaveBeenCalledWith(
//         expect.stringContaining('/comments/all'),
//         expect.objectContaining({
//           method: 'GET',
//           headers: { 'Content-Type': 'application/json' },
//         })
//       );
//     });

//     it('should handle network errors', async () => {
//       mockFetch.mockRejectedValue(new Error('Network error'));

//       const result = await apiService.getAllComments();

//       expect(result.success).toBe(false);
//       if (!result.success) {
//         expect(result.error).toBeInstanceOf(ApiError);
//         expect(result.error.type).toBe(ApiErrorType.NETWORK_ERROR);
//       }
//     });
//   });

//   describe('getTeachers', () => {
//     it('should fetch all teachers successfully', async () => {
//       const mockTeachers = [
//         { id: '1', name: 'Teacher 1' },
//         { id: '2', name: 'Teacher 2' },
//       ];

//       mockFetch.mockResolvedValue({
//         ok: true,
//         status: 200,
//         json: async () => ({ success: true, data: mockTeachers }),
//       });

//       const result = await apiService.getTeachers();

//       expect(result.success).toBe(true);
//       if (result.success) {
//         expect(result.data).toEqual(mockTeachers);
//       }
//     });

//     it('should handle 404 errors', async () => {
//       mockFetch.mockResolvedValue({
//         ok: false,
//         status: 404,
//       });

//       const result = await apiService.getTeachers();

//       expect(result.success).toBe(false);
//       if (!result.success) {
//         expect(result.error.type).toBe(ApiErrorType.SESSION_NOT_FOUND);
//       }
//     });
//   });

//   describe('registerUser', () => {
//     it('should register user successfully', async () => {
//       const userRequest = {
//         name: 'Test User',
//         email: 'test@example.com',
//         password: 'password123',
//         provider: 'email' as const,
//         first_free_class: false,
//       };

//       mockFetch.mockResolvedValue({
//         ok: true,
//         status: 200,
//         json: async () => ({ success: true, data: 'User created' }),
//       });

//       const result = await apiService.registerUser(userRequest);

//       expect(result.success).toBe(true);
//       if (result.success) {
//         expect(result.data).toBe('User created');
//       }
//       expect(mockFetch).toHaveBeenCalledWith(
//         expect.stringContaining('/users/register'),
//         expect.objectContaining({
//           method: 'POST',
//           body: JSON.stringify(userRequest),
//         })
//       );
//     });

//     it('should handle validation errors', async () => {
//       mockFetch.mockResolvedValue({
//         ok: false,
//         status: 422,
//       });

//       const userRequest = {
//         name: '',
//         email: 'invalid-email',
//         password: '123',
//         provider: 'email' as const,
//         first_free_class: false,
//       };

//       const result = await apiService.registerUser(userRequest);

//       expect(result.success).toBe(false);
//       if (!result.success) {
//         expect(result.error.type).toBe(ApiErrorType.VALIDATION_ERROR);
//       }
//     });
//   });

//   describe('sendContact', () => {
//     it('should send contact email successfully', async () => {
//       const emailData = {
//         email: 'test@example.com',
//         message: 'Test message',
//         name: 'Test User',
//       };

//       mockFetch.mockResolvedValue({
//         ok: true,
//         status: 200,
//         json: async () => ({ success: true, data: { id: '123' } }),
//       });

//       const result = await apiService.sendContact(emailData);

//       expect(result.success).toBe(true);
//       expect(mockFetch).toHaveBeenCalledWith(
//         expect.stringContaining('/email/contact'),
//         expect.objectContaining({
//           method: 'POST',
//           body: JSON.stringify(emailData),
//         })
//       );
//     });
//   });

//   describe('HTTP Error Handling', () => {
//     it('should handle 401 authentication errors', async () => {
//       mockFetch.mockResolvedValue({
//         ok: false,
//         status: 401,
//       });

//       const result = await apiService.getAllComments();

//       expect(result.success).toBe(false);
//       if (!result.success) {
//         expect(result.error.type).toBe(ApiErrorType.AUTHENTICATION_ERROR);
//       }
//     });

//     it('should handle 500 server errors', async () => {
//       mockFetch.mockResolvedValue({
//         ok: false,
//         status: 500,
//       });

//       const result = await apiService.getAllComments();

//       expect(result.success).toBe(false);
//       if (!result.success) {
//         expect(result.error.type).toBe(ApiErrorType.SERVER_ERROR);
//       }
//     });

//     it('should handle 204 No Content responses', async () => {
//       mockFetch.mockResolvedValue({
//         ok: true,
//         status: 204,
//       });

//       const result = await apiService.deleteComment('test-id');

//       expect(result.success).toBe(true);
//     });
//   });
// });
