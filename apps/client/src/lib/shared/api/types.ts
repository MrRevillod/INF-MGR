export type ApiResponse<T = unknown> = {
	data: T | null
	status: number
	timestamp: string
	message: string
}

export type Conflicts = {
	conflicts: Array<{
		field: string
		value?: string
		message: string
	}>
}
