export { auth as middleware } from "@/auth"

export const config = {
	matcher: [
		/*
		 * Match all request paths except for the ones starting with:
		 * - api/auth (auth API routes)
		 * - auth/signin (signin page)
		 * - _next/static (static files)
		 * - _next/image (image optimization files)
		 * - favicon.ico (favicon file)
		 * - / (landing page - allow public access)
		 */
		"/((?!api/auth|auth/signin|_next/static|_next/image|favicon.ico|$).*)",
	],
}