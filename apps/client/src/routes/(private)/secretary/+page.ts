import { requireRole } from "$lib/auth/guards"
import type { PageLoad } from "./$types"

export const ssr = false

export const load: PageLoad = async ({ parent }) => {
	const { user } = await parent()
	requireRole("secretary")
	return { user }
}
