<script lang="ts">
	import { updateEnrollmentMutation } from "../mutations"
	import type { Enrollment } from "$lib/enrollments/schemas"
	import type { Evaluation, StudentScore } from "../schemas"

	interface Props {
		enrollment: Enrollment
		evaluations: Evaluation[]
		onSuccess?: () => void
	}

	let { enrollment, evaluations, onSuccess }: Props = $props()

	const mutation = updateEnrollmentMutation(enrollment.id)

	let isOpen = $state(false)
	let scores = $state<StudentScore[]>([])
	let errorMessage = $state("")

	const openModal = () => {
		// Inicializar scores con valores actuales
		scores = evaluations.map(evaluation => {
			const existingScore = enrollment.studentScores?.find(
				s => s.evaluationId === evaluation.id
			)
			return {
				evaluationId: evaluation.id,
				score: existingScore?.score ?? 0,
			}
		})
		isOpen = true
		errorMessage = ""
	}

	const closeModal = () => {
		isOpen = false
		errorMessage = ""
	}

	const handleSave = () => {
		errorMessage = ""

		// Filtrar solo los scores válidos (>= 1.0, respetando validación del backend)
		const validScores = scores.filter(s => s.score >= 1.0 && s.score <= 7.0)

		// Validar que todos los scores que no son 0 estén en rango válido
		const invalidScores = scores.filter(
			s => s.score > 0 && (s.score < 1.0 || s.score > 7.0)
		)
		if (invalidScores.length > 0) {
			errorMessage = "Las calificaciones deben estar entre 1.0 y 7.0"
			return
		}

		const payload = {
			studentScores: validScores.length > 0 ? validScores : undefined,
		}

		mutation.mutate(payload, {
			onSuccess: () => {
				closeModal()
				if (onSuccess) {
					onSuccess()
				}
			},
			onError: (error: any) => {
				errorMessage = error?.message || "Error al actualizar calificaciones"
			},
		})
	}
</script>

<button class="btn-edit" onclick={openModal} title="Editar calificaciones">
	📝 Calificar
</button>

{#if isOpen}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal-overlay" onclick={closeModal}>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="modal-content" onclick={e => e.stopPropagation()}>
			<div class="modal-header">
				<h3>Actualizar Calificaciones</h3>
				<button class="btn-close" onclick={closeModal}>✕</button>
			</div>

			<div class="modal-body">
				<div class="student-info">
					<p><strong>Estudiante:</strong> {enrollment.student?.name}</p>
					<p><strong>Curso:</strong> {enrollment.course?.name}</p>
				</div>

				<div class="scores-list">
					{#each evaluations as evaluation, index}
						<div class="score-item">
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
								bind:value={scores[index].score}
								placeholder="No calificado"
							/>
						</div>
					{/each}
				</div>

				{#if errorMessage}
					<div class="alert error">
						{errorMessage}
					</div>
				{/if}
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={closeModal}>Cancelar</button>
				<button
					class="btn-primary"
					onclick={handleSave}
					disabled={mutation.isPending}
				>
					{mutation.isPending ? "Guardando..." : "Guardar"}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.btn-edit {
		padding: 0.5rem 1rem;
		background-color: #3498db;
		color: white;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		font-size: 0.875rem;
		transition: background-color 0.2s;
	}

	.btn-edit:hover {
		background-color: #2980b9;
	}

	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal-content {
		background: white;
		border-radius: 8px;
		width: 90%;
		max-width: 600px;
		max-height: 80vh;
		overflow-y: auto;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid #eee;
	}

	.modal-header h3 {
		margin: 0;
		font-size: 1.25rem;
		color: #333;
	}

	.btn-close {
		background: none;
		border: none;
		font-size: 1.5rem;
		cursor: pointer;
		color: #999;
		padding: 0;
		width: 30px;
		height: 30px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 4px;
		transition: background-color 0.2s;
	}

	.btn-close:hover {
		background-color: #f0f0f0;
		color: #333;
	}

	.modal-body {
		padding: 1.5rem;
	}

	.student-info {
		margin-bottom: 1.5rem;
		padding: 1rem;
		background-color: #f8f9fa;
		border-radius: 4px;
	}

	.student-info p {
		margin: 0.5rem 0;
		color: #555;
	}

	.scores-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.score-item {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.score-item label {
		font-weight: 500;
		color: #333;
	}

	.weight {
		font-size: 0.875rem;
		color: #666;
		font-weight: normal;
		margin-left: 0.5rem;
	}

	.score-item input {
		padding: 0.75rem;
		border: 1px solid #ddd;
		border-radius: 4px;
		font-size: 1rem;
	}

	.score-item input:focus {
		outline: none;
		border-color: #3498db;
	}

	.alert {
		padding: 1rem;
		border-radius: 4px;
		margin-top: 1rem;
	}

	.alert.error {
		background-color: #fee;
		color: #c33;
		border: 1px solid #fcc;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		padding: 1.5rem;
		border-top: 1px solid #eee;
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
