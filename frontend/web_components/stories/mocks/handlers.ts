/**
 * Mock API handlers for Storybook
 * Uses MSW (Mock Service Worker) to intercept API calls
 */
import { http, HttpResponse, delay } from 'msw';

// Mock data for countries
export const mockCountries = [
  { id: 1, name: 'Czech Republic', iihf: true, iocCode: 'CZE', iso2Code: 'CZ', isHistorical: false, years: null, enabled: true },
  { id: 2, name: 'Slovakia', iihf: true, iocCode: 'SVK', iso2Code: 'SK', isHistorical: false, years: null, enabled: true },
  { id: 3, name: 'Finland', iihf: true, iocCode: 'FIN', iso2Code: 'FI', isHistorical: false, years: null, enabled: true },
  { id: 4, name: 'Sweden', iihf: true, iocCode: 'SWE', iso2Code: 'SE', isHistorical: false, years: null, enabled: true },
  { id: 5, name: 'Canada', iihf: true, iocCode: 'CAN', iso2Code: 'CA', isHistorical: false, years: null, enabled: true },
  { id: 6, name: 'United States', iihf: true, iocCode: 'USA', iso2Code: 'US', isHistorical: false, years: null, enabled: true },
  { id: 7, name: 'Russia', iihf: false, iocCode: 'RUS', iso2Code: 'RU', isHistorical: false, years: null, enabled: false },
  { id: 8, name: 'Soviet Union', iihf: false, iocCode: 'URS', iso2Code: null, isHistorical: true, years: '1946-1991', enabled: false },
  { id: 9, name: 'Germany', iihf: true, iocCode: 'GER', iso2Code: 'DE', isHistorical: false, years: null, enabled: true },
  { id: 10, name: 'Switzerland', iihf: true, iocCode: 'SUI', iso2Code: 'CH', isHistorical: false, years: null, enabled: true },
  { id: 11, name: 'Latvia', iihf: true, iocCode: 'LAT', iso2Code: 'LV', isHistorical: false, years: null, enabled: true },
  { id: 12, name: 'Denmark', iihf: true, iocCode: 'DEN', iso2Code: 'DK', isHistorical: false, years: null, enabled: true },
  { id: 13, name: 'Norway', iihf: true, iocCode: 'NOR', iso2Code: 'NO', isHistorical: false, years: null, enabled: true },
  { id: 14, name: 'Austria', iihf: true, iocCode: 'AUT', iso2Code: 'AT', isHistorical: false, years: null, enabled: true },
  { id: 15, name: 'France', iihf: true, iocCode: 'FRA', iso2Code: 'FR', isHistorical: false, years: null, enabled: true },
];

// Default handlers
export const handlers = [
  // GET /api/countries - Returns list of countries
  http.get('/api/countries', async () => {
    await delay(150); // Simulate network latency
    return HttpResponse.json(mockCountries);
  }),

  // POST /api/countries/:id/toggle - Toggle country enabled status
  http.post('/api/countries/:id/toggle', async ({ params }) => {
    await delay(200);
    const id = Number(params.id);
    const country = mockCountries.find(c => c.id === id);
    if (country) {
      country.enabled = !country.enabled;
      return HttpResponse.json({ success: true, enabled: country.enabled });
    }
    return HttpResponse.json({ error: 'Country not found' }, { status: 404 });
  }),

  // GET /api/countries/enabled - Returns only enabled countries (for selector)
  http.get('/api/countries/enabled', async () => {
    await delay(100);
    return HttpResponse.json(mockCountries.filter(c => c.enabled));
  }),
];

// Error scenarios for testing
export const errorHandlers = [
  http.get('/api/countries', async () => {
    await delay(150);
    return HttpResponse.json({ error: 'Internal server error' }, { status: 500 });
  }),
];

// Slow response handlers for loading state testing
export const slowHandlers = [
  http.get('/api/countries', async () => {
    await delay(3000); // 3 second delay
    return HttpResponse.json(mockCountries);
  }),
];

// Empty response handlers
export const emptyHandlers = [
  http.get('/api/countries', async () => {
    await delay(150);
    return HttpResponse.json([]);
  }),
];
