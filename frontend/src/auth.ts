import NextAuth from "next-auth"
import Credentials from "next-auth/providers/credentials"
import { getUserFromDb } from "@/utils/password"
import type { NextAuthConfig } from "next-auth"
import { getApiUrl } from "@/lib/config"

/**
 * OAuth2 Refresh Token Response from backend
 */
interface RefreshResponse {
	access_token: string
	token_type: string
	expires_in: number
	refresh_token: string
}

/**
 * Refresh the access token using the refresh token
 */
async function refreshAccessToken(refreshToken: string) {
	try {
		const response = await fetch(`${getApiUrl()}/auth/refresh`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ refresh_token: refreshToken }),
		})

		if (!response.ok) {
			throw new Error("Failed to refresh token")
		}

		const data: RefreshResponse = await response.json()

		// Calculate new expiration time
		const expiresAt = Math.floor(Date.now() / 1000) + data.expires_in

		return {
			accessToken: data.access_token,
			refreshToken: data.refresh_token,
			expiresAt,
		}
	} catch (error) {
		console.error("Error refreshing access token:", error)
		throw error
	}
}

export const config = {
	secret: process.env.NEXTAUTH_SECRET,
	trustHost: true,
	providers: [
		Credentials({
			credentials: {
				email: { label: "Email", type: "email" },
				password: { label: "Password", type: "password" },
			},
			authorize: async (credentials) => {
				if (!credentials?.email || !credentials?.password) {
					return null
				}

				const user = await getUserFromDb(
					credentials.email as string,
					credentials.password as string
				)

				if (!user) {
					return null
				}

				return {
					id: user.id,
					email: user.email,
					name: user.name || null,
					accessToken: user.accessToken,
					refreshToken: user.refreshToken,
					expiresAt: user.expiresAt,
				}
			},
		}),
	],
	pages: {
		signIn: "/auth/signin",
	},
	callbacks: {
		async jwt({ token, user }) {
			// Initial sign in - store tokens from user object
			if (user) {
				return {
					...token,
					accessToken: user.accessToken,
					refreshToken: user.refreshToken,
					expiresAt: user.expiresAt,
					id: user.id,
				}
			}

			// Return previous token if the access token has not expired yet
			const now = Math.floor(Date.now() / 1000)
			if (token.expiresAt && now < token.expiresAt) {
				return token
			}

			// Access token has expired, try to refresh it
			if (!token.refreshToken) {
				console.error("No refresh token available")
				return {
					...token,
					error: "RefreshAccessTokenError" as const,
				}
			}

			try {
				const refreshed = await refreshAccessToken(token.refreshToken as string)
				return {
					...token,
					accessToken: refreshed.accessToken,
					refreshToken: refreshed.refreshToken,
					expiresAt: refreshed.expiresAt,
					error: undefined,
				}
			} catch (error) {
				console.error("Error refreshing access token:", error)
				return {
					...token,
					error: "RefreshAccessTokenError" as const,
				}
			}
		},
		async session({ session, token }) {
			// Add access token to session
			if (token.accessToken) {
				session.accessToken = token.accessToken as string
			}

			// Add user ID to session
			if (token.id && session.user) {
				session.user.id = token.id as string
			}

			// Pass error to session for handling in the client
			if (token.error) {
				session.error = token.error
			}

			return session
		},
	},
} satisfies NextAuthConfig

export const { handlers, signIn, signOut, auth } = NextAuth(config)