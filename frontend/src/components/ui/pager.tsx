"use client"

import { ChevronLeft, ChevronRight } from 'lucide-react'

interface PagerProps {
  currentPage: number
  totalPages: number
  onPageChange: (page: number) => void
  pageSize?: number
  totalItems?: number
}

export default function Pager({ 
  currentPage, 
  totalPages, 
  onPageChange, 
  pageSize = 10, 
  totalItems 
}: PagerProps) {
  const startItem = currentPage * pageSize + 1
  const endItem = Math.min((currentPage + 1) * pageSize, totalItems || 0)

  const getVisiblePages = () => {
    const pages = []
    const maxVisiblePages = 7
    
    if (totalPages <= maxVisiblePages) {
      // Show all pages if total is small
      for (let i = 0; i < totalPages; i++) {
        pages.push(i)
      }
    } else {
      // Show first page
      pages.push(0)
      
      if (currentPage > 3) {
        pages.push(-1) // Ellipsis
      }
      
      // Show pages around current page
      const start = Math.max(1, currentPage - 2)
      const end = Math.min(totalPages - 1, currentPage + 2)
      
      for (let i = start; i <= end; i++) {
        pages.push(i)
      }
      
      if (currentPage < totalPages - 4) {
        pages.push(-1) // Ellipsis
      }
      
      // Show last page
      if (totalPages > 1) {
        pages.push(totalPages - 1)
      }
    }
    
    return pages
  }

  if (totalPages <= 1) {
    return null
  }

  return (
    <div className="flex items-center justify-between px-6 py-3 border-t border-gray-200 bg-white">
      <div className="flex flex-1 justify-between sm:hidden">
        {/* Mobile pagination */}
        <button
          type="button"
          onClick={(e) => {
            e.preventDefault()
						e.stopPropagation()
            onPageChange(currentPage - 1)
          }}
          disabled={currentPage === 0}
          className="relative inline-flex items-center px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Previous
        </button>
        <button
          type="button"
          onClick={(e) => {
            e.preventDefault()
						e.stopPropagation()
            onPageChange(currentPage + 1)
          }}
          disabled={currentPage === totalPages - 1}
          className="relative ml-3 inline-flex items-center px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Next
        </button>
      </div>
      
      <div className="hidden sm:flex sm:flex-1 sm:items-center sm:justify-between">
        <div>
          <p className="text-sm text-gray-700">
            Showing <span className="font-medium">{startItem}</span> to{' '}
            <span className="font-medium">{endItem}</span> of{' '}
            <span className="font-medium">{totalItems}</span> results
          </p>
        </div>
        
        <div>
          <nav className="relative z-0 inline-flex rounded-md shadow-sm -space-x-px">
            {/* Previous button */}
            <button
              type="button"
              onClick={(e) => {
                e.preventDefault()
                onPageChange(currentPage - 1)
              }}
              disabled={currentPage === 0}
              className="relative inline-flex items-center px-2 py-2 rounded-l-md border border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <ChevronLeft className="h-5 w-5" />
            </button>
            
            {/* Page numbers */}
            {getVisiblePages().map((page, index) => (
              <div key={page === -1 ? `ellipsis-${index}` : `page-${page}`}>
                {page === -1 ? (
                  <span className="relative inline-flex items-center px-4 py-2 border border-gray-300 bg-white text-sm font-medium text-gray-700">
                    ...
                  </span>
                ) : (
                  <button
                    type="button"
                    onClick={(e) => {
                      e.preventDefault()
                      onPageChange(page)
                    }}
                    className={`relative inline-flex items-center px-4 py-2 border text-sm font-medium ${
                      page === currentPage
                        ? 'z-10 bg-blue-50 border-blue-500 text-blue-600'
                        : 'bg-white border-gray-300 text-gray-500 hover:bg-gray-50'
                    }`}
                  >
                    {page + 1}
                  </button>
                )}
              </div>
            ))}
            
            {/* Next button */}
            <button
              type="button"
              onClick={(e) => {
                e.preventDefault()
                onPageChange(currentPage + 1)
              }}
              disabled={currentPage === totalPages - 1}
              className="relative inline-flex items-center px-2 py-2 rounded-r-md border border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <ChevronRight className="h-5 w-5" />
            </button>
          </nav>
        </div>
      </div>
    </div>
  )
}
