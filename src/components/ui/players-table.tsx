"use client"

import React from 'react'
import {
  useReactTable,
  getCoreRowModel,
  getSortedRowModel,
  getFilteredRowModel,
  flexRender,
  createColumnHelper,
  type SortingState,
  type ColumnFiltersState,
} from '@tanstack/react-table'
import { ChevronUp, ChevronDown, ChevronsUpDown } from 'lucide-react'
import Image from 'next/image'
import type { Player } from '@/types/player'
import { getCountryFlag } from '@/utils/countryFlag'
import { useDeletePlayer } from '@/queries/players'

const columnHelper = createColumnHelper<Player>()

interface PlayersTableProps {
  data: Player[]
  loading?: boolean
  totalItems: number
  currentPage: number
  pageSize: number
  totalPages: number
  hasNext: boolean
  hasPrevious: boolean
  onPageChange: (page: number) => void
}

export default function PlayersTable({ 
  data, 
  loading = false, 
  totalItems,
  currentPage,
  pageSize,
  totalPages,
  hasNext,
  hasPrevious,
  onPageChange
}: PlayersTableProps) {
  const [sorting, setSorting] = React.useState<SortingState>([])
  const [columnFilters, setColumnFilters] = React.useState<ColumnFiltersState>([])
  
  const deletePlayerMutation = useDeletePlayer()

  const handleDeletePlayer = (player: Player) => {
    if (window.confirm(`Are you sure you want to delete player "${player.name}"?`)) {
      deletePlayerMutation.mutate(player.id)
    }
  }

  const columns = React.useMemo(() => [
    columnHelper.accessor('id', {
      header: 'ID',
      cell: info => info.getValue(),
      enableSorting: true,
    }),
    columnHelper.accessor('name', {
      header: 'Player Name',
      cell: info => info.getValue(),
      enableSorting: true,
    }),
    columnHelper.accessor('country_name', {
      header: 'Country',
      cell: ({ row }) => {
        const player = row.original
        return (
          <div className="flex items-center space-x-2">
            <Image 
              width={24} 
              height={18} 
              src={getCountryFlag(player.country_iso2_code, false)} 
              alt={player.country_iso2_code} 
              className='shadow-sm shadow-black' 
            />
            <span>{player.country_name}</span>
          </div>
        )
      },
      enableSorting: true,
    }),
    columnHelper.display({
      id: 'actions',
      header: 'Actions',
      cell: ({ row }) => (
        <div className="flex space-x-2">
          <button
            className="px-3 py-1 text-xs bg-blue-100 text-blue-700 rounded-md hover:bg-blue-200 transition-colors"
            onClick={() => console.log('Edit player:', row.original.id)}
          >
            Edit
          </button>
          <button
            className="px-3 py-1 text-xs bg-red-100 text-red-700 rounded-md hover:bg-red-200 transition-colors"
            onClick={() => handleDeletePlayer(row.original)}
            disabled={deletePlayerMutation.isPending}
          >
            {deletePlayerMutation.isPending ? 'Deleting...' : 'Delete'}
          </button>
        </div>
      ),
    }),
  ], [deletePlayerMutation.isPending])

  const table = useReactTable({
    data: data || [],
    columns,
    state: {
      sorting,
      columnFilters,
    },
    onSortingChange: setSorting,
    onColumnFiltersChange: setColumnFilters,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
  })

  if (loading) {
    return (
      <div className="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
        <table className="min-w-full divide-y divide-gray-300">
          <thead className="bg-gray-50">
            <tr>
              {columns.map((_, index) => (
                <th key={index} className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  <div className="h-4 bg-gray-200 rounded animate-pulse"></div>
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {Array.from({ length: 5 }).map((_, rowIndex) => (
              <tr key={rowIndex}>
                {columns.map((_, colIndex) => (
                  <td key={colIndex} className="px-6 py-4 whitespace-nowrap">
                    <div className="h-4 bg-gray-200 rounded animate-pulse"></div>
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    )
  }

  return (
    <div className="space-y-4">
      {/* Table */}
      <div className="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
        <table className="min-w-full divide-y divide-gray-300">
          <thead className="bg-gray-50">
            {table.getHeaderGroups().map(headerGroup => (
              <tr key={headerGroup.id}>
                {headerGroup.headers.map(header => (
                  <th
                    key={header.id}
                    className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                  >
                    {header.isPlaceholder ? null : (
                      <div
                        className={`flex items-center space-x-1 ${
                          header.column.getCanSort() ? 'cursor-pointer select-none' : ''
                        }`}
                        onClick={header.column.getToggleSortingHandler()}
                      >
                        <span>
                          {flexRender(header.column.columnDef.header, header.getContext())}
                        </span>
                        {header.column.getCanSort() && (
                          <span className="flex">
                            {header.column.getIsSorted() === 'asc' ? (
                              <ChevronUp className="h-4 w-4" />
                            ) : header.column.getIsSorted() === 'desc' ? (
                              <ChevronDown className="h-4 w-4" />
                            ) : (
                              <ChevronsUpDown className="h-4 w-4 opacity-50" />
                            )}
                          </span>
                        )}
                      </div>
                    )}
                  </th>
                ))}
              </tr>
            ))}
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {(table.getRowModel()?.rows || []).map(row => (
              <tr key={row.id} className="hover:bg-gray-50">
                {row.getVisibleCells().map(cell => (
                  <td key={cell.id} className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Empty state */}
      {(table.getRowModel()?.rows || []).length === 0 && (
        <div className="text-center py-8 text-gray-500">
          No players found
        </div>
      )}

      {/* Backend Pagination */}
      {totalPages > 1 && (
        <div className="flex items-center justify-between">
          <div className="text-sm text-gray-700">
            Showing {(currentPage - 1) * pageSize + 1} to{' '}
            {Math.min(currentPage * pageSize, totalItems)}{' '}
            of {totalItems} results
          </div>
          <div className="flex items-center space-x-2">
            <button
              className="px-3 py-1 text-sm bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => onPageChange(currentPage - 1)}
              disabled={!hasPrevious}
            >
              Previous
            </button>
            <span className="px-3 py-1 text-sm text-gray-700">
              Page {currentPage} of {totalPages}
            </span>
            <button
              className="px-3 py-1 text-sm bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => onPageChange(currentPage + 1)}
              disabled={!hasNext}
            >
              Next
            </button>
          </div>
        </div>
      )}
    </div>
  )
}