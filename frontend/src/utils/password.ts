import { API_URL } from "@/lib/config";

// Note: This is a simple implementation for demo purposes
// In production, password hashing should be done on the server side
export function saltAndHashPassword(password: string): string {
	// This is a placeholder - in a real app, you'd use a proper hashing library
	// For this demo, we'll just return the password as-is since we'll validate on the backend
	return password;
}

interface User {
	id: string;
	email: string;
	name: string | null;
	accessToken: string;
	refreshToken: string;
	expiresAt: number;
}

/**
 * OAuth2 Login Response from backend
 */
interface LoginResponse {
	access_token: string;
	token_type: string;
	expires_in: number;
	refresh_token: string;
	user_id: number;
	email: string;
	name: string | null;
}

export async function getUserFromDb(email: string, passwordHash: string): Promise<User | null> {
	try {
		// Call the backend login endpoint
		const response = await fetch(`${API_URL}/auth/login`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify({ email, password: passwordHash }),
		});

		if (!response.ok) {
			return null;
		}

		const data: LoginResponse = await response.json();

		// Calculate token expiration time (current time + expires_in seconds)
		const expiresAt = Math.floor(Date.now() / 1000) + data.expires_in;

		// Return user object in the format expected by NextAuth
		return {
			id: data.user_id.toString(),
			email: data.email,
			name: data.name,
			accessToken: data.access_token,
			refreshToken: data.refresh_token,
			expiresAt,
		};
	} catch (error) {
		console.error('Login error:', error);
		return null;
	}
}
