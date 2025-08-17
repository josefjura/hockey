import type { NextConfig } from "next";
import createNextIntlPlugin from 'next-intl/plugin';

const nextConfig: NextConfig = {
	/* config options here */
	images: {
		remotePatterns: [new URL("https://flagcdn.com/**")],
	},
	output: 'standalone',
};

const withNextIntl = createNextIntlPlugin();
export default withNextIntl(nextConfig);
