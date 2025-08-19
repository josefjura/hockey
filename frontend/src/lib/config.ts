/**
 * Application configuration constants
 */

export const API_CONFIG = {
  // Server-side API URL (not available in browser)
  SERVER_URL: process.env.HOCKEY_BACKEND_URL || 'http://localhost:8080',
  // Client-side API URL (baked into bundle at build time)
  CLIENT_URL: process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080',
} as const;

/**
 * Helper to get the right API URL based on environment
 * @returns API URL for current environment (server-side or client-side)
 */
export function getApiUrl(): string {
  return typeof window === 'undefined' 
    ? API_CONFIG.SERVER_URL 
    : API_CONFIG.CLIENT_URL;
}

// Backwards compatibility
export const API_URL = getApiUrl();