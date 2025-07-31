import type { PageLoad } from "./$types"
import type { Asignature } from "$lib/features/asignatures/schemas"
import type { Inscription, User } from "$lib/features/users/schemas"

import { useDecodeData } from "$lib/shared/hooks/useUrlData"

export const load: PageLoad = async ({ url }) => {
	const pageData = url.searchParams.get("data")
	const decoded = useDecodeData(pageData ?? "")

	if (!decoded || !decoded.user || !decoded.inscription) {
		history.back()
	}

	return {
		user: decoded.user as User,
		inscription: decoded.inscription as Inscription,
		asignature: decoded.inscription.asignature as Asignature,
	}
}
