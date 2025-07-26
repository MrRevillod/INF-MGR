<script lang="ts">
	import { page } from "$app/state"
	import { goto } from "$app/navigation"
	import { getUserQuery } from "$lib/api/users"
	import { tableColumns, actions } from "$lib/utils/users"

	import Table from "$components/Table/Table.svelte"
	import SearchBar from "$components/SearchBar.svelte"
	import PageTitle from "$components/PageTitle.svelte"
	import Button from "$components/Button.svelte"

	const role = $derived(page.params.role ?? "students")
	const search = $derived(page.url.searchParams.get("search") ?? "")

	function onQueryChange(value: string) {
		const encoded = encodeURIComponent(value)
		goto(`/users/${role}?search=${encoded}`, {
			keepFocus: true,
			noScroll: true,
		})
	}

	const query = $derived(getUserQuery(role, search))

	const { data: response, isLoading, isError, refetch } = $derived($query)
</script>

<div class="space-y-6">
	<PageTitle
		description="Gestiona y visualiza la información de los usuarios del sistema"
	/>

	<div class="flex items-center justify-between gap-4">
		<div class="max-w-md flex-1">
			<SearchBar
				{search}
				placeholder="Buscar usuarios..."
				onSearchChange={onQueryChange}
			/>
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
		data={response?.data}
		columns={tableColumns}
		{actions}
		{isError}
		{isLoading}
		onPageChange={onQueryChange}
	/>
</div>
