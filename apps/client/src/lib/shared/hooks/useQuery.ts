import { derived, type Readable } from "svelte/store"

import type { ApiResponse } from "$api/types"
import type { CreateQueryResult } from "@tanstack/svelte-query"

export const useQuery = <T>(queryFn: () => CreateQueryResult<ApiResponse<T>>) => {
	const query = queryFn()

	const queryData = derived(query, $query => $query.data)
	const isLoading = derived(query, $query => $query.isLoading)
	const error = derived(query, $query => $query.error)
	const isError = derived(query, $query => $query.isError)
	const refetch = derived(query, $query => $query.refetch) as Readable<
		() => Promise<void>
	>

	const data = derived(queryData, $queryData => $queryData?.data ?? null)

	const response = derived(queryData, $queryData => $queryData)

	return {
		data,
		isLoading,
		error,
		isError,
		refetch,
		response,
		query,
	}
}
