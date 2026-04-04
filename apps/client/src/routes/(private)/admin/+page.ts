import { requireRole } from "$lib/auth/guards"
import type { PageLoad } from "./$types"

export const ssr = false

export const load: PageLoad = async ({ parent }) => {
	// El layout padre ya validó autenticación y cargó el usuario
	const { user } = await parent()

	// Validar rol específico
	requireRole("administrator")

	return { user }
}
