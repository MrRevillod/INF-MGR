import { requireRole } from "$lib/auth/guards"
import type { PageLoad } from "./$types"

export const ssr = false

export const load: PageLoad = async ({ parent, params }) => {
	const { user } = await parent()
	requireRole("teacher")
	return { user, courseId: params.id }
}
