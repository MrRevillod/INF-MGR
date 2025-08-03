<script lang="ts">
	import { PlusIcon, BookOpenIcon } from "@fvilers/heroicons-svelte/20/solid"
</script>

<div class="space-y-6">
	<!-- Page Header -->
	<div class="border-border border-b pb-6">
		<h1 class="text-text-primary text-2xl font-semibold">Asignaturas</h1>
		<p class="text-text-muted mt-2 text-sm">
			Gestiona las asignaturas y materias del sistema académico
		</p>
	</div>

	<!-- Empty State (temporal mientras no hay datos) -->
	<div class="bg-surface border-border rounded-lg border">
		<div class="px-6 py-12 text-center">
			<BookOpenIcon class="text-text-muted mx-auto h-12 w-12" />
			<h3 class="text-text-primary mt-4 text-lg font-medium">No hay asignaturas</h3>
			<p class="text-text-muted mt-2 text-sm">
				Comienza agregando la primera asignatura al sistema
			</p>
			<div class="mt-6">
				<button
					class="bg-accent hover:bg-accent-hover focus:ring-focus-ring inline-flex items-center gap-2 rounded-lg px-4
					       py-2.5 text-sm font-medium
					       text-white transition-all duration-200
					       focus:outline-none focus:ring-2"
				>
					<PlusIcon class="h-4 w-4" />
					Agregar asignatura
				</button>
			</div>
		</div>
	</div>
</div>
