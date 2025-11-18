<script lang="ts">
	import { createForm, Field, validate } from "@formisch/svelte"
	import {
		UpdateCourseSchema,
		UpdateCourseFormSchema,
		type CreateEvaluation,
		type Course,
	} from "$lib/courses/schemas"
	import { updateCourseMutation } from "$lib/courses/mutations"
	import { toast } from "svelte-sonner"
	import * as v from "valibot"

	interface Props {
		course: Course
		onSuccess: () => void
		teachers: Array<{ id: string; name: string }>
	}

	let { course, onSuccess, teachers }: Props = $props()

	const mutation = updateCourseMutation(course.id)

	// Estado local para las evaluaciones (inicializar con las evaluaciones existentes)
	let evaluations = $state<CreateEvaluation[]>(
		course.evaluations.map(ev => ({ name: ev.name, weight: ev.weight }))
	)

	const form = createForm({
		schema: UpdateCourseFormSchema,
		initialInput: {
			code: course.code,
			name: course.name,
			teacherId: course.teacherId,
			year: course.year,
		},
	})

	// Funciones para manejar evaluaciones dinámicamente
	function addEvaluation() {
		evaluations = [...evaluations, { name: "", weight: 0 }]
	}

	function removeEvaluation(index: number) {
		evaluations = evaluations.filter((_: any, i: number) => i !== index)
	}

	// Calcular total de porcentajes
	const totalWeight = $derived(
		evaluations.reduce(
			(sum: number, ev: CreateEvaluation) => sum + (ev.weight || 0),
			0
		)
	)

	async function onSubmit() {
		// Validar el formulario completo
		const formResult = await validate(form)

		if (!formResult.success) {
			toast.error("Por favor corrige los errores del formulario")
			return
		}

		// Combinar los datos del formulario con las evaluaciones locales
		const formData = {
			...formResult.output,
			evaluations: $state.snapshot(evaluations),
		}

		// Validar todo junto con el schema completo
		const result = v.safeParse(UpdateCourseSchema, formData)

		if (!result.success) {
			const firstError = result.issues?.[0]
			toast.error(
				firstError?.message || "Por favor corrige los errores del formulario"
			)
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
		} catch (error: any) {
			toast.error("Error al actualizar el curso")
		}
	}

	function handleFormSubmit(e: SubmitEvent) {
		e.preventDefault()
		onSubmit()
	}
</script>

<form onsubmit={handleFormSubmit}>
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

		<!-- Sección de Evaluaciones -->
		<div class="space-y-3 rounded-lg border border-gray-200 bg-white p-4">
			<div class="flex items-center justify-between">
				<div>
					<h3 class="text-base font-semibold text-gray-900">Evaluaciones</h3>
					<p class="text-sm text-gray-500">
						La suma de los porcentajes debe ser exactamente 100%
					</p>
				</div>
				<button
					type="button"
					onclick={addEvaluation}
					class="inline-flex items-center gap-1.5 rounded-md bg-blue-50 px-3 py-1.5 text-sm font-medium text-blue-700 hover:bg-blue-100"
				>
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
							d="M12 4v16m8-8H4"
						/>
					</svg>
					Agregar Evaluación
				</button>
			</div>

			{#each evaluations as evaluation, index}
				<div class="flex gap-2 rounded-md border border-gray-100 bg-gray-50 p-3">
					<div class="flex-1">
						<div>
							<label
								for="eval-name-{index}"
								class="block text-xs font-medium text-gray-700"
							>
								Nombre de la Evaluación
							</label>
							<input
								id="eval-name-{index}"
								type="text"
								bind:value={evaluations[index].name}
								placeholder="Ej: Prueba 1"
								class="mt-1 block w-full rounded-md border border-gray-300 px-2 py-1.5 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
							/>
						</div>
					</div>

					<div class="w-32">
						<div>
							<label
								for="eval-weight-{index}"
								class="block text-xs font-medium text-gray-700"
							>
								Porcentaje (%)
							</label>
							<input
								id="eval-weight-{index}"
								type="number"
								bind:value={evaluations[index].weight}
								min="1"
								max="100"
								placeholder="0"
								class="mt-1 block w-full rounded-md border border-gray-300 px-2 py-1.5 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
							/>
						</div>
					</div>

					<div class="flex items-end">
						<button
							type="button"
							onclick={() => removeEvaluation(index)}
							disabled={evaluations.length <= 1}
							aria-label="Eliminar evaluación"
							class="rounded-md bg-red-50 p-2 text-red-700 hover:bg-red-100 disabled:cursor-not-allowed disabled:opacity-50"
							title="Eliminar evaluación"
						>
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
									d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
								/>
							</svg>
						</button>
					</div>
				</div>
			{/each}

			<!-- Mostrar total de porcentajes -->
			<div
				class="flex items-center justify-between rounded-md bg-gray-100 px-3 py-2"
			>
				<span class="text-sm font-medium text-gray-700">Total de porcentajes:</span>
				<span
					class="text-base font-bold"
					class:text-green-600={totalWeight === 100}
					class:text-red-600={totalWeight !== 100}
				>
					{totalWeight}%
				</span>
			</div>
		</div>

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
</form>
