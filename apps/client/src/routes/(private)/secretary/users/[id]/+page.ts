import type { User } from "$users/schemas"
import type { PageLoad } from "./$types"

import { useDecodeData } from "$lib/shared/hooks/useUrlData"

export const load: PageLoad = async ({ url }) => {
	const pageData = url.searchParams.get("data")
	const decoded = useDecodeData(pageData ?? "")

	if (!decoded || !decoded.user) {
		// Si no hay datos, regresar a la página de usuarios
		history.back()
	}

	return {
		user: decoded.user as User,
	}
}
