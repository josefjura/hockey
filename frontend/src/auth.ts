import NextAuth from "next-auth"
import Credentials from "next-auth/providers/credentials"
import { getUserFromDb } from "@/utils/password"
import type { NextAuthConfig } from "next-auth"

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
					id: user.id.toString(),
					email: user.email,
					name: user.name || null,
					token: user.token,
				}
			},
		}),
	],
	pages: {
		signIn: "/auth/signin",
	},
	callbacks: {
		async jwt({ token, user }) {
			if (user) {
				token.accessToken = user.token
				token.id = user.id
			}
			return token
		},
		async session({ session, token }) {
			if (token.accessToken) {
				session.accessToken = token.accessToken
			}
			if (token.id && session.user) {
				session.user.id = token.id
			}
			return session
		},
	},
} satisfies NextAuthConfig

export const { handlers, signIn, signOut, auth } = NextAuth(config)