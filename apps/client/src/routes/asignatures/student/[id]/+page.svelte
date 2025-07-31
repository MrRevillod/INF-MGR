<script lang="ts">
	import type { PageProps } from "./$types"

	import PageTitle from "$lib/components/ui/PageTitle.svelte"
	import StudentCalifications from "$lib/features/asignatures/components/StudentCalifications.svelte"
	import Acordion from "$lib/components/ui/Acordion.svelte"

	const { data: page }: PageProps = $props()
	const { asignature, inscription } = page

	// get the user reports with user.id AND inscription.id
</script>

<section class="space-y-6">
	<PageTitle
		title={`${page.asignature?.name} - ${page.user?.name ?? "Cargando..."}`}
		description={`${page.asignature?.code} - ${page.asignature?.year}`}
	/>

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
</section>
