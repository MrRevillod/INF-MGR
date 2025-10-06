<script lang="ts">
	import { googleLoginMutation } from "$lib/features/auth/querys"

	let { data, error, isError, mutate } = googleLoginMutation()

	const onClick = () => {
		mutate(null, {
			onSuccess: response => {
				window.location.href = response.data ?? "/"
			},
		})
	}
</script>

<a href={data?.data ?? "#"}>Login Callback</a> <br />

<button onclick={onClick}>Login with Google</button>

{#if isError}
	<p>Error: {error}</p>
{/if}
