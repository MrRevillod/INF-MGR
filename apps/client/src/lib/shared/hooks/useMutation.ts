import type { ApiResponse } from "$api/types"
import type { CreateMutationResult } from "@tanstack/svelte-query"

import { derived } from "svelte/store"

type MutationResult<T, E> = CreateMutationResult<ApiResponse<T>, E, unknown, unknown>

export const useMutation = <TData, TError>(
	mutationFn: () => MutationResult<TData, TError>
) => {
	const mutation = mutationFn()

	const mutationData = derived(mutation, $mutation => $mutation.data)
	const isPending = derived(mutation, $mutation => $mutation.isPending)
	const isError = derived(mutation, $mutation => $mutation.isError)
	const error = derived(mutation, $mutation => $mutation.error)
	const mutate = derived(mutation, $mutation => $mutation.mutate)
	const mutateAsync = derived(mutation, $mutation => $mutation.mutateAsync)
	const reset = derived(mutation, $mutation => $mutation.reset)

	const data = derived(mutationData, $mutationData => $mutationData?.data ?? null)
	const response = derived(mutationData, $mutationData => $mutationData)

	return {
		data,
		isPending,
		isError,
		error,
		mutate,
		mutateAsync,
		reset,
		response,
		mutation,
	}
}
