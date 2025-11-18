import type { User } from "$users/schemas"
import type { ApiResponse } from "$api/utils"

import { protectedApi } from "$api/client"
import { createQuery } from "@tanstack/svelte-query"
import { TryFn } from "$api/utils"

export interface GetUsersParams {
	search?: string
	page?: number
	role?: string
}

export interface GetUsersData {
	users: User[]
	currentPage: number
	totalPages: number
	totalUsers: number
	hasNext: boolean
	hasPrevious: boolean
}

export const getUsersQuery = (getParams: () => GetUsersParams) => {
	return createQuery<ApiResponse<GetUsersData>, ApiResponse>(() => {
		const params = getParams()
		return {
			queryKey: ["users", params.search, params.page, params.role] as const,
			staleTime: 1000 * 60 * 1,
			queryFn: () => TryFn(() => protectedApi.get("users", { params })),
		}
	})
}
