# Authenticated API Client - Migration Guide

This guide shows how to migrate from direct `fetch()` calls to the new authenticated API client.

## Overview

The new API client automatically:
- Injects JWT tokens from NextAuth session
- Handles 401 errors by redirecting to login
- Provides type-safe request methods
- Works in both Server Components and Client Components

## Server Components (Recommended)

Use the `apiClient` function or its typed helpers for server-side data fetching:

### Before (Direct fetch):
```typescript
export const fetchTeamList = async (page: number = 0): Promise<PaginatedResponse<Team>> => {
  const response = await fetch(`${API_URL}/team?page=${page}`);
  if (!response.ok) {
    throw new Error('Network response was not ok');
  }
  return response.json();
};
```

### After (With API Client):
```typescript
import { apiGet } from '@/lib/api-client';
import { PaginatedResponse } from '@/types/paging';
import { Team } from '@/types/team';

export const fetchTeamList = async (page: number = 0): Promise<PaginatedResponse<Team>> => {
  return apiGet<PaginatedResponse<Team>>(`/team?page=${page}`);
};
```

### Benefits:
- Automatic JWT token injection
- Automatic 401 error handling (redirect to login)
- Less boilerplate code
- Type-safe responses

## Client Components

For Client Components, use the `createClientApiClient` function:

### Example:
```typescript
'use client';

import { useSession } from 'next-auth/react';
import { createClientApiClient } from '@/lib/api-client';

export function MyClientComponent() {
  const { data: session } = useSession();

  const handleClick = async () => {
    const apiClient = createClientApiClient(session?.accessToken);

    try {
      const teams = await apiClient<PaginatedResponse<Team>>('/team');
      console.log('Teams:', teams);
    } catch (error) {
      console.error('Failed to fetch teams:', error);
    }
  };

  return <button onClick={handleClick}>Fetch Teams</button>;
}
```

## Typed Helper Functions

The API client provides typed helpers for common HTTP methods:

### GET Request:
```typescript
import { apiGet } from '@/lib/api-client';

const teams = await apiGet<PaginatedResponse<Team>>('/team');
```

### POST Request:
```typescript
import { apiPost } from '@/lib/api-client';

const newTeam = await apiPost<{ id: number }>(
  '/team',
  { name: 'Team A', country_id: 1 }
);
```

### PUT Request:
```typescript
import { apiPut } from '@/lib/api-client';

await apiPut<string>(
  '/team/123',
  { name: 'Updated Team', country_id: 2 }
);
```

### DELETE Request:
```typescript
import { apiDelete } from '@/lib/api-client';

await apiDelete<string>('/team/123');
```

## Error Handling

The API client throws `ApiError` for HTTP errors:

```typescript
import { apiGet, ApiError } from '@/lib/api-client';

try {
  const data = await apiGet<Team>('/team/123');
} catch (error) {
  if (error instanceof ApiError) {
    console.error(`API Error: ${error.status} - ${error.message}`);
    console.error(`URL: ${error.url}`);
  } else {
    console.error('Unexpected error:', error);
  }
}
```

## Unauthenticated Requests

For public endpoints that don't require authentication:

```typescript
import { apiGet } from '@/lib/api-client';

const publicData = await apiGet('/public/data', {
  requireAuth: false
});
```

## Migration Checklist

For each query file in `src/queries/`:

1. Import the API client helpers:
   ```typescript
   import { apiGet, apiPost, apiPut, apiDelete } from '@/lib/api-client';
   ```

2. Replace `fetch()` calls with typed helpers:
   - GET: `fetch(url)` → `apiGet<T>(endpoint)`
   - POST: `fetch(url, { method: 'POST', body })` → `apiPost<T>(endpoint, data)`
   - PUT: `fetch(url, { method: 'PUT', body })` → `apiPut<T>(endpoint, data)`
   - DELETE: `fetch(url, { method: 'DELETE' })` → `apiDelete<T>(endpoint)`

3. Remove manual error handling for 401s (now automatic)

4. Remove manual header management (JWT added automatically)

5. Simplify response parsing (no need for `response.json()`)

## Example Migration

### Before:
```typescript
export const createTeam = async (teamData: { name: string; country_id: string }): Promise<{ id: number }> => {
  const response = await fetch(`${API_URL}/team`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      name: teamData.name,
      country_id: parseInt(teamData.country_id),
    }),
  });
  if (!response.ok) {
    throw new Error('Network response was not ok');
  }
  return response.json();
};
```

### After:
```typescript
import { apiPost } from '@/lib/api-client';

export const createTeam = async (teamData: { name: string; country_id: string }): Promise<{ id: number }> => {
  return apiPost<{ id: number }>('/team', {
    name: teamData.name,
    country_id: parseInt(teamData.country_id),
  });
};
```

### Code reduction: 18 lines → 7 lines (61% reduction!)

## Testing

To test your migration:

1. Try the endpoint in the browser (should redirect to login if not authenticated)
2. Login and verify the API call works
3. Logout and verify 401 handling redirects to login
4. Check the Network tab to verify JWT token is included in requests

## Troubleshooting

### "Authentication required" error
- Ensure you're logged in
- Check that the session contains `accessToken`
- Verify `NEXTAUTH_SECRET` is set in `.env.local`

### 401 errors not redirecting
- Check that `handle401` is not set to `false`
- Verify the auth configuration in `src/auth.ts`

### Token not included in request
- Check the Network tab in DevTools
- Verify `Authorization: Bearer <token>` header is present
- Ensure you're using the server-side `apiClient` (not client-side fetch)

## Best Practices

1. Always specify the response type: `apiGet<ResponseType>(...)`
2. Use typed helpers (`apiGet`, `apiPost`, etc.) instead of raw `apiClient`
3. Handle errors appropriately (use try-catch or let React Query handle it)
4. For Client Components, always check session exists before creating client
5. Use Server Components when possible for better security (tokens stay server-side)
