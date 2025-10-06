<script lang="ts">
	import { api } from "$lib/shared/api/client"
	import { useMutation } from "$lib/shared/hooks/useTanstack"
	import { createMutation } from "@tanstack/svelte-query"
	import { type ApiResponse, req as tryRequest } from "$api/utils"

	let loginMutation = createMutation<ApiResponse<string>, ApiResponse, unknown>(
		() => ({
			mutationKey: ["google-login"],
			mutationFn: () => tryRequest(() => api.post("auth/login")),
			onMutate: () => {},
		})
	)

	let { mutate, error, isError } = useMutation(() => loginMutation)

	const onClick = () => {
		mutate(undefined, {
			onSuccess: response => {
				window.location.href = response.data ?? "/"
			},
		})
	}
</script>

<button onclick={onClick}>Login with Google</button>

{#if isError}
	<p>Error: {error}</p>
{/if}
