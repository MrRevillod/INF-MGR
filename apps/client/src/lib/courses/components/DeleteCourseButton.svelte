<script lang="ts">
	import { deleteCourseMutation } from "$lib/courses/mutations"
	import { goto } from "$app/navigation"
	import { toast } from "svelte-sonner"
	import ConfirmDialog from "$lib/shared/components/ConfirmDialog.svelte"

	interface Props {
		courseId: string
		courseName: string
	}

	let { courseId, courseName }: Props = $props()

	const mutation = deleteCourseMutation(courseId)

	let isDeleting = $state(false)
	let showConfirmDialog = $state(false)

	function handleDeleteClick() {
		showConfirmDialog = true
	}

	async function handleConfirmDelete() {
		showConfirmDialog = false
		isDeleting = true

		try {
			const apiResult = await mutation.mutateAsync()

			if (apiResult.success) {
				toast.success("Curso eliminado exitosamente")
				goto("/admin/courses")
			} else {
				// Detectar si el error es por estudiantes inscritos
				const errorMessage =
					typeof apiResult.error === "string"
						? apiResult.error
						: JSON.stringify(apiResult.error)

				if (
					errorMessage.toLowerCase().includes("enrollment") ||
					errorMessage.toLowerCase().includes("inscritos")
				) {
					toast.error(
						"No se puede eliminar el curso porque tiene estudiantes inscritos. Elimina primero todas las inscripciones."
					)
				} else {
					toast.error(
						typeof apiResult.error === "string"
							? apiResult.error
							: "Error al eliminar el curso"
					)
				}
				isDeleting = false
			}
		} catch (error: any) {
			// Manejar el error de axios
			const errorMsg = error?.response?.data?.message || error?.message || ""

			if (
				errorMsg.toLowerCase().includes("enrollment") ||
				errorMsg.toLowerCase().includes("inscritos")
			) {
				toast.error(
					"No se puede eliminar el curso porque tiene estudiantes inscritos. Elimina primero todas las inscripciones."
				)
			} else {
				toast.error(errorMsg || "Error al eliminar el curso")
			}
			isDeleting = false
		}
	}

	function handleCancelDelete() {
		showConfirmDialog = false
	}
</script>

<button
	type="button"
	onclick={handleDeleteClick}
	disabled={isDeleting}
	class="rounded bg-red-600 px-4 py-2 text-white hover:bg-red-700 disabled:opacity-50"
>
	{isDeleting ? "Eliminando..." : "Eliminar Curso"}
</button>

<ConfirmDialog
	bind:isOpen={showConfirmDialog}
	title="Eliminar Curso"
	message="¿Estás seguro de eliminar el curso '{courseName}'? Si tiene estudiantes inscritos, debes eliminarlos primero."
	confirmText="Sí, eliminar"
	cancelText="Cancelar"
	variant="danger"
	onConfirm={handleConfirmDelete}
	onCancel={handleCancelDelete}
/>
