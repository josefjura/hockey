/**
 * Application configuration constants
 */

export const API_CONFIG = {
  BASE_URL: process.env.HOCKEY_BACKEND_URL || 'http://localhost:9080',
} as const;

export const API_URL = API_CONFIG.BASE_URL;