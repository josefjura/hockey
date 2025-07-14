/**
 * Get flag URL for any country using ISO code
 * @param {string} isoCode - ISO2 country code (e.g., 'CA
 * N', 'DE', 'RU')
 * @param {string} iocCode - IOC country code (e.g., 'CAN
 * ', 'GER', 'URS')
 * @param {boolean} isHistorical - Whether the country is historical
 * @param {string} size - Size variant ('w20', 'w40', '
 * w80', etc.)
 * @returns {string} Flag URL
 */
export function getCountryFlag(isoCode: string, iocCode: string, isHistorical: boolean, size = 'w40'): string {
	// For historical countries, return local SVG path
	if (isHistorical) {
		return `/flags/${isoCode.toLowerCase()}.svg`;
	}

	// For current countries, use flagcdn.com with ISO2 code
	if (isoCode) {
		return `https://flagcdn.com/${size}/${isoCode.toLowerCase()}.png`;
	}

	// Fallback
	return '/flags/unknown.svg';
}
