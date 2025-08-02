<script lang="ts">
	import type { PageProps } from "./$types"

	import { BookOpenIcon, PlusIcon } from "@fvilers/heroicons-svelte/20/solid"

	import Acordion from "$lib/components/ui/Acordion.svelte"
	import PageTitle from "$lib/components/ui/PageTitle.svelte"
	import StudentCalifications from "$lib/features/asignatures/components/StudentCalifications.svelte"

	const { data: page }: PageProps = $props()
	const { asignature, inscription } = page

	// get the user reports with user.id AND inscription.id
</script>

<section class="space-y-6">
	<PageTitle
		title={`${page.asignature?.name} - ${page.user?.name ?? "Cargando..."}`}
		description={`${page.asignature?.code} - ${page.asignature?.year}`}
	/>

	{#if inscription.status === "active"}
		<section class="flex w-full flex-col items-start justify-between gap-12">
			<section class="flex w-full flex-row gap-8">
				<section class="flex w-2/5 flex-col gap-8">
					<h2 class="text-text-primary text-lg font-semibold">Documentos</h2>
					<ul class="list-disc pl-6">
						<li>Documento 1</li>
						<li>Documento 2</li>
						<li>Documento 3</li>
					</ul>
				</section>

				<StudentCalifications {inscription} {asignature} />
			</section>

			<section class="flex w-full flex-col gap-8">
				<h2 class="text-text-primary text-lg font-semibold">Bitácoras</h2>
				<div>
					<Acordion title="Bitácora 1" description="Descripción de la bitácora 1" />
				</div>
			</section>
		</section>
	{/if}

	{#if inscription.status === "inactive"}
		<div class="bg-surface border-border rounded-lg border">
			<div class="px-6 py-12 text-center">
				<BookOpenIcon class="text-text-muted mx-auto h-12 w-12" />
				<h3 class="text-text-primary mt-4 text-lg font-medium">
					La Práctica no está activa
				</h3>
				<p class="text-text-muted mt-2 text-sm">
					Inicia la práctica agregando la empresa y confirmando las fechas de inicio
					y fin
				</p>
				<div class="mt-6">
					<button
						class="bg-accent hover:bg-accent-hover focus:ring-focus-ring inline-flex items-center gap-2 rounded-lg px-4
					       py-2.5 text-sm font-medium
					       text-white transition-all duration-200
					       focus:outline-none focus:ring-2"
					>
						<PlusIcon class="h-4 w-4" />
						Iniciar
					</button>
				</div>
			</div>
		</div>
	{/if}
</section>
