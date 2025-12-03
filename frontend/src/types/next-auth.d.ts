import { DefaultSession, DefaultUser } from "next-auth"
import { DefaultJWT } from "next-auth/jwt"
import { User as HockeyUser } from "./auth"

declare module "next-auth" {
	interface User extends DefaultUser {
		id: string
		email: string
		name?: string | null
		accessToken: string
		refreshToken: string
		expiresAt: number
	}

	interface Session extends DefaultSession {
		accessToken?: string
		error?: "RefreshAccessTokenError"
		user: HockeyUser & DefaultSession["user"]
	}
}

declare module "next-auth/jwt" {
	interface JWT extends DefaultJWT {
		accessToken?: string
		refreshToken?: string
		expiresAt?: number
		id?: string
		error?: "RefreshAccessTokenError"
	}
}