<script lang="ts">
	import type { Course, Inscription } from "../schemas"

	import { PencilSquareIcon } from "@fvilers/heroicons-svelte/20/solid"

	interface Props {
		inscription: Inscription
		course: Course
	}

	const { inscription, course }: Props = $props()

	const califications = $derived.by(() => {
		return course?.evaluations.map(evaluation => {
			const studentScore = inscription?.studentScores?.find(
				score => score.id === evaluation.id
			)

			return {
				...evaluation,
				score: studentScore?.score ?? 1.0,
			}
		})
	})
</script>

<section class="flex w-3/5 flex-col gap-8">
	<h2 class="text-text-primary text-lg font-semibold">Notas</h2>

	<table class="w-full table-auto">
		<thead>
			<tr>
				<th class="text-left">Evaluación</th>
				<th class="text-left">Ponderación</th>
				<th class="text-left">Nota</th>
				<th class="text-left">Editar</th>
			</tr>
		</thead>
		<tbody class="divide-border divide-y">
			{#each califications as calification (calification.id)}
				<tr>
					<td class="py-2">{calification.name}</td>
					<td class="py-2">{calification.weight}%</td>
					<td class="py-2">{calification.score.toFixed(2)}</td>
					<td class="py-2">
						<button
							type="button"
							class="text-text-muted hover:text-text-primary hover:bg-hover-bg
										focus:ring-focus-ring inline-flex h-8 w-8 cursor-pointer items-center
										justify-center rounded-md transition-colors duration-200 focus:outline-none focus:ring-2"
							aria-label="Editar nota"
						>
							<PencilSquareIcon class="h-5 w-5" />
						</button>
					</td>
				</tr>
			{/each}

			<tr>
				<td colspan="3" class="text-right font-semibold"> Promedio: &nbsp;</td>

				<td class="py-2">
					{(
						califications.reduce((sum, c) => sum + c.score, 0) / califications.length
					).toFixed(2)}
				</td>
			</tr>

			{#if califications.length === 0}
				<tr>
					<td colspan="4" class="text-text-muted py-2 text-center">
						No hay notas registradas.
					</td>
				</tr>
			{/if}
		</tbody>
	</table>
</section>
