import { api } from "."
import { createQuery } from "@tanstack/svelte-query"
import { appendQueryParams, tryHttp } from "./utils"

interface GetUsersParams {
	role?: string
	search?: string
}

export const getUserQuery = (role: string, search: string) => {
	const action = async ({ role, search }: GetUsersParams) => {
		return tryHttp({
			fn: await api.get(appendQueryParams("/users", { role, search })),
		})
	}

	return createQuery({
		queryKey: ["users", role, search],
		enabled: !!role,
		staleTime: 1000 * 60 * 1,
		queryFn: async () => await action({ role, search }),
	})
}
