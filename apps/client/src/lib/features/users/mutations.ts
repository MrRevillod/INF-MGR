import type { User } from "./schemas"
import type { ApiResponse } from "$api/types"

import { api } from "$api/client"
import { tryHttp } from "$api/utils"
import { createMutation, useQueryClient } from "@tanstack/svelte-query"

export const updateUserMutation = <E>(id: string, data: Record<string, unknown>) => {
	const request = (data?: Record<string, unknown>) => {
		return api.patch<User>(`users/${id}`, data)
	}

	return createMutation<ApiResponse<User>, E, unknown, unknown>({
		mutationKey: ["update-user", id],
		mutationFn: () => tryHttp<User>({ fn: request, args: data }),
		onSuccess: () => {
			useQueryClient().invalidateQueries({
				queryKey: ["user", id],
			})
		},
	})
}
