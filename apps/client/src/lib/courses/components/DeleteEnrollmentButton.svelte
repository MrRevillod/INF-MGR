<script lang="ts">
	import { deleteEnrollmentMutation } from "$lib/courses/mutations"
	import { toast } from "svelte-sonner"
	import ConfirmDialog from "$lib/shared/components/ConfirmDialog.svelte"

	interface Props {
		enrollmentId: string
		studentName: string
	}

	let { enrollmentId, studentName }: Props = $props()

	const mutation = deleteEnrollmentMutation(enrollmentId)

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
				toast.success("Inscripción eliminada exitosamente")
			} else {
				toast.error(
					typeof apiResult.error === "string"
						? apiResult.error
						: "Error al eliminar la inscripción"
				)
				isDeleting = false
			}
		} catch (error) {
			toast.error("Error al eliminar la inscripción")
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
	title="Eliminar inscripción"
	class="rounded bg-red-600 px-2 py-1 text-sm text-white hover:bg-red-700 disabled:opacity-50"
>
	{isDeleting ? "..." : "❌"}
</button>

<ConfirmDialog
	bind:isOpen={showConfirmDialog}
	title="Eliminar Inscripción"
	message="¿Estás seguro de eliminar la inscripción de '{studentName}'?"
	confirmText="Sí, eliminar"
	cancelText="Cancelar"
	variant="danger"
	onConfirm={handleConfirmDelete}
	onCancel={handleCancelDelete}
/>
