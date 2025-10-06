import type { ApiResponse } from "$api/types"
import type { CreateMutationResult } from "@tanstack/svelte-query"

type MutationResult<T, E> = CreateMutationResult<ApiResponse<T>, E, unknown, unknown>

export const useMutation = <TData, TError>(
	mutationFn: () => MutationResult<TData, TError>
) => {
	const mutation = mutationFn()

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
