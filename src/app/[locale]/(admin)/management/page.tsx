import { dehydrate, HydrationBoundary } from '@tanstack/react-query'
import { fetchCountryList } from '@/queries/countries'
import ManagementPage from '@/ui/pages/management-page'
import { makeQueryClient } from '@/lib/query-client'

export default async function Management() {
    const queryClient = makeQueryClient()
    
    // Ensure the initial data is available server-side
    await queryClient.ensureQueryData({
        queryKey: ['countries', '', 0], // Initial state: no search, page 0
        queryFn: () => fetchCountryList(0, undefined),
        staleTime: 5 * 60 * 1000,
    })

    return (
        <HydrationBoundary state={dehydrate(queryClient)}>
            <ManagementPage />
        </HydrationBoundary>
    )
}
