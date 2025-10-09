import type { User } from "./schemas"
import type { ApiResponse } from "$api/utils"

import { api } from "$api/client"
import { TryFn } from "$api/utils"
import { createMutation, useQueryClient } from "@tanstack/svelte-query"

export const updateUserMutation = (id: string, data: Record<string, unknown>) => {
	const request = (data?: Record<string, unknown>) => {
		return api.patch<User>(`users/${id}`, data)
	}

	return createMutation<ApiResponse<User>, ApiResponse, unknown>(() => ({
		mutationKey: ["update-user", id],
		mutationFn: () => TryFn<User>(request, data),
		onSuccess: () => {
			useQueryClient().invalidateQueries({
				queryKey: ["user", id],
			})
		},
	}))
}
