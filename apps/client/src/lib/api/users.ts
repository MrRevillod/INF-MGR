import type { Asignature } from "$lib/schemas/asignature"
import type { Inscription } from "$lib/schemas/inscriptions"
import type { User } from "$lib/schemas/user"

import { api } from "."
import { tryHttp } from "./utils"
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
		staleTime: 1000 * 60 * 1,
		queryFn: () => tryHttp<User>({ fn: request }),
	})
}
