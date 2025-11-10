import type { User } from "./schemas"
import type { ApiResponse } from "$api/utils"

import { protectedApi } from "$api/client"
import { TryFn } from "$api/utils"
import { createMutation, useQueryClient } from "@tanstack/svelte-query"

export const createUserMutation = () => {
	const queryClient = useQueryClient()

	return createMutation<ApiResponse<User>, ApiResponse, Record<string, unknown>>(
		() => ({
			mutationKey: ["create-user"],
			mutationFn: data => TryFn<User>(() => protectedApi.post<User>(`users`, data)),
			onSuccess: () => {
				queryClient.invalidateQueries({
					queryKey: ["users"],
				})
			},
		})
	)
}

export const updateUserMutation = (id: string) => {
	const queryClient = useQueryClient()

	return createMutation<ApiResponse<User>, ApiResponse, Record<string, unknown>>(
		() => ({
			mutationKey: ["update-user", id],
			mutationFn: data =>
				TryFn<User>(() => protectedApi.patch<User>(`users/${id}`, data)),
			onSuccess: () => {
				queryClient.invalidateQueries({
					queryKey: ["user", id],
				})
				queryClient.invalidateQueries({
					queryKey: ["users"],
				})
			},
		})
	)
}

export const deleteUserMutation = (id: string) => {
	const queryClient = useQueryClient()

	return createMutation<ApiResponse<void>, ApiResponse, void>(() => ({
		mutationKey: ["delete-user", id],
		mutationFn: () => TryFn<void>(() => protectedApi.delete(`users/${id}`)),
		onSuccess: () => {
			queryClient.invalidateQueries({
				queryKey: ["users"],
			})
			queryClient.invalidateQueries({
				queryKey: ["user", id],
			})
		},
	}))
}
