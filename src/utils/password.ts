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
	token: string;
}

export async function getUserFromDb(email: string, passwordHash: string): Promise<User | null> {
	try {
		const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:8080';
		// Call the backend login endpoint
		const response = await fetch(`${backendUrl}/auth/login`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify({ email, password: passwordHash }),
		});

		if (!response.ok) {
			return null;
		}

		const data = await response.json();

		// Return user object in the format expected by NextAuth
		return {
			id: data.user_id.toString(),
			email: data.email,
			name: data.name,
			token: data.token,
		};
	} catch (error) {
		console.error('Login error:', error);
		return null;
	}
}
