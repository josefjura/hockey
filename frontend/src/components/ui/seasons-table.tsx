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
import { ChevronUp, ChevronDown, ChevronsUpDown, Edit2, Trash2 } from 'lucide-react'
import { useSession } from 'next-auth/react'
import type { Season } from '@/types/season'
import { useDeleteSeason } from '@/queries/seasons'
import Pager from '@/components/ui/pager'

const columnHelper = createColumnHelper<Season>()

interface SeasonsTableProps {
  data: Season[]
  loading?: boolean
  totalItems: number
  currentPage: number
  pageSize: number
  totalPages: number
  hasNext: boolean
  hasPrevious: boolean
  onPageChange: (page: number) => void
  onEdit?: (season: Season) => void
}

export default function SeasonsTable({ 
  data, 
  loading = false, 
  totalItems,
  currentPage,
  pageSize,
  totalPages,
  onPageChange,
  onEdit
}: SeasonsTableProps) {
  const { data: session } = useSession()
  const [sorting, setSorting] = React.useState<SortingState>([])
  const [columnFilters, setColumnFilters] = React.useState<ColumnFiltersState>([])
  const deleteSeasonMutation = useDeleteSeason()

  const handleDelete = React.useCallback(async (season: Season) => {
    if (window.confirm(`Are you sure you want to delete season ${season.year}${season.display_name ? ` "${season.display_name}"` : ''}?`)) {
      deleteSeasonMutation.mutate({ id: season.id, accessToken: session?.accessToken })
    }
  }, [deleteSeasonMutation, session?.accessToken])

  const columns = React.useMemo(() => [
    columnHelper.accessor('id', {
      header: 'ID',
      cell: info => info.getValue(),
      size: 80,
    }),
    columnHelper.accessor('year', {
      header: ({ column }) => (
        <button
          className="flex items-center space-x-1 hover:bg-gray-100 p-1 rounded"
          onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
        >
          <span>Year</span>
          {{
            asc: <ChevronUp className="h-4 w-4" />,
            desc: <ChevronDown className="h-4 w-4" />,
          }[column.getIsSorted() as string] ?? <ChevronsUpDown className="h-4 w-4" />}
        </button>
      ),
      cell: info => (
        <span className="font-semibold text-blue-600">
          {info.getValue()}
        </span>
      ),
      size: 100,
    }),
    columnHelper.accessor('display_name', {
      header: 'Display Name',
      cell: info => {
        const value = info.getValue()
        return (
          <span className={value ? "text-gray-900" : "text-gray-400 italic"}>
            {value || 'No display name'}
          </span>
        )
      },
      size: 200,
    }),
    columnHelper.accessor('event_name', {
      header: ({ column }) => (
        <button
          className="flex items-center space-x-1 hover:bg-gray-100 p-1 rounded"
          onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
        >
          <span>Event</span>
          {{
            asc: <ChevronUp className="h-4 w-4" />,
            desc: <ChevronDown className="h-4 w-4" />,
          }[column.getIsSorted() as string] ?? <ChevronsUpDown className="h-4 w-4" />}
        </button>
      ),
      cell: info => (
        <span className="text-gray-900">
          {info.getValue()}
        </span>
      ),
      size: 200,
    }),
    columnHelper.display({
      id: 'actions',
      header: 'Actions',
      cell: ({ row }) => (
        <div className="flex items-center space-x-2">
          {onEdit && (
            <button
              onClick={() => onEdit(row.original)}
              className="p-1 text-blue-600 hover:text-blue-800 hover:bg-blue-50 rounded transition-colors"
              title="Edit season"
            >
              <Edit2 className="h-4 w-4" />
            </button>
          )}
          <button
            onClick={() => handleDelete(row.original)}
            disabled={deleteSeasonMutation.isPending}
            className="p-1 text-red-600 hover:text-red-800 hover:bg-red-50 rounded transition-colors disabled:opacity-50"
            title="Delete season"
          >
            <Trash2 className="h-4 w-4" />
          </button>
        </div>
      ),
      size: 100,
    }),
  ], [onEdit, deleteSeasonMutation.isPending, handleDelete])

  const table = useReactTable({
    data,
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
      <div className="space-y-4">
        <div className="animate-pulse">
          <div className="h-10 bg-gray-200 rounded mb-4"></div>
          {[...Array(5)].map((_, i) => (
            <div key={i} className="h-12 bg-gray-100 rounded mb-2"></div>
          ))}
        </div>
      </div>
    )
  }

  return (
    <div className="space-y-4">
      <div className="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
        <table className="min-w-full divide-y divide-gray-300">
          <thead className="bg-gray-50">
            {table.getHeaderGroups().map(headerGroup => (
              <tr key={headerGroup.id}>
                {headerGroup.headers.map(header => (
                  <th
                    key={header.id}
                    className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    style={{ width: header.getSize() }}
                  >
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                  </th>
                ))}
              </tr>
            ))}
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {table.getRowModel().rows.map(row => (
              <tr key={row.id} className="hover:bg-gray-50">
                {row.getVisibleCells().map(cell => (
                  <td
                    key={cell.id}
                    className="px-6 py-4 whitespace-nowrap text-sm"
                  >
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {data.length === 0 && !loading && (
        <div className="text-center py-12">
          <div className="text-gray-500">No seasons found</div>
        </div>
      )}

      <Pager
        currentPage={currentPage}
        totalPages={totalPages}
        onPageChange={onPageChange}
        totalItems={totalItems}
        pageSize={pageSize}
      />
    </div>
  )
}