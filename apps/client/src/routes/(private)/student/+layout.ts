import { requireRole } from "$lib/auth/guards"
import type { LayoutLoad } from "./$types"

export const ssr = false

export const load: LayoutLoad = async ({ parent }) => {
	// El layout padre ya validó autenticación y cargó el usuario
	const { user } = await parent()

	// Validar rol específico
	requireRole("student")

	return { user }
}
