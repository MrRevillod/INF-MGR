<script lang="ts">
	import type { Enrollment } from "$lib/enrollments/schemas"
	import { protectedApi } from "$api/client"
	import { toast } from "svelte-sonner"

	interface Props {
		enrollment: Enrollment
	}

	let { enrollment }: Props = $props()

	let isLoading = $state(false)

	const handleViewAuthorization = async (e: MouseEvent) => {
		e.preventDefault()
		e.stopPropagation()

		if (!enrollment.practice?.id) {
			toast.error("Este estudiante no tiene una práctica asignada")
			return
		}

		isLoading = true

		try {
			// Obtener el PDF del servidor
			const response = await protectedApi.get(
				`/enrollments/practice/${enrollment.practice.id}/docs`,
				{
					responseType: "blob",
				}
			)

			// Crear una URL temporal para el blob
			const blob = new Blob([response.data], { type: "application/pdf" })
			const url = window.URL.createObjectURL(blob)

			// Abrir en nueva pestaña
			window.open(url, "_blank")

			// Limpiar la URL después de un tiempo
			setTimeout(() => window.URL.revokeObjectURL(url), 1000)
		} catch (error) {
			console.error("Error al obtener autorización:", error)
			toast.error("Error al cargar el documento de autorización")
		} finally {
			isLoading = false
		}
	}
</script>

<button
	onclick={handleViewAuthorization}
	disabled={isLoading || !enrollment.practice?.id}
	type="button"
	class="inline-flex items-center gap-1.5 rounded-md bg-blue-50 px-3 py-1.5 text-sm font-medium text-blue-700 hover:bg-blue-100 disabled:cursor-not-allowed disabled:opacity-50"
>
	{#if isLoading}
		<svg
			class="h-4 w-4 animate-spin"
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
		>
			<circle
				class="opacity-25"
				cx="12"
				cy="12"
				r="10"
				stroke="currentColor"
				stroke-width="4"
			></circle>
			<path
				class="opacity-75"
				fill="currentColor"
				d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
			></path>
		</svg>
		Cargando...
	{:else}
		<svg
			xmlns="http://www.w3.org/2000/svg"
			class="h-4 w-4"
			fill="none"
			viewBox="0 0 24 24"
			stroke="currentColor"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
			/>
		</svg>
		Ver Autorización
	{/if}
</button>
