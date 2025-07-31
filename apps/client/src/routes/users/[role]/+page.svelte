<script lang="ts">
	import { page } from "$app/state"
	import { goto } from "$app/navigation"
	import { useQuery } from "$lib/shared/hooks/useQuery"
	import { tableColumns } from "$users/utils"
	import { getUsersQuery } from "$users/querys"

	import Table from "$lib/components/Table.svelte"
	import Button from "$lib/components/ui/Button.svelte"
	import SearchBar from "$lib/components/SearchBar.svelte"
	import PageTitle from "$lib/components/ui/PageTitle.svelte"

	let search = $state("")
	let currentPage = $state(1)
	let role = $derived(page.params.role ?? "")

	const { data, isLoading, isError, refetch } = $derived(
		useQuery(() => getUsersQuery({ role, search, page: currentPage }))
	)

	const paginationProps = $derived({
		currentPage,
		totalPages: $data?.totalPages ?? 1,
		totalUsers: $data?.totalUsers ?? 0,
		hasNext: $data?.hasNext ?? false,
		hasPrevious: $data?.hasPrevious ?? false,
		onPageChange: (page: number) => (currentPage = page),
	})

	$effect(() => {
		if (search.length > 0) currentPage = 1
	})
</script>

<div class="space-y-6">
	<PageTitle
		description="Gestiona y visualiza la información de los usuarios del sistema"
	/>

	<div class="flex items-center justify-between gap-4">
		<div class="max-w-md flex-1">
			<SearchBar bind:search placeholder="Buscar usuarios..." />
		</div>

		<div class="flex items-center gap-3">
			<Button
				onclick={() => $refetch()}
				variant="secondary"
				disabled={$isLoading}
				text={$isLoading ? "Cargando..." : "Actualizar"}
			/>

			<Button onclick={() => {}} variant="primary" text="Nuevo usuario" />
		</div>
	</div>

	<Table
		data={$data?.users ?? []}
		columns={tableColumns}
		isError={$isError}
		isLoading={$isLoading}
		pagination={paginationProps}
		onDetailsClick={item => {
			goto(`/users/${role}/${item.id}`)
		}}
	/>
</div>
