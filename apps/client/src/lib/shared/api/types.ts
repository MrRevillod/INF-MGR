export type ApiResponse<T = unknown> = {
	data?: T
	status: number
	success: boolean
	timestamp: string
	message: string
	error?: string
	errors?: Record<
		string,
		{
			message: string
			code?: string
		}[]
	>
}

export type Conflicts = {
	conflicts: Array<{
		field: string
		value?: string
		message: string
	}>
}
