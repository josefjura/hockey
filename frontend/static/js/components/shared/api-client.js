/**
 * API client utilities for web components
 */
/**
 * Fetch wrapper with error handling
 * @param url - API endpoint URL
 * @param options - Fetch options
 * @returns Promise with typed response or error
 */
export async function apiClient(url, options) {
    try {
        const response = await fetch(url, {
            ...options,
            headers: {
                'Content-Type': 'application/json',
                ...options?.headers,
            },
        });
        if (!response.ok) {
            const errorText = await response.text();
            return {
                error: {
                    message: errorText || `HTTP error ${response.status}`,
                    status: response.status,
                },
            };
        }
        const data = await response.json();
        return { data };
    }
    catch (error) {
        return {
            error: {
                message: error instanceof Error ? error.message : 'Network error',
            },
        };
    }
}
/**
 * GET request helper
 */
export async function get(url) {
    return apiClient(url, { method: 'GET' });
}
/**
 * POST request helper
 */
export async function post(url, body) {
    return apiClient(url, {
        method: 'POST',
        body: body ? JSON.stringify(body) : undefined,
    });
}
/**
 * PUT request helper
 */
export async function put(url, body) {
    return apiClient(url, {
        method: 'PUT',
        body: body ? JSON.stringify(body) : undefined,
    });
}
/**
 * DELETE request helper
 */
export async function del(url) {
    return apiClient(url, { method: 'DELETE' });
}
/**
 * Build query string from object
 */
export function buildQueryString(params) {
    const filtered = Object.entries(params)
        .filter(([_, value]) => value !== undefined && value !== null && value !== '')
        .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value)}`);
    return filtered.length > 0 ? `?${filtered.join('&')}` : '';
}
//# sourceMappingURL=api-client.js.map