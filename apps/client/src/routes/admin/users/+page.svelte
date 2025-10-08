<script lang="ts">
	import { goto } from "$app/navigation"
	import { tableColumns } from "$users/utils"
	import { getUsersQuery } from "$lib/users/querys.svelte"
	import { useEncodeData } from "$lib/shared/hooks/useUrlData"

	import Table from "$lib/shared/components/Table.svelte"
	import Button from "$lib/shared/components/ui/Button.svelte"
	import SearchBar from "$lib/shared/components/SearchBar.svelte"
	import PageTitle from "$lib/shared/components/ui/PageTitle.svelte"

	let search = $state("")
	let currentPage = $state(1)

	const query = getUsersQuery(() => ({ search, page: currentPage }))

	const paginationProps = $derived({
		currentPage,
		totalPages: query.data?.data?.totalPages ?? 1,
		totalUsers: query.data?.data?.totalUsers ?? 0,
		hasNext: query.data?.data?.hasNext ?? false,
		hasPrevious: query.data?.data?.hasPrevious ?? false,
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
				onclick={() => query.refetch()}
				variant="secondary"
				disabled={query.isLoading}
				text={query.isLoading ? "Cargando..." : "Actualizar"}
			/>

			<Button onclick={() => {}} variant="primary" text="Nuevo usuario" />
		</div>
	</div>

	<Table
		data={query.data?.data?.users ?? []}
		columns={tableColumns}
		pagination={paginationProps}
		onDetailsClick={item =>
			goto(`/users/${item.id}?${useEncodeData({ user: item })}`)}
		isError={query.isError}
		isLoading={query.isLoading}
	/>
</div>
