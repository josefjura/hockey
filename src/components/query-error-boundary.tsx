"use client"

import { QueryErrorResetBoundary } from '@tanstack/react-query'
import ErrorBoundary from './error-boundary'
import { ReactNode } from 'react'

interface QueryErrorBoundaryProps {
  children: ReactNode
  fallback?: ReactNode
}

export default function QueryErrorBoundary({ children, fallback }: QueryErrorBoundaryProps) {
  return (
    <QueryErrorResetBoundary>
      {({ reset }) => (
        <ErrorBoundary
          onError={(error, errorInfo) => {
            console.error('Query error boundary caught:', error, errorInfo)
            // Reset React Query state on error
            reset()
          }}
          fallback={fallback}
        >
          {children}
        </ErrorBoundary>
      )}
    </QueryErrorResetBoundary>
  )
}