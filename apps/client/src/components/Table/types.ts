export interface TableColumn<T> {
	key: keyof T
	label: string
	formatter?: (value: T) => string
}

export interface TableProps<T> {
	data: T[] | null
	columns: TableColumn<T>[]
	isLoading?: boolean
	isError?: boolean
	pagination: {
		currentPage: number
		totalPages?: number
		totalUsers?: number
		hasNext?: boolean
		hasPrevious?: boolean
		onPageChange: (page: number) => void
	}
	onDetailsClick?: (item: T) => void
}
