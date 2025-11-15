import type { Practice } from "./schemas"
import type { ApiResponse } from "$api/utils"

import { protectedApi } from "$api/client"
import { TryFn } from "$api/utils"
import { createQuery } from "@tanstack/svelte-query"

export const getPracticeQuery = (practiceId: string) => {
	return createQuery<ApiResponse<Practice>>(() => ({
		queryKey: ["practice", practiceId],
		queryFn: () =>
			TryFn<Practice>(() => protectedApi.get(`/practices/${practiceId}`)),
		enabled: !!practiceId,
	}))
}
