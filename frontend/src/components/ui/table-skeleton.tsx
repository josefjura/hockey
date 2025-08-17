interface TableSkeletonProps {
    rows?: number
    columns?: number
    headers?: string[]
}

export default function TableSkeleton({ 
    rows = 10, 
    columns = 5, 
    headers = [] 
}: TableSkeletonProps) {
    return (
        <div className="animate-pulse">
            <div className="overflow-x-auto">
                <table className="min-w-full divide-y divide-gray-200">
                    {headers.length > 0 && (
                        <thead className="bg-gray-50">
                            <tr>
                                {headers.map((header, index) => (
                                    <th 
                                        key={index}
                                        className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                    >
                                        {header}
                                    </th>
                                ))}
                            </tr>
                        </thead>
                    )}
                    <tbody className="bg-white divide-y divide-gray-200">
                        {Array.from({ length: rows }, (_, rowIndex) => (
                            <tr key={`skeleton-row-${rowIndex}`}>
                                {Array.from({ length: columns }, (_, colIndex) => (
                                    <td key={`skeleton-cell-${rowIndex}-${colIndex}`} className="px-6 py-4 whitespace-nowrap">
                                        {colIndex === 0 ? (
                                            // First column with flag + text pattern
                                            <div className="flex items-center">
                                                <div className="w-8 h-6 bg-gray-300 rounded mr-2"></div>
                                                <div className="h-4 bg-gray-300 rounded w-32"></div>
                                            </div>
                                        ) : (
                                            // Other columns with badge-like pattern
                                            <div className="h-6 bg-gray-300 rounded-full w-12"></div>
                                        )}
                                    </td>
                                ))}
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    )
}