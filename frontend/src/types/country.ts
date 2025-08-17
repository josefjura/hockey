export interface Country {
	id: string;
	name: string;
	enabled: boolean;
	iihf: boolean;
	iso2_code: string | null;
	ioc_code: string | null; // Added IOC code for consistency with utils
	is_historical: boolean;
}
