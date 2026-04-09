<script lang="ts">
	import { createForm, Field, validate } from "@formisch/svelte"
	import {
		CreateCourseSchema,
		CreateCourseFormSchema,
		type CreateEvaluation,
	} from "$lib/courses/schemas"
	import { createCourseMutation } from "$lib/courses/mutations"
	import { toast } from "svelte-sonner"
	import * as v from "valibot"

	interface Props {
		onSuccess: () => void
		teachers: Array<{ id: string; name: string }>
	}

	let { onSuccess, teachers }: Props = $props()

	const mutation = createCourseMutation()

	// Estado local para las evaluaciones (temporal)
	let evaluations = $state<CreateEvaluation[]>([{ name: "", weight: 0 }])
	let supervisorEvaluationIndex = $state(0)

	const form = createForm({
		schema: CreateCourseFormSchema,
		initialInput: {
			code: "",
			name: "",
			teacherId: "",
			year: new Date().getFullYear(),
		},
	})

	// Funciones para manejar evaluaciones dinámicamente
	function addEvaluation() {
		evaluations = [...evaluations, { name: "", weight: 0 }]
	}

	function removeEvaluation(index: number) {
		evaluations = evaluations.filter((_: any, i: number) => i !== index)

		if (supervisorEvaluationIndex >= evaluations.length) {
			supervisorEvaluationIndex = Math.max(0, evaluations.length - 1)
		}
	}

	function orderedEvaluationsForSubmit(): CreateEvaluation[] {
		if (
			supervisorEvaluationIndex < 0 ||
			supervisorEvaluationIndex >= evaluations.length
		) {
			return $state.snapshot(evaluations)
		}

		const snapshot = $state.snapshot(evaluations)
		const selected = snapshot[supervisorEvaluationIndex]
		const others = snapshot.filter((_, idx) => idx !== supervisorEvaluationIndex)

		return [selected, ...others]
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
			evaluations: orderedEvaluationsForSubmit(),
		}

		// Validar todo junto con el schema completo
		const result = v.safeParse(CreateCourseSchema, formData)

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
				toast.success("Curso creado exitosamente")
				onSuccess()
			} else {
				toast.error(
					typeof apiResult.error === "string"
						? apiResult.error
						: "Error al crear el curso"
				)
			}
		} catch (error: any) {
			toast.error("Error al crear el curso")
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
		<div class="space-y-3 rounded-lg border border-gray-200 p-4">
			<div class="flex items-center justify-between">
				<div>
					<h3 class="text-sm font-medium text-gray-900">Evaluaciones *</h3>
					<p class="text-xs text-gray-500">
						La evaluacion marcada para supervisor se usara en el correo de practica.
					</p>
				</div>
				<button
					type="button"
					onclick={addEvaluation}
					class="inline-flex items-center gap-1 rounded-md bg-green-50 px-3 py-1.5 text-sm font-medium text-green-700 hover:bg-green-100"
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

					<div class="w-44">
						<div>
							<label
								for="eval-supervisor-{index}"
								class="block text-xs font-medium text-gray-700"
							>
								Evaluacion supervisor
							</label>
							<div class="mt-2 flex items-center gap-2">
								<input
									id="eval-supervisor-{index}"
									type="radio"
									name="supervisorEvaluation"
									checked={supervisorEvaluationIndex === index}
									onchange={() => (supervisorEvaluationIndex = index)}
								/>
								<span class="text-xs text-gray-600">Usar esta</span>
							</div>
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
			<div class="flex justify-end text-sm">
				<span
					class="font-medium"
					class:text-green-600={totalWeight === 100}
					class:text-red-600={totalWeight !== 100}
				>
					Total: {totalWeight}%
					{#if totalWeight !== 100}
						(debe sumar 100%)
					{/if}
				</span>
			</div>
		</div>

		<div class="flex justify-end gap-2 pt-4">
			<button
				type="submit"
				disabled={form.isSubmitting}
				class="rounded bg-blue-600 px-4 py-2 text-white hover:bg-blue-700 disabled:opacity-50"
			>
				{form.isSubmitting ? "Creando..." : "Crear Curso"}
			</button>
		</div>
	</div>
</form>
