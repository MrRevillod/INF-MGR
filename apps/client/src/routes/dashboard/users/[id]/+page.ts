import type { User } from "$lib/features/users/schemas"
import type { PageLoad } from "./$types"

import { useDecodeData } from "$lib/shared/hooks/useUrlData"

export const load: PageLoad = async ({ url }) => {
	const pageData = url.searchParams.get("data")
	const decoded = useDecodeData(pageData ?? "")

	if (!decoded || !decoded.user) {
		history.back()
	}

	return {
		user: decoded.user as User,
		roles: decoded.user.roles as User["roles"],
	}
}
