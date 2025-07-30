<script lang="ts">
	import { page } from "$app/state"
	import { tableColumns } from "$lib/utils/users"
	import { getUsersQuery } from "$lib/api/users"

	import Table from "$components/Table/Table.svelte"
	import Button from "$components/Button.svelte"
	import SearchBar from "$components/SearchBar.svelte"
	import PageTitle from "$components/PageTitle.svelte"
	import { goto } from "$app/navigation"
	import { appStore } from "$lib/stores/app.svelte"

	let search = $state("")
	let role = $derived(page.params.role ?? "")

	let currentPage = $state(1)

	const query = $derived(getUsersQuery({ role, search, page: currentPage }))

	const { data: response, isLoading, isError, refetch } = $derived($query)

	const paginationProps = $derived({
		currentPage,
		totalPages: response?.data.totalPages ?? 1,
		totalUsers: response?.data.totalUsers ?? 0,
		hasNext: response?.data.hasNext ?? false,
		hasPrevious: response?.data.hasPrevious ?? false,
		onPageChange: (page: number) => {
			currentPage = page
		},
	})

	$effect(() => {
		if (search.length > 0) {
			currentPage = 1
		}
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
				onclick={() => refetch()}
				variant="secondary"
				disabled={isLoading}
				text={isLoading ? "Cargando..." : "Actualizar"}
			/>

			<Button onclick={() => {}} variant="primary" text="Nuevo usuario" />
		</div>
	</div>

	<Table
		data={response?.data.users ?? []}
		columns={tableColumns}
		{isError}
		{isLoading}
		pagination={paginationProps}
		onDetailsClick={item => {
			goto(`/users/${role}/${item.id}`)
		}}
	/>
</div>
