import { UseMutateFunction } from "@tanstack/react-query"

// API Call types -----

export type ApiResponse<T = unknown> = {
	data: T
	status: number
	message?: string
}

export type MutationFn<TData, TVariables> = UseMutateFunction<
	TData,
	Error,
	TVariables,
	unknown
>
