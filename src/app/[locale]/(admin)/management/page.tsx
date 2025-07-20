import { dehydrate, HydrationBoundary } from '@tanstack/react-query'
import { countryQueries } from '@/queries/countries'
import ManagementPage from '@/ui/pages/management-page'
import { makeQueryClient } from '@/lib/query-client'

export default async function Management() {
    const queryClient = makeQueryClient()
    
    // Ensure the initial data is available server-side
    await queryClient.ensureQueryData(countryQueries.list('', 0))

    return (
        <HydrationBoundary state={dehydrate(queryClient)}>
            <ManagementPage />
        </HydrationBoundary>
    )
}
