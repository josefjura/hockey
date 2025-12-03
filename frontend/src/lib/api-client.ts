import { getApiUrl } from './config';
import { auth } from '@/auth';
import { redirect } from 'next/navigation';

/**
 * Configuration options for API requests
 */
export interface ApiRequestOptions extends RequestInit {
  /** Whether to include authentication token (default: true) */
  requireAuth?: boolean;
  /** Whether to handle 401 errors automatically (default: true) */
  handle401?: boolean;
}

/**
 * Error thrown when API request fails
 */
export class ApiError extends Error {
  constructor(
    message: string,
    public status: number,
    public statusText: string,
    public url: string
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

/**
 * Authenticated API client that automatically injects JWT tokens
 * and handles authentication errors.
 *
 * This function should be used for server-side API calls (Server Components, Server Actions).
 * For client-side calls, use `createClientApiClient()` instead.
 *
 * @param endpoint - API endpoint path (e.g., '/team' or '/team/123')
 * @param options - Request options (method, body, headers, etc.)
 * @returns Promise resolving to the response
 *
 * @example
 * // GET request
 * const teams = await apiClient('/team');
 *
 * @example
 * // POST request
 * const newTeam = await apiClient('/team', {
 *   method: 'POST',
 *   body: JSON.stringify({ name: 'Team A', country_id: 1 }),
 * });
 *
 * @example
 * // Unauthenticated request
 * const publicData = await apiClient('/public/data', { requireAuth: false });
 */
export async function apiClient<T = unknown>(
  endpoint: string,
  options: ApiRequestOptions = {}
): Promise<T> {
  const {
    requireAuth = true,
    handle401 = true,
    headers = {},
    ...fetchOptions
  } = options;

  const url = `${getApiUrl()}${endpoint}`;
  const requestHeaders: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(headers as Record<string, string>),
  };

  // Add JWT token if authentication is required
  if (requireAuth) {
    const session = await auth();

    if (!session?.accessToken) {
      console.error('No access token found in session');
      if (handle401) {
        redirect('/auth/signin');
      }
      throw new ApiError(
        'Authentication required',
        401,
        'Unauthorized',
        url
      );
    }

    requestHeaders['Authorization'] = `Bearer ${session.accessToken}`;
  }

  try {
    const response = await fetch(url, {
      ...fetchOptions,
      headers: requestHeaders,
    });

    // Handle 401 Unauthorized
    if (response.status === 401 && handle401) {
      console.error('API returned 401 - redirecting to login');
      redirect('/auth/signin');
    }

    // Handle other errors
    if (!response.ok) {
      const errorMessage = await response.text().catch(() => response.statusText);
      throw new ApiError(
        errorMessage || 'API request failed',
        response.status,
        response.statusText,
        url
      );
    }

    // Parse JSON response
    const data = await response.json();
    return data as T;
  } catch (error) {
    if (error instanceof ApiError) {
      throw error;
    }

    // Handle network errors
    if (error instanceof TypeError && error.message.includes('fetch')) {
      throw new ApiError(
        'Network error - could not reach API server',
        0,
        'Network Error',
        url
      );
    }

    // Re-throw other errors
    throw error;
  }
}

/**
 * Creates a client-side API client with a specific access token.
 * This is meant to be used in Client Components where we can't use the auth() function.
 *
 * @param accessToken - JWT access token from the session
 * @returns API client function
 *
 * @example
 * // In a Client Component
 * 'use client';
 * import { useSession } from 'next-auth/react';
 *
 * function MyComponent() {
 *   const { data: session } = useSession();
 *   const client = createClientApiClient(session?.accessToken);
 *
 *   const handleClick = async () => {
 *     const data = await client('/team');
 *   };
 * }
 */
export function createClientApiClient(accessToken?: string) {
  return async function clientApiClient<T = unknown>(
    endpoint: string,
    options: Omit<ApiRequestOptions, 'requireAuth'> & { requireAuth?: boolean } = {}
  ): Promise<T> {
    const {
      requireAuth = true,
      handle401 = true,
      headers = {},
      ...fetchOptions
    } = options;

    const url = `${getApiUrl()}${endpoint}`;
    const requestHeaders: Record<string, string> = {
      'Content-Type': 'application/json',
      ...(headers as Record<string, string>),
    };

    // Add JWT token if authentication is required
    if (requireAuth) {
      if (!accessToken) {
        console.error('No access token provided to client API client');
        if (handle401 && typeof window !== 'undefined') {
          window.location.href = '/auth/signin';
        }
        throw new ApiError(
          'Authentication required',
          401,
          'Unauthorized',
          url
        );
      }

      requestHeaders['Authorization'] = `Bearer ${accessToken}`;
    }

    try {
      const response = await fetch(url, {
        ...fetchOptions,
        headers: requestHeaders,
      });

      // Handle 401 Unauthorized
      if (response.status === 401 && handle401) {
        console.error('API returned 401 - redirecting to login');
        if (typeof window !== 'undefined') {
          window.location.href = '/auth/signin';
        }
      }

      // Handle other errors
      if (!response.ok) {
        const errorMessage = await response.text().catch(() => response.statusText);
        throw new ApiError(
          errorMessage || 'API request failed',
          response.status,
          response.statusText,
          url
        );
      }

      // Parse JSON response
      const data = await response.json();
      return data as T;
    } catch (error) {
      if (error instanceof ApiError) {
        throw error;
      }

      // Handle network errors
      if (error instanceof TypeError && error.message.includes('fetch')) {
        throw new ApiError(
          'Network error - could not reach API server',
          0,
          'Network Error',
          url
        );
      }

      // Re-throw other errors
      throw error;
    }
  };
}

/**
 * Type-safe API client for GET requests
 */
export async function apiGet<T = unknown>(
  endpoint: string,
  options?: Omit<ApiRequestOptions, 'method' | 'body'>
): Promise<T> {
  return apiClient<T>(endpoint, { ...options, method: 'GET' });
}

/**
 * Type-safe API client for POST requests
 */
export async function apiPost<T = unknown>(
  endpoint: string,
  body?: unknown,
  options?: Omit<ApiRequestOptions, 'method' | 'body'>
): Promise<T> {
  return apiClient<T>(endpoint, {
    ...options,
    method: 'POST',
    body: body ? JSON.stringify(body) : undefined,
  });
}

/**
 * Type-safe API client for PUT requests
 */
export async function apiPut<T = unknown>(
  endpoint: string,
  body?: unknown,
  options?: Omit<ApiRequestOptions, 'method' | 'body'>
): Promise<T> {
  return apiClient<T>(endpoint, {
    ...options,
    method: 'PUT',
    body: body ? JSON.stringify(body) : undefined,
  });
}

/**
 * Type-safe API client for DELETE requests
 */
export async function apiDelete<T = unknown>(
  endpoint: string,
  options?: Omit<ApiRequestOptions, 'method' | 'body'>
): Promise<T> {
  return apiClient<T>(endpoint, { ...options, method: 'DELETE' });
}
