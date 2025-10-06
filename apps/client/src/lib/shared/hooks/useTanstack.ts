import type { ApiResponse } from "$api/utils"
import type { CreateMutationResult, CreateQueryResult } from "@tanstack/svelte-query"

type QueryResult<T> = CreateQueryResult<ApiResponse<T>, ApiResponse>
type MutationResult<T, E> = CreateMutationResult<ApiResponse<T>, E, unknown, unknown>

export const useMutation = <T>(fn: () => MutationResult<T, ApiResponse>) => {
	const mutation = fn()

	return {
		data: mutation.data?.data,
		isPending: mutation.isPending,
		isError: mutation.isError,
		error: mutation.error,
		mutate: mutation.mutate,
		mutateAsync: mutation.mutateAsync,
		reset: mutation.reset,
		response: mutation.data,
		mutation,
	}
}

export const useQuery = <T>(fn: () => QueryResult<T>) => {
	const query = fn()

	return {
		data: query.data?.data ?? null,
		isLoading: query.isLoading,
		error: query.error,
		isError: query.isError,
		refetch: query.refetch,
		response: query.data,
		query,
	}
}
