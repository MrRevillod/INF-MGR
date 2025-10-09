import { browser } from "$app/environment"
import type { User } from "$lib/users/schemas"

class AuthStore {
	accessToken: string | null = $state(null)
	refreshToken: string | null = $state(null)
	isLoading: boolean = $state(false)

	user: User | null = $state(null)
	isAuthenticated: boolean = $derived(this.user !== null)

	reset() {
		this.accessToken = null
		this.refreshToken = null
		this.user = null
		this.isLoading = false

		if (browser) {
			localStorage.removeItem("ACCESS")
			localStorage.removeItem("REFRESH")
		}
	}
}

export const authStore = new AuthStore()
