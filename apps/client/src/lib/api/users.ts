import { api } from "."
import { appendQueryParams, tryHttp } from "./utils"

import type { Action } from "$lib/types"
import type { User } from "$lib/schemas/user"

interface GetUsersParams {
	role?: string
	search?: string
}

export const getUsers: Action<User[], GetUsersParams> = async ({ role, search }) => {
	return await tryHttp(
		await api.get(
			appendQueryParams("/users", {
				role: role?.slice(0, -1),
				search,
			})
		)
	)
}
