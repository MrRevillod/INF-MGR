<script lang="ts">
	import type { User } from "$lib/schemas/user"

	import { page } from "$app/state"
	import { match, P } from "ts-pattern"
	import { PencilSquareIcon } from "@fvilers/heroicons-svelte/20/solid"
	import { formatRoles, RutFormatter } from "$lib/utils/users"
	import { getStudentInscriptionsQuery, getUserQuery } from "$lib/api/users"

	import AsignatureCard from "$components/AsignatureCard.svelte"

	const userId = $derived(page.params.id ?? "")
	const userQuery = $derived(getUserQuery(userId))

	let { data: userResponse } = $derived($userQuery)

	const user = $derived(userResponse?.data as User | undefined)

	const asignaturesQuery = $derived.by(() => {
		const roles = userResponse?.data.roles ?? []

		return match(roles)
			.with(
				P.when(r => r.includes("student")),
				() => getStudentInscriptionsQuery(userId)
			)
			.otherwise(() => getStudentInscriptionsQuery(userId))
	})

	const query = $derived(asignaturesQuery)

	let { data: inscriptionsData, isLoading } = $derived($query)
</script>

<section class="space-y-6">
	<header
		class="border-border flex flex-row items-center justify-between border-b pb-8"
	>
		<div>
			<h1 class="text-text-primary text-2xl font-semibold">
				{user?.name ?? "Cargando..."}
			</h1>
			<p class="text-text-muted mt-2 text-sm">
				Tipo de usuario: {formatRoles(user?.roles ?? [])}
			</p>
		</div>
	</header>

	<section class="flex w-full flex-row items-start justify-between gap-12">
		<section class="flex w-1/2 flex-col gap-8">
			<div>
				<h2 class="text-text-primary text-lg font-semibold">Información</h2>
			</div>

			<div class="text-text-muted flex w-5/6 flex-col gap-6 text-base">
				{@render field("ID", user?.id ?? "")}
				{@render field("RUT", RutFormatter(user?.rut ?? ""))}
				{@render field("Nombre", user?.name ?? "")}
				{@render field("Correo electrónico", user?.email ?? "", true)}
				{@render field("Contraseña", "Campo oculto", true)}
				{@render field("Roles", formatRoles(user?.roles ?? []), true)}
			</div>
		</section>

		<section class="flex w-1/2 flex-col gap-4">
			<div>
				<h2 class="text-text-primary text-lg font-semibold">Asignaturas</h2>
			</div>

			<div>
				{#if isLoading}
					<p>Cargando asignaturas...</p>
				{:else}
					<ul class="flex w-5/6 list-none flex-col gap-2">
						{#each inscriptionsData?.data ?? [] as inscription}
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

{#snippet field(label: string, value: any, editable: boolean = false)}
	<div class="border-border flex w-full flex-row justify-between border-b pb-2">
		<p>
			<strong>{label}:</strong>
			{value}
		</p>
		{#if editable}
			<button class="flex cursor-pointer flex-row gap-2">
				Editar
				<PencilSquareIcon class="text-text-muted h-5 w-5" />
			</button>
		{/if}
	</div>
{/snippet}
