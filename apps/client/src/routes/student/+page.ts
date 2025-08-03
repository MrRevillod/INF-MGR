import type { User } from "$lib/features/users/schemas"
import type { PageLoad } from "./$types"

export const load: PageLoad = async () => {
	return {
		user: null as User | null,
	}
}
