<script lang="ts">
	import { page } from "$app/stores"
	import { onMount } from "svelte"
	import { api } from "$lib/shared/api/client"

	let success = $state<boolean | null>(null)
	let isLoading = $state(true)

	onMount(async () => {
		const enrollmentsId = $page.params.enrollmentsId
		const practiceId = $page.params.practiceId

		try {
			const response = await api.post(
				`/enrollments/${enrollmentsId}/practice/${practiceId}/approve`
			)
			success = response.data.success
		} catch (error) {
			console.error("Error al aprobar práctica:", error)
			success = false
		} finally {
			isLoading = false
		}
	})
</script>

<div class="flex min-h-screen items-center justify-center bg-gray-50 px-4">
	<div class="w-full max-w-md rounded-lg bg-white p-8 shadow-lg">
		{#if isLoading}
			<div class="text-center">
				<div
					class="mx-auto mb-4 h-16 w-16 animate-spin rounded-full border-4 border-gray-200 border-t-indigo-600"
				></div>
				<h1 class="mb-2 text-2xl font-bold text-gray-900">Procesando...</h1>
				<p class="text-gray-600">Aprobando la práctica, por favor espere.</p>
			</div>
		{:else if success}
			<div class="text-center">
				<div
					class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-green-100"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-8 w-8 text-green-600"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M5 13l4 4L19 7"
						/>
					</svg>
				</div>
				<h1 class="mb-2 text-2xl font-bold text-gray-900">Práctica Aprobada</h1>
				<p class="text-gray-600">
					La práctica ha sido aprobada exitosamente. El estudiante recibirá una
					notificación por correo electrónico con los próximos pasos.
				</p>
			</div>
		{:else}
			<div class="text-center">
				<div
					class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-red-100"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-8 w-8 text-red-600"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						/>
					</svg>
				</div>
				<h1 class="mb-2 text-2xl font-bold text-gray-900">Error</h1>
				<p class="text-gray-600">
					No se pudo aprobar la práctica. Por favor, contacte con la secretaría de la
					carrera.
				</p>
			</div>
		{/if}
	</div>
</div>
