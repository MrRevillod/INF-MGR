import type { Asignature } from "$asignatures/schemas"
import type { User, Inscription } from "$users/schemas"

import { api } from "$api/client"
import { tryHttp } from "$api/utils"
import { createQuery } from "@tanstack/svelte-query"

interface GetUsersParams {
	role?: string
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
	const { role, search, page } = params

	const request = () => {
		return api.get<GetUsersResponseData>("users", {
			params: {
				role,
				page,
				search,
			},
		})
	}

	return createQuery({
		queryKey: ["users", role, search, page],
		enabled: !!role,
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
		return api.get<StudentInscriptionsResponse>(
			`users/student/${userId}/inscriptions`
		)
	}

	return createQuery({
		queryKey: ["student-inscriptions", userId],
		staleTime: 1000 * 60 * 1,
		queryFn: () => tryHttp<StudentInscriptionsResponse>({ fn: request }),
	})
}

export const getUserQuery = (userId: string) => {
	const request = () => {
		return api.get<User>(`users/${userId}`)
	}

	return createQuery({
		queryKey: ["user", userId],
		enabled: !!userId,
		staleTime: 1000 * 60 * 1,
		queryFn: () => tryHttp<User>({ fn: request }),
	})
}
