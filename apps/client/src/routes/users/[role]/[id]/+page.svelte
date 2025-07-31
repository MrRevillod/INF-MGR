<script lang="ts">
	import { page } from "$app/state"
	import { useQuery } from "$lib/shared/hooks/useQuery"
	import { formatRoles } from "$users/utils"
	import { getStudentInscriptionsQuery, getUserQuery } from "$users/querys"

	import PageTitle from "$lib/components/ui/PageTitle.svelte"
	import AsignatureCard from "$lib/features/components/Card.svelte"
	import UpdateUserForm from "$lib/features/users/components/UpdateUserForm.svelte"

	const userId = $derived(page.params.id ?? "")

	const { data: user } = $derived(useQuery(() => getUserQuery(userId)))
	const { data: inscriptions, isLoading } = $derived(
		useQuery(() => getStudentInscriptionsQuery(userId))
	)
</script>

<section class="space-y-6">
	<PageTitle
		title={`Perfil de ${$user?.name ?? "Cargando..."}`}
		description={`Tipo de usuario: ${formatRoles($user?.roles ?? [])}`}
	/>

	<section class="flex w-full flex-row items-start justify-between gap-12">
		<section class="flex w-1/2 flex-col gap-8">
			<div>
				<h2 class="text-text-primary text-lg font-semibold">Información</h2>
			</div>

			<UpdateUserForm user={$user} />
		</section>

		<section class="flex w-1/2 flex-col gap-4">
			<div>
				<h2 class="text-text-primary text-lg font-semibold">Asignaturas</h2>
			</div>

			<div>
				{#if $isLoading}
					<p>Cargando asignaturas...</p>
				{:else}
					<ul class="flex w-5/6 list-none flex-col gap-2">
						{#each $inscriptions ?? [] as inscription}
							<AsignatureCard
								code={inscription.asignature.code}
								name={inscription.asignature.name}
								year={inscription.asignature.year}
							/>
						{/each}
					</ul>
				{/if}
			</div>
		</section>
	</section>
</section>
