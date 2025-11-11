<script lang="ts">
	import { createForm, Form, Field, validate } from "@formisch/svelte"
	import { UpdateCourseSchema } from "$lib/courses/schemas"
	import { updateCourseMutation } from "$lib/courses/mutations"
	import { toast } from "svelte-sonner"
	import type { Course } from "$lib/courses/schemas"

	interface Props {
		course: Course
		onSuccess: () => void
		teachers: Array<{ id: string; name: string }>
	}

	let { course, onSuccess, teachers }: Props = $props()

	const mutation = updateCourseMutation(course.id)

	const form = createForm({
		schema: UpdateCourseSchema,
		initialInput: {
			code: course.code,
			name: course.name,
			teacherId: course.teacherId,
			year: course.year,
			evaluations: course.evaluations || [],
		},
	})

	async function onSubmit() {
		const result = await validate(form)

		if (!result.success) {
			toast.error("Por favor corrige los errores del formulario")
			return
		}

		try {
			const apiResult = await mutation.mutateAsync(result.output)

			if (apiResult.success) {
				toast.success("Curso actualizado exitosamente")
				onSuccess()
			} else {
				toast.error(
					typeof apiResult.error === "string"
						? apiResult.error
						: "Error al actualizar el curso"
				)
			}
		} catch (error) {
			toast.error("Error al actualizar el curso")
		}
	}
</script>

<Form of={form} onsubmit={onSubmit}>
	<div class="space-y-4">
		<Field of={form} path={["code"]}>
			{#snippet children(field)}
				<div>
					<label for="code" class="block text-sm font-medium text-gray-700">
						Código del Curso *
					</label>
					<input
						id="code"
						type="text"
						value={field.input ?? ""}
						{...field.props}
						placeholder="Ej: INFO1164"
						class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
					/>
					{#if field.errors}
						<p class="mt-1 text-sm text-red-600">{field.errors[0]}</p>
					{/if}
				</div>
			{/snippet}
		</Field>

		<Field of={form} path={["name"]}>
			{#snippet children(field)}
				<div>
					<label for="name" class="block text-sm font-medium text-gray-700">
						Nombre del Curso *
					</label>
					<input
						id="name"
						type="text"
						value={field.input ?? ""}
						{...field.props}
						placeholder="Ej: Ingeniería de Software"
						class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
					/>
					{#if field.errors}
						<p class="mt-1 text-sm text-red-600">{field.errors[0]}</p>
					{/if}
				</div>
			{/snippet}
		</Field>

		<Field of={form} path={["teacherId"]}>
			{#snippet children(field)}
				<div>
					<label for="teacherId" class="block text-sm font-medium text-gray-700">
						Profesor *
					</label>
					<select
						id="teacherId"
						value={field.input ?? ""}
						{...field.props}
						class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
					>
						<option value="">Seleccione un profesor</option>
						{#each teachers as teacher}
							<option value={teacher.id}>{teacher.name}</option>
						{/each}
					</select>
					{#if field.errors}
						<p class="mt-1 text-sm text-red-600">{field.errors[0]}</p>
					{/if}
				</div>
			{/snippet}
		</Field>

		<Field of={form} path={["year"]}>
			{#snippet children(field)}
				<div>
					<label for="year" class="block text-sm font-medium text-gray-700">
						Año *
					</label>
					<input
						id="year"
						type="number"
						value={field.input ?? new Date().getFullYear()}
						{...field.props}
						min="2000"
						max="2100"
						class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
					/>
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
				{form.isSubmitting ? "Guardando..." : "Guardar Cambios"}
			</button>
		</div>
	</div>
</Form>
