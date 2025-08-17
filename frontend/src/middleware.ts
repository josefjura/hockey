import { auth } from "@/auth"
import createMiddleware from 'next-intl/middleware';
import { routing } from './i18n/routing';
import { NextRequest, NextResponse } from 'next/server';

const intlMiddleware = createMiddleware(routing);

export default async function middleware(request: NextRequest) {
	// Handle internationalization first
	const response = intlMiddleware(request);

	const { pathname } = request.nextUrl;

	// Extract locale from pathname
	const pathnameHasLocale = routing.locales.some(
		(locale) => pathname.startsWith(`/${locale}/`) || pathname === `/${locale}`
	);

	// If no locale in pathname, redirect to default locale
	if (!pathnameHasLocale && pathname !== '/') {
		const locale = routing.defaultLocale;
		return NextResponse.redirect(new URL(`/${locale}${pathname}`, request.url));
	}

	// Get current locale
	const locale = pathnameHasLocale
		? pathname.split('/')[1]
		: routing.defaultLocale;

	// Check if on auth pages or landing page
	const isOnAuthPage = pathname.includes('/auth');
	const isOnLandingPage = pathname === '/' || pathname === `/${locale}`;

	// Allow access to landing page and auth pages
	if (isOnLandingPage || isOnAuthPage) {
		return response;
	}

	// For protected routes, check authentication
	const authResult = await auth();
	const isLoggedIn = !!authResult?.user;

	if (!isLoggedIn) {
		const signInUrl = new URL(`/${locale}/auth/signin`, request.url);
		return NextResponse.redirect(signInUrl);
	}

	return response;
}

export const config = {
	matcher: [
		// Match all pathnames except for
		// - … if they start with `/api`, `/_next` or `/_vercel`
		// - … the ones containing a dot (e.g. `favicon.ico`)
		'/((?!api|_next|_vercel|.*\\..*).*)',
	],
}