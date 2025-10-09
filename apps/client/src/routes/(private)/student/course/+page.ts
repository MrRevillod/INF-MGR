import type { User } from "$lib/features/users/schemas"
import type { PageLoad } from "./$types"
import type { Inscription } from "$lib/features/courses/schemas"

import { useDecodeData } from "$lib/shared/hooks/useUrlData"

export const load: PageLoad = async ({ url }) => {
	const pageData = url.searchParams.get("data")
	const decoded = useDecodeData(pageData ?? "")

	if (!decoded || !decoded.inscription) {
		history.back()
	}

	return {
		inscription: decoded.inscription as Inscription,
		user: null as User | null,
	}
}
