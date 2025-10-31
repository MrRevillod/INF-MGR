import { api, protectedApi } from "$lib/shared/api/client"
import { createMutation } from "@tanstack/svelte-query"
import { type ApiResponse, TryFn } from "$api/utils"

export const useLogoutMutation = () => {
	return createMutation<ApiResponse<null>, ApiResponse>(() => ({
		mutationKey: ["logout"],
		mutationFn: () => TryFn(() => protectedApi.post("auth/logout")),
	}))
}

export const useGoogleLoginMutation = () => {
	return createMutation<ApiResponse<string>, ApiResponse>(() => ({
		mutationKey: ["google-login"],
		mutationFn: () => TryFn(() => api.post("auth/login")),
	}))
}
