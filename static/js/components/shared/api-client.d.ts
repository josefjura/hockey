/**
 * API client utilities for web components
 */
import { ApiResponse } from './types.js';
/**
 * Fetch wrapper with error handling
 * @param url - API endpoint URL
 * @param options - Fetch options
 * @returns Promise with typed response or error
 */
export declare function apiClient<T>(url: string, options?: RequestInit): Promise<ApiResponse<T>>;
/**
 * GET request helper
 */
export declare function get<T>(url: string): Promise<ApiResponse<T>>;
/**
 * POST request helper
 */
export declare function post<T>(url: string, body?: any): Promise<ApiResponse<T>>;
/**
 * PUT request helper
 */
export declare function put<T>(url: string, body?: any): Promise<ApiResponse<T>>;
/**
 * DELETE request helper
 */
export declare function del<T>(url: string): Promise<ApiResponse<T>>;
/**
 * Build query string from object
 */
export declare function buildQueryString(params: Record<string, any>): string;
//# sourceMappingURL=api-client.d.ts.map