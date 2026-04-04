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

<section class="flex h-screen w-full flex-col items-center justify-center">
	<section
		class="flex flex-col items-center justify-center gap-4 rounded-md border-2 border-gray-200 bg-white p-12"
	>
		<h1 class="mb-4 text-2xl">Sistema de Prácticas y Tesis</h1>
		<button
			class="bg-accent hover:bg-accent-hover cursor-pointer rounded px-4 py-2 text-white"
			on:click={onClick}
		>
			Iniciar sesión con Google
		</button>
	</section>
</section>

{#if mutation.isError}
	<p>Error: {mutation.error}</p>
{/if}
