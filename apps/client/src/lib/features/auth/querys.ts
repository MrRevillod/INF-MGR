import { api } from "$api/client"
import { tryHttp } from "$api/utils"
import { createMutation } from "@tanstack/svelte-query"

import type { ApiResponse } from "$lib/shared/api/types"

export const googleLoginMutation = <E>() => {
	const request = () => {
		return api.post("auth/login")
	}

	return createMutation<ApiResponse<string>, E, unknown, unknown>(() => ({
		mutationKey: ["google-login"],
		mutationFn: () => tryHttp<string>({ fn: request }),
		onMutate: () => {},
	}))
}
