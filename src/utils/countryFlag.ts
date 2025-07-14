// Complete mapping from IOC codes to ISO 3166-1 alpha-2 codes for FlagCDN
// FlagCDN uses 2-letter ISO codes, not 3-letter codes

const IOC_TO_ISO_ALPHA2: Record<string, string> = {
	'AFG': 'AF', // Afghanistan
	'ALB': 'AL', // Albania
	'ALG': 'DZ', // Algeria
	'AND': 'AD', // Andorra
	'ANG': 'AO', // Angola
	'ANT': 'AG', // Antigua and Barbuda
	'ARG': 'AR', // Argentina
	'ARM': 'AM', // Armenia
	'ARU': 'AW', // Aruba
	'ASA': 'AS', // American Samoa
	'AUS': 'AU', // Australia
	'AUT': 'AT', // Austria
	'AZE': 'AZ', // Azerbaijan
	'BAH': 'BS', // Bahamas
	'BAN': 'BD', // Bangladesh
	'BAR': 'BB', // Barbados
	'BDI': 'BI', // Burundi
	'BEL': 'BE', // Belgium
	'BEN': 'BJ', // Benin
	'BER': 'BM', // Bermuda
	'BHU': 'BT', // Bhutan
	'BIH': 'BA', // Bosnia and Herzegovina
	'BIZ': 'BZ', // Belize
	'BLR': 'BY', // Belarus
	'BOL': 'BO', // Bolivia
	'BOT': 'BW', // Botswana
	'BRA': 'BR', // Brazil
	'BRN': 'BH', // Bahrain
	'BRU': 'BN', // Brunei
	'BUL': 'BG', // Bulgaria
	'BUR': 'BF', // Burkina Faso
	'CAF': 'CF', // Central African Republic
	'CAM': 'KH', // Cambodia
	'CAN': 'CA', // Canada
	'CAY': 'KY', // Cayman Islands
	'CGO': 'CG', // Republic of the Congo
	'CHA': 'TD', // Chad
	'CHI': 'CL', // Chile
	'CHN': 'CN', // China
	'CIV': 'CI', // Ivory Coast
	'CMR': 'CM', // Cameroon
	'COD': 'CD', // Democratic Republic of the Congo
	'COK': 'CK', // Cook Islands
	'COL': 'CO', // Colombia
	'COM': 'KM', // Comoros
	'CPV': 'CV', // Cape Verde
	'CRC': 'CR', // Costa Rica
	'CRO': 'HR', // Croatia
	'CUB': 'CU', // Cuba
	'CYP': 'CY', // Cyprus
	'CZE': 'CZ', // Czechia
	'DEN': 'DK', // Denmark
	'DJI': 'DJ', // Djibouti
	'DMA': 'DM', // Dominica
	'DOM': 'DO', // Dominican Republic
	'ECU': 'EC', // Ecuador
	'EGY': 'EG', // Egypt
	'ERI': 'ER', // Eritrea
	'ESA': 'SV', // El Salvador
	'ESP': 'ES', // Spain
	'EST': 'EE', // Estonia
	'ETH': 'ET', // Ethiopia
	'FIJ': 'FJ', // Fiji
	'FIN': 'FI', // Finland
	'FRA': 'FR', // France
	'FSM': 'FM', // Federated States of Micronesia
	'GAB': 'GA', // Gabon
	'GAM': 'GM', // The Gambia
	'GBR': 'GB', // Great Britain
	'GBS': 'GW', // Guinea-Bissau
	'GEO': 'GE', // Georgia
	'GEQ': 'GQ', // Equatorial Guinea
	'GER': 'DE', // Germany
	'GHA': 'GH', // Ghana
	'GRE': 'GR', // Greece
	'GRN': 'GD', // Grenada
	'GUA': 'GT', // Guatemala
	'GUI': 'GN', // Guinea
	'GUM': 'GU', // Guam
	'GUY': 'GY', // Guyana
	'HAI': 'HT', // Haiti
	'HKG': 'HK', // Hong Kong
	'HON': 'HN', // Honduras
	'HUN': 'HU', // Hungary
	'INA': 'ID', // Indonesia
	'IND': 'IN', // India
	'IRI': 'IR', // Iran
	'IRL': 'IE', // Ireland
	'IRQ': 'IQ', // Iraq
	'ISL': 'IS', // Iceland
	'ISR': 'IL', // Israel
	'ISV': 'VI', // Virgin Islands (US)
	'ITA': 'IT', // Italy
	'IVB': 'VG', // British Virgin Islands
	'JAM': 'JM', // Jamaica
	'JOR': 'JO', // Jordan
	'JPN': 'JP', // Japan
	'KAZ': 'KZ', // Kazakhstan
	'KEN': 'KE', // Kenya
	'KGZ': 'KG', // Kyrgyzstan
	'KIR': 'KI', // Kiribati
	'KOR': 'KR', // South Korea
	'KOS': 'XK', // Kosovo
	'KSA': 'SA', // Saudi Arabia
	'KUW': 'KW', // Kuwait
	'LAO': 'LA', // Laos
	'LAT': 'LV', // Latvia
	'LBA': 'LY', // Libya
	'LBN': 'LB', // Lebanon
	'LBR': 'LR', // Liberia
	'LCA': 'LC', // Saint Lucia
	'LES': 'LS', // Lesotho
	'LIE': 'LI', // Liechtenstein
	'LTU': 'LT', // Lithuania
	'LUX': 'LU', // Luxembourg
	'MAD': 'MG', // Madagascar
	'MAR': 'MA', // Morocco
	'MAS': 'MY', // Malaysia
	'MAW': 'MW', // Malawi
	'MDA': 'MD', // Moldova
	'MDV': 'MV', // Maldives
	'MEX': 'MX', // Mexico
	'MGL': 'MN', // Mongolia
	'MHL': 'MH', // Marshall Islands
	'MKD': 'MK', // North Macedonia
	'MLI': 'ML', // Mali
	'MLT': 'MT', // Malta
	'MNE': 'ME', // Montenegro
	'MON': 'MC', // Monaco
	'MOZ': 'MZ', // Mozambique
	'MRI': 'MU', // Mauritius
	'MTN': 'MR', // Mauritania
	'MYA': 'MM', // Myanmar
	'NAM': 'NA', // Namibia
	'NCA': 'NI', // Nicaragua
	'NED': 'NL', // Netherlands
	'NEP': 'NP', // Nepal
	'NGR': 'NG', // Nigeria
	'NIG': 'NE', // Niger
	'NOR': 'NO', // Norway
	'NRU': 'NR', // Nauru
	'NZL': 'NZ', // New Zealand
	'OMA': 'OM', // Oman
	'PAK': 'PK', // Pakistan
	'PAN': 'PA', // Panama
	'PAR': 'PY', // Paraguay
	'PER': 'PE', // Peru
	'PHI': 'PH', // Philippines
	'PLE': 'PS', // Palestine
	'PLW': 'PW', // Palau
	'PNG': 'PG', // Papua New Guinea
	'POL': 'PL', // Poland
	'POR': 'PT', // Portugal
	'PRK': 'KP', // North Korea
	'PUR': 'PR', // Puerto Rico
	'QAT': 'QA', // Qatar
	'ROU': 'RO', // Romania
	'RSA': 'ZA', // South Africa
	'RUS': 'RU', // Russia
	'RWA': 'RW', // Rwanda
	'SAM': 'WS', // Samoa
	'SEN': 'SN', // Senegal
	'SEY': 'SC', // Seychelles
	'SGP': 'SG', // Singapore
	'SKN': 'KN', // Saint Kitts and Nevis
	'SLE': 'SL', // Sierra Leone
	'SLO': 'SI', // Slovenia
	'SMR': 'SM', // San Marino
	'SOL': 'SB', // Solomon Islands
	'SOM': 'SO', // Somalia
	'SRB': 'RS', // Serbia
	'SRI': 'LK', // Sri Lanka
	'SSD': 'SS', // South Sudan
	'STP': 'ST', // São Tomé and Príncipe
	'SUD': 'SD', // Sudan
	'SUI': 'CH', // Switzerland
	'SUR': 'SR', // Suriname
	'SVK': 'SK', // Slovakia
	'SWE': 'SE', // Sweden
	'SWZ': 'SZ', // Eswatini
	'SYR': 'SY', // Syria
	'TAN': 'TZ', // Tanzania
	'TGA': 'TO', // Tonga
	'THA': 'TH', // Thailand
	'TJK': 'TJ', // Tajikistan
	'TKM': 'TM', // Turkmenistan
	'TLS': 'TL', // Timor-Leste
	'TOG': 'TG', // Togo
	'TPE': 'TW', // Chinese Taipei (Taiwan)
	'TTO': 'TT', // Trinidad and Tobago
	'TUN': 'TN', // Tunisia
	'TUR': 'TR', // Turkey
	'TUV': 'TV', // Tuvalu
	'UAE': 'AE', // United Arab Emirates
	'UGA': 'UG', // Uganda
	'UKR': 'UA', // Ukraine
	'URU': 'UY', // Uruguay
	'USA': 'US', // United States
	'UZB': 'UZ', // Uzbekistan
	'VAN': 'VU', // Vanuatu
	'VEN': 'VE', // Venezuela
	'VIE': 'VN', // Vietnam
	'VIN': 'VC', // Saint Vincent and the Grenadines
	'YEM': 'YE', // Yemen
	'ZAM': 'ZM', // Zambia
	'ZIM': 'ZW', // Zimbabwe
};

// Historical countries that need custom flags (no ISO codes)
const HISTORICAL_COUNTRIES = [
	'URS', // Soviet Union
	'TCH', // Czechoslovakia
	'YUG', // Yugoslavia
	'GDR', // East Germany
	'FRG', // West Germany
	'EUN', // Unified Team
	'SCG'  // Serbia and Montenegro
];

/**
 * Get flag URL for any country using IOC code
 * @param {string} iocCode - IOC country code (e.g., 'CAN', 'GER', 'URS')
 * @param {string} size - Size variant ('16x12', '20x15', '24x18', '28x21', '32x24', '40x30', '48x36', '56x42', '60x45', '64x48', '72x54', '80x60', '96x72', '108x81', '112x84', '120x90', '144x108', '160x120', '192x144', '224x168', '240x180', '256x192')
 * @returns {string} Flag URL
 */
function getCountryFlag(iocCode: string, size = 'w40') {
	// Check if it's a historical country
	if (HISTORICAL_COUNTRIES.includes(iocCode)) {
		return `/flags/${iocCode.toLowerCase()}.svg`;
	}

	// Convert IOC to ISO alpha-2 code
	const iso2Code = IOC_TO_ISO_ALPHA2[iocCode];

	if (!iso2Code) {
		console.warn(`No ISO alpha-2 code found for IOC code: ${iocCode}`);
		return `/flags/unknown.svg`; // fallback
	}

	// Return flag URL using ISO alpha-2 code
	return `https://flagcdn.com/${size}/${iso2Code.toLowerCase()}.png`;
}
console.log('Cambodia:', IOC_TO_ISO_ALPHA2['CAM']); // Should be 'KH'
console.log('Cameroon:', IOC_TO_ISO_ALPHA2['CMR']); // Should be 'CM' 
console.log('Canada:', IOC_TO_ISO_ALPHA2['CAN']);
/**
 * Get responsive flag image HTML with srcset for retina displays
 * @param {string} iocCode - IOC country code
 * @param {string} countryName - Country name for alt text
 * @param {number} baseWidth - Base width in pixels (default: 16)
 * @returns {string} HTML img element with srcset
 */
function getResponsiveFlagHTML(iocCode: string, countryName: string, baseWidth = 16) {
	if (HISTORICAL_COUNTRIES.includes(iocCode)) {
		return `<img src="/flags/${iocCode.toLowerCase()}.svg" width="${baseWidth}" alt="${countryName}">`;
	}

	const iso2Code = IOC_TO_ISO_ALPHA2[iocCode];
	if (!iso2Code) {
		return `<img src="/flags/unknown.svg" width="${baseWidth}" alt="${countryName}">`;
	}

	const baseHeight = Math.round(baseWidth * 0.75); // 4:3 aspect ratio
	const size1x = `${baseWidth}x${baseHeight}`;
	const size2x = `${baseWidth * 2}x${baseHeight * 2}`;
	const size3x = `${baseWidth * 3}x${baseHeight * 3}`;

	return `<img src="https://flagcdn.com/${size1x}/${iso2Code.toLowerCase()}.png" 
    srcset="https://flagcdn.com/${size2x}/${iso2Code.toLowerCase()}.png 2x, 
            https://flagcdn.com/${size3x}/${iso2Code.toLowerCase()}.png 3x" 
    width="${baseWidth}" 
    height="${baseHeight}" 
    alt="${countryName}">`;
}

// Export for use in your app
export {
	IOC_TO_ISO_ALPHA2,
	HISTORICAL_COUNTRIES,
	getCountryFlag,
	getResponsiveFlagHTML
};