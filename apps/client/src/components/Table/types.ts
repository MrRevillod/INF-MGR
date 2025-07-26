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
	actions: Array<{
		label: string
		icon: any
		func: (item: T) => void
	}>
	onPageChange: (action: "prev" | "next") => void
}
