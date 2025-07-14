const API_URL = process.env.BACKEND_URL || 'http://localhost:8080';

export const fetchCountryList = async () => {
	const response = await fetch(`${API_URL}/country`);
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	return response.json();
}

export const updateCountryStatus = async (countryId: string, status: boolean) => {
	const response = await fetch(`${API_URL}/country/${countryId}`, {
		method: 'PATCH',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify(status),
	});
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	return response.json();
}
