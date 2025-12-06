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
import Image from 'next/image'
import type { Team } from '@/types/team'
import { getCountryFlag } from '@/utils/countryFlag'
import { useDeleteTeam } from '@/queries/teams'
import Pager from '@/components/ui/pager'

const columnHelper = createColumnHelper<Team>()

interface TeamsTableProps {
  data: Team[]
  loading?: boolean
  totalItems: number
  currentPage: number
  pageSize: number
  totalPages: number
  hasNext: boolean
  hasPrevious: boolean
  onPageChange: (page: number) => void
  onEdit?: (team: Team) => void
}

export default function TeamsTable({ 
  data, 
  loading = false, 
  totalItems,
  currentPage,
  pageSize,
  totalPages,
  onPageChange,
  onEdit
}: TeamsTableProps) {
  const { data: session } = useSession()
  const [sorting, setSorting] = React.useState<SortingState>([])
  const [columnFilters, setColumnFilters] = React.useState<ColumnFiltersState>([])
  const deleteTeamMutation = useDeleteTeam()

  const handleDelete = React.useCallback(async (team: Team) => {
    if (window.confirm(`Are you sure you want to delete team "${team.name || 'National Team'}"?`)) {
      deleteTeamMutation.mutate({ id: parseInt(team.id), accessToken: session?.accessToken })
    }
  }, [deleteTeamMutation, session?.accessToken])

  const columns = React.useMemo(() => [
    columnHelper.accessor('id', {
      header: 'ID',
      cell: info => info.getValue(),
      size: 80,
    }),
    columnHelper.accessor('name', {
      header: ({ column }) => (
        <button
          className="flex items-center space-x-1 hover:bg-gray-100 p-1 rounded"
          onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
        >
          <span>Team Name</span>
          {{
            asc: <ChevronUp className="h-4 w-4" />,
            desc: <ChevronDown className="h-4 w-4" />,
          }[column.getIsSorted() as string] ?? <ChevronsUpDown className="h-4 w-4" />}
        </button>
      ),
      cell: info => (
        <span className="font-semibold text-blue-600">
          {info.getValue() || 'National Team'}
        </span>
      ),
      size: 200,
    }),
    columnHelper.accessor('country_name', {
      header: ({ column }) => (
        <button
          className="flex items-center space-x-1 hover:bg-gray-100 p-1 rounded"
          onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
        >
          <span>Country</span>
          {{
            asc: <ChevronUp className="h-4 w-4" />,
            desc: <ChevronDown className="h-4 w-4" />,
          }[column.getIsSorted() as string] ?? <ChevronsUpDown className="h-4 w-4" />}
        </button>
      ),
      cell: ({ row }) => {
        const team = row.original
        return (
          <div className="flex items-center space-x-2">
            <Image 
              width={24} 
              height={18} 
              src={getCountryFlag(team.country_iso2_code, false)} 
              alt={team.country_iso2_code} 
              className='shadow-sm shadow-black' 
            />
            <span className="text-gray-900">{team.country_name}</span>
          </div>
        )
      },
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
              title="Edit team"
            >
              <Edit2 className="h-4 w-4" />
            </button>
          )}
          <button
            onClick={() => handleDelete(row.original)}
            disabled={deleteTeamMutation.isPending}
            className="p-1 text-red-600 hover:text-red-800 hover:bg-red-50 rounded transition-colors disabled:opacity-50"
            title="Delete team"
          >
            <Trash2 className="h-4 w-4" />
          </button>
        </div>
      ),
      size: 100,
    }),
  ], [onEdit, deleteTeamMutation.isPending, handleDelete])

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
          <div className="text-gray-500">No teams found</div>
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