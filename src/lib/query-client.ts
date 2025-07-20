import { QueryClient } from '@tanstack/react-query'

export const queryClientConfig = {
    defaultOptions: {
        queries: {
            refetchOnWindowFocus: false,
            retry: 5,
            // All data we use is static and doesn't change frequently, so we can set a long cache time.
            // Any changes to the data will be known in advance and done manually.
            // When that happens, we can invalidate the cache manually.
            staleTime: Infinity,
        },
    },
}

export function makeQueryClient() {
    return new QueryClient(queryClientConfig)
}