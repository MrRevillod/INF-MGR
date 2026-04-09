<script lang="ts">
	import type { PageProps } from "./$types"

	let { data }: PageProps = $props()

	let scoreInput = $state("")
	let submitting = $state(false)
	let success = $state(false)
	let errorMessage = $state("")

	function parseAndValidateScore(raw: string): number | null {
		const value = Number.parseFloat(raw)

		if (Number.isNaN(value)) {
			errorMessage = "Debe ingresar una nota valida."
			return null
		}

		if (value < 1 || value > 7) {
			errorMessage = "La nota debe estar entre 1.0 y 7.0."
			return null
		}

		errorMessage = ""
		return value
	}

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault()

		const score = parseAndValidateScore(scoreInput)
		if (score === null) return

		submitting = true
		errorMessage = ""

		try {
			const response = await fetch(
				`/api/enrollments/${data.enrollmentId}/practice/${data.practiceId}/evaluate/${data.evaluationId}`,
				{
					method: "POST",
					headers: {
						"Content-Type": "application/json",
					},
					body: JSON.stringify({ score }),
				}
			)

			if (!response.ok) {
				errorMessage =
					"No se pudo enviar la evaluacion. Intente nuevamente o contacte a secretaria."
				return
			}

			success = true
		} catch (_error) {
			errorMessage = "No se pudo enviar la evaluacion en este momento."
		} finally {
			submitting = false
		}
	}
</script>

<svelte:head>
	<title>Evaluación de Práctica</title>
</svelte:head>

<div class="flex min-h-screen items-center justify-center bg-gray-50 px-4 py-12">
	<div class="w-full max-w-md rounded-lg bg-white p-8 shadow-lg">
		{#if success}
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
				<h1 class="mb-2 text-2xl font-bold text-gray-900">Evaluacion Enviada</h1>
				<p class="text-gray-600">La evaluacion ha sido enviada correctamente.</p>
				<p class="mt-4 text-sm text-gray-500">Puede cerrar esta ventana.</p>
			</div>
		{:else}
			<div>
				<h1 class="mb-3 text-2xl font-bold text-gray-900">Evaluacion de Practica</h1>
				<p class="mb-6 text-gray-600">
					Ingrese la nota final del estudiante en escala 1.0 a 7.0.
				</p>

				<form onsubmit={handleSubmit} class="space-y-4">
					<div>
						<label for="score" class="mb-2 block text-sm font-medium text-gray-700">
							Nota final
						</label>
						<input
							id="score"
							type="number"
							step="0.1"
							min="1"
							max="7"
							placeholder="Ej: 5.5"
							bind:value={scoreInput}
							class="w-full rounded-md border border-gray-300 p-2 text-sm"
						/>
						<p class="mt-2 text-xs text-gray-500">Use punto decimal. Ejemplo: 5.5</p>
					</div>

					{#if errorMessage}
						<div class="rounded-md bg-red-50 p-3 text-sm text-red-700">
							{errorMessage}
						</div>
					{/if}

					<button
						type="submit"
						disabled={submitting}
						class="w-full rounded-md bg-blue-600 px-4 py-2 font-semibold text-white hover:bg-blue-700 disabled:cursor-not-allowed disabled:bg-gray-300"
					>
						{submitting ? "Enviando..." : "Enviar Evaluacion"}
					</button>
				</form>
			</div>
		{/if}
	</div>
</div>
