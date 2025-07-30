import type { MutateFunction } from "@tanstack/svelte-query"

// API Call types -----

export type ApiResponse<T = unknown> = {
	data: T
	status: number
	timestamp: string
	message: string
}

export type Conflicts = {
	conflicts: string[]
}

export type MutationFn<TData, TVariables> = MutateFunction<
	TData,
	Error,
	TVariables,
	unknown
>
