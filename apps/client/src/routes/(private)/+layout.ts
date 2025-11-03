import { browser } from "$app/environment"
import { redirect } from "@sveltejs/kit"
import { auth } from "$lib/auth/store.svelte"
import { protectedApi } from "$lib/shared/api/client"

import type { User } from "$lib/users/schemas"
import type { LayoutLoad } from "./$types"

export const ssr = false

export const load: LayoutLoad = async () => {
	if (!browser) return {}

	const tokens = auth.getTokens()

	if (!tokens) {
		throw redirect(302, "/auth/login")
	}

	if (auth.user) {
		return { user: auth.user }
	}

	auth.isLoading = true

	try {
		const response = await protectedApi.get<{ data: User }>("/auth/me")
		auth.setUser(response.data.data)
		auth.isLoading = false

		return { user: auth.user }
	} catch (_: unknown) {
		auth.reset()
		throw redirect(302, "/auth/login")
	}
}
