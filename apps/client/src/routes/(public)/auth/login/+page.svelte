<script lang="ts">
	import { useGoogleLoginMutation } from "$lib/auth/queries"

	const mutation = useGoogleLoginMutation()

	const onClick = () => {
		mutation.mutate(undefined, {
			onSuccess: response => {
				window.location.href = response.data ?? "/"
			},
		})
	}
</script>

<button onclick={onClick}>Login with Google</button>

{#if mutation.isError}
	<p>Error: {mutation.error}</p>
{/if}
