import type { PageLoad } from "./$types"

export const ssr = false

export const load: PageLoad = async ({ parent }) => {
	// El layout padre ya validó el rol de estudiante
	const { user } = await parent()
	return { user }
}
