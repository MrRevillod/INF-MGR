import type { Asignature } from "$lib/features/courses/schemas"
import type { User, Inscription } from "$users/schemas"

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

	return createQuery({
		queryKey: ["users", search, page],
		staleTime: 1000 * 60 * 1,
		queryFn: () => tryHttp<GetUsersResponseData>({ fn: request }),
	})
}

type StudentInscriptionsResponse = Array<
	Inscription & {
		asignature: Asignature
	}
>

export const getStudentInscriptionsQuery = (userId: string) => {
	const request = () => {
		return api.get<StudentInscriptionsResponse>(`inscriptions?userId=${userId}`)
	}

	return createQuery({
		queryKey: ["student-inscriptions", userId],
		staleTime: 1000 * 60 * 1,
		queryFn: () => tryHttp<StudentInscriptionsResponse>({ fn: request }),
	})
}
