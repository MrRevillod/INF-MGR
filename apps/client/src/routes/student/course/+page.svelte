<script lang="ts">
	import type { PageData } from "./$types"

	import Acordion from "$lib/components/ui/Acordion.svelte"
	import PageTitle from "$lib/components/ui/PageTitle.svelte"
	import StudentCalifications from "$lib/features/courses/components/StudentScores.svelte"

	const { inscription, user }: PageData = $props()
</script>

<section class="space-y-6">
	<PageTitle
		title={`${inscription.course?.name} - ${user?.name ?? "Cargando..."}`}
		description={`${inscription.course?.code} - ${inscription.course?.year}`}
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

			<StudentCalifications {inscription} course={inscription.course} />
		</section>

		<section class="flex w-full flex-col gap-8">
			<h2 class="text-text-primary text-lg font-semibold">Bitácoras</h2>
			<div class="flex w-full flex-col gap-4">
				<Acordion title="Bitácora 1" description="Descripción de la bitácora 1" />
			</div>
		</section>
	</section>
</section>
