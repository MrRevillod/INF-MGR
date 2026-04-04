<script lang="ts">
	import { createForm, Form, Field, validate } from "@formisch/svelte"
	import { updateEnrollmentMutation } from "../mutations"
	import {
		UpdateEnrollmentSchema,
		type Evaluation,
		type StudentScore,
	} from "../schemas"
	import type { Enrollment } from "$lib/enrollments/schemas"

	interface Props {
		enrollment: Enrollment
		evaluations: Evaluation[]
		onSuccess?: () => void
		onCancel?: () => void
	}

	let { enrollment, evaluations, onSuccess, onCancel }: Props = $props()

	const mutation = updateEnrollmentMutation(enrollment.id)

	// Inicializar los scores con los valores actuales o vacíos
	const initialScores: StudentScore[] = evaluations.map(evaluation => {
		const existingScore = enrollment.studentScores?.find(
			s => s.evaluationId === evaluation.id
		)
		return {
			evaluationId: evaluation.id,
			score:
				existingScore?.score && existingScore.score > 0 ? existingScore.score : 1.0,
		}
	})

	const form = createForm({
		schema: UpdateEnrollmentSchema,
		initialInput: {
			studentScores: initialScores,
			practiceId: enrollment.practiceId,
		},
	})

	let errorMessage = $state("")
	let successMessage = $state("")

	async function handleSubmit() {
		errorMessage = ""
		successMessage = ""

		const result = await validate(form)

		if (!result.success) {
			errorMessage = "Por favor, corrija los errores en el formulario"
			return
		}

		const values = result.output

		// Filtrar solo los scores que tienen un valor >= 1.0 (respetando validación del backend)
		const validScores =
			values.studentScores?.filter((s: any) => s.score >= 1.0) ?? []

		const payload = {
			studentScores: validScores.length > 0 ? validScores : undefined,
			practiceId: values.practiceId || undefined,
		}

		mutation.mutate(payload, {
			onSuccess: () => {
				successMessage = "Inscripción actualizada exitosamente"
				if (onSuccess) {
					setTimeout(onSuccess, 1500)
				}
			},
			onError: (error: any) => {
				errorMessage = error?.message || "Error al actualizar la inscripción"
			},
		})
	}
</script>

<div class="update-enrollment-form">
	<h3>Actualizar Calificaciones</h3>

	<Form of={form} onsubmit={handleSubmit}>
		<div class="evaluations-section">
			<h4>Calificaciones por Evaluación</h4>

			{#each evaluations as evaluation, index}
				<Field of={form} path={["studentScores", index, "score"]}>
					{#snippet children(field)}
						<div class="evaluation-field">
							<label for={`score-${evaluation.id}`}>
								{evaluation.name}
								<span class="weight">(Peso: {evaluation.weight}%)</span>
							</label>

							<input
								id={`score-${evaluation.id}`}
								type="number"
								min="1.0"
								max="7.0"
								step="0.1"
								value={field.input ?? 0}
								{...field.props}
								class:error={field.errors && field.errors.length > 0}
							/>

							{#if field.errors && field.errors.length > 0}
								<span class="error-message">{field.errors[0]}</span>
							{/if}
						</div>
					{/snippet}
				</Field>
			{/each}
		</div>

		{#if errorMessage}
			<div class="alert error">
				{errorMessage}
			</div>
		{/if}

		{#if successMessage}
			<div class="alert success">
				{successMessage}
			</div>
		{/if}

		<div class="form-actions">
			{#if onCancel}
				<button type="button" class="btn-secondary" onclick={onCancel}>
					Cancelar
				</button>
			{/if}

			<button type="submit" class="btn-primary" disabled={mutation.isPending}>
				{mutation.isPending ? "Guardando..." : "Guardar Calificaciones"}
			</button>
		</div>
	</Form>
</div>

<style>
	.update-enrollment-form {
		background: white;
		padding: 2rem;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	h3 {
		margin: 0 0 1.5rem 0;
		font-size: 1.5rem;
		color: #333;
	}

	h4 {
		margin: 0 0 1rem 0;
		font-size: 1.1rem;
		color: #555;
	}

	.evaluations-section {
		margin-bottom: 1.5rem;
	}

	.evaluation-field {
		margin-bottom: 1rem;
	}

	label {
		display: block;
		margin-bottom: 0.5rem;
		font-weight: 500;
		color: #333;
	}

	.weight {
		font-size: 0.875rem;
		color: #666;
		font-weight: normal;
		margin-left: 0.5rem;
	}

	input[type="number"] {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #ddd;
		border-radius: 4px;
		font-size: 1rem;
		transition: border-color 0.2s;
	}

	input:focus {
		outline: none;
		border-color: #4a90e2;
	}

	input.error {
		border-color: #e74c3c;
	}

	.error-message {
		display: block;
		color: #e74c3c;
		font-size: 0.875rem;
		margin-top: 0.25rem;
	}

	.alert {
		padding: 1rem;
		border-radius: 4px;
		margin-bottom: 1rem;
	}

	.alert.error {
		background-color: #fee;
		color: #c33;
		border: 1px solid #fcc;
	}

	.alert.success {
		background-color: #efe;
		color: #3c3;
		border: 1px solid #cfc;
	}

	.form-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-end;
		margin-top: 1.5rem;
	}

	button {
		padding: 0.75rem 1.5rem;
		border: none;
		border-radius: 4px;
		font-size: 1rem;
		cursor: pointer;
		transition: background-color 0.2s;
	}

	.btn-primary {
		background-color: #4a90e2;
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		background-color: #357abd;
	}

	.btn-secondary {
		background-color: #95a5a6;
		color: white;
	}

	.btn-secondary:hover {
		background-color: #7f8c8d;
	}

	button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
</style>
