<script lang="ts">
	import { createForm, Form, Field, validate } from "@formisch/svelte"
	import { EnrollStudentSchema } from "$lib/courses/schemas"
	import { enrollStudentMutation } from "$lib/courses/mutations"
	import { toast } from "svelte-sonner"

	interface Props {
		courseId: string
		onSuccess: () => void
		students: Array<{ id: string; name: string; rut: string }>
	}

	let { courseId, onSuccess, students }: Props = $props()

	const mutation = enrollStudentMutation()

	const form = createForm({
		schema: EnrollStudentSchema,
		initialInput: {
			studentId: "",
			courseId: courseId,
		},
	})

	async function onSubmit() {
		const result = await validate(form)

		if (!result.success) {
			toast.error("Por favor selecciona un estudiante")
			return
		}

		try {
			const apiResult = await mutation.mutateAsync(result.output)

			if (apiResult.success) {
				toast.success("Estudiante inscrito exitosamente")
				onSuccess()
			} else {
				toast.error(
					typeof apiResult.error === "string"
						? apiResult.error
						: "Error al inscribir al estudiante"
				)
			}
		} catch (error) {
			toast.error("Error al inscribir al estudiante")
		}
	}
</script>

<Form of={form} onsubmit={onSubmit}>
	<div class="space-y-4">
		<Field of={form} path={["studentId"]}>
			{#snippet children(field)}
				<div>
					<label for="studentId" class="block text-sm font-medium text-gray-700">
						Estudiante *
					</label>
					<select
						id="studentId"
						value={field.input ?? ""}
						{...field.props}
						class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
					>
						<option value="">Seleccione un estudiante</option>
						{#each students as student}
							<option value={student.id}>
								{student.name} - {student.rut}
							</option>
						{/each}
					</select>
					{#if field.errors}
						<p class="mt-1 text-sm text-red-600">{field.errors[0]}</p>
					{/if}
				</div>
			{/snippet}
		</Field>

		<div class="flex justify-end gap-2 pt-4">
			<button
				type="submit"
				disabled={form.isSubmitting}
				class="rounded bg-blue-600 px-4 py-2 text-white hover:bg-blue-700 disabled:opacity-50"
			>
				{form.isSubmitting ? "Inscribiendo..." : "Inscribir Estudiante"}
			</button>
		</div>
	</div>
</Form>
