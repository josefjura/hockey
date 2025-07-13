import { DefaultSession, DefaultUser } from "next-auth"
import { DefaultJWT } from "next-auth/jwt"
import { User as HockeyUser } from "./auth"

declare module "next-auth" {
	interface User extends DefaultUser {
		id: string
		email: string
		name?: string | null
		token?: string
	}

	interface Session extends DefaultSession {
		accessToken?: string
		user: HockeyUser & DefaultSession["user"]
	}
}

declare module "next-auth/jwt" {
	interface JWT extends DefaultJWT {
		accessToken?: string
		id?: string
	}
}