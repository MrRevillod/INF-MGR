import { auth } from "./store.svelte"
import { redirect } from "@sveltejs/kit"
import type { Role } from "$lib/users/schemas"

export function requireRole(role: Role): void {
	if (!auth.user) {
		throw redirect(302, "/auth/login")
	}

	if (!auth.hasRole(role)) {
		auth.navigateToDashboard()
		throw redirect(302, "/")
	}
}

export function requireAnyRole(roles: Role[]): void {
	if (!auth.user) {
		throw redirect(302, "/auth/login")
	}

	if (!auth.hasAnyRole(roles)) {
		auth.navigateToDashboard()
		throw redirect(302, "/")
	}
}
