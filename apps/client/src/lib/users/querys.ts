import type { User } from "$users/schemas"
import type { ApiResponse } from "$api/utils"

import { protectedApi } from "$api/client"
import { createQuery } from "@tanstack/svelte-query"
import { req as tryRequest } from "$api/utils"

interface GetUsersParams {
	search?: string
	page?: number
}

interface GetUsersData {
	users: User[]
	currentPage: number
	totalPages: number
	totalUsers: number
	hasNext: boolean
	hasPrevious: boolean
}

export const getUsersQuery = (params: GetUsersParams) => {
	return createQuery<ApiResponse<GetUsersData>, ApiResponse>(() => ({
		queryKey: ["users", params.search, params.page],
		staleTime: 1000 * 60 * 1,
		queryFn: () => tryRequest(() => protectedApi.get("users", { params: params })),
	}))
}
