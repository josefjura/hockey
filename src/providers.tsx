"use client"

import type { ReactNode } from "react"
import { QueryClientProvider as TanstackQueryClientProvider } from "@tanstack/react-query"
import { ReactQueryDevtools } from "@tanstack/react-query-devtools"
import { useState } from "react"
import { makeQueryClient } from "@/lib/query-client"

interface Props {
    children: ReactNode
}

export function QueryClientProvider({ children }: Props) {
    const [client] = useState(() => makeQueryClient())

    return (
        <TanstackQueryClientProvider client={client}>
            {children}
            <ReactQueryDevtools />
        </TanstackQueryClientProvider>
    )
}