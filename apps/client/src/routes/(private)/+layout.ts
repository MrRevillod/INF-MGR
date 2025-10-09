import { browser } from "$app/environment"
import { authStore } from "$lib/shared/stores/auth.store.svelte"
import { protectedApi } from "$lib/shared/api/client"

import type { User } from "$lib/users/schemas"
import type { LayoutLoad } from "./$types"

export const load: LayoutLoad = async () => {
	if (!browser) return {}

	authStore.isLoading = true

	try {
		// El interceptor manejará automáticamente el refresh si es necesario
		const response = await protectedApi.get<{ data: User }>("/auth/me")

		authStore.user = response.data.data
		authStore.isAuthenticated = true
		authStore.isLoading = false

		return {
			user: authStore.user,
		}
	} catch (error: unknown) {
		console.error("Error checking auth:", error)
		// Si llegamos aquí, significa que:
		// 1. No había tokens
		// 2. El refresh falló
		// El interceptor ya habrá hecho reset() y goto("/auth/login")
		authStore.isLoading = false

		// No necesitas hacer nada más, el interceptor ya redirigió
		return {}
	}
}
