import type { User } from "$users/schemas"

import { api } from "$api/client"
import { tryHttp } from "$api/utils"
import { createQuery } from "@tanstack/svelte-query"

interface GetUsersParams {
	search?: string
	page?: number
}

interface GetUsersResponseData {
	users: User[]
	currentPage: number
	totalPages: number
	totalUsers: number
	hasNext: boolean
	hasPrevious: boolean
}

export const getUsersQuery = (params: GetUsersParams) => {
	const { search, page } = params

	const request = () => {
		return api.get<GetUsersResponseData>("users", {
			params: {
				page,
				search,
			},
		})
	}

	return createQuery(() => ({
		queryKey: ["users", search, page],
		staleTime: 1000 * 60 * 1,
		queryFn: () => tryHttp<GetUsersResponseData>({ fn: request }),
	}))
}
