<script lang="ts">
	import { goto } from "$app/navigation"
	import { tableColumns } from "$users/utils"
	import { getUsersQuery } from "$lib/users/querys"
	import { useEncodeData } from "$lib/shared/hooks/useUrlData"

	import Table from "$lib/shared/components/Table.svelte"
	import Button from "$lib/shared/components/ui/Button.svelte"
	import SearchBar from "$lib/shared/components/SearchBar.svelte"
	import PageTitle from "$lib/shared/components/ui/PageTitle.svelte"
	import Modal from "$lib/shared/components/Modal.svelte"
	import CreateUserForm from "$lib/users/components/CreateUserForm.svelte"

	let search = $state("")
	let currentPage = $state(1)
	let isModalOpen = $state(false)

	const query = getUsersQuery(() => ({ search, page: currentPage }))

	const { data: res, isError, isLoading, refetch } = $derived(query)

	const paginationProps = $derived({
		currentPage,
		totalPages: res?.data?.totalPages ?? 1,
		totalUsers: res?.data?.totalUsers ?? 0,
		hasNext: res?.data?.hasNext ?? false,
		hasPrevious: res?.data?.hasPrevious ?? false,
		onPageChange: (page: number) => (currentPage = page),
	})

	function handleCloseModal() {
		isModalOpen = false
	}

	function handleSuccess() {
		isModalOpen = false
		refetch()
	}

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
				onclick={() => refetch()}
				variant="secondary"
				disabled={isLoading}
				text={isLoading ? "Cargando..." : "Actualizar"}
			/>

			<Button
				onclick={() => (isModalOpen = true)}
				variant="primary"
				text="Nuevo usuario"
			/>
		</div>
	</div>

	<Table
		data={res?.data?.users ?? []}
		columns={tableColumns}
		pagination={paginationProps}
		onDetailsClick={item =>
			goto(`/admin/users/${item.id}?${useEncodeData({ user: item })}`)}
		{isError}
		{isLoading}
	/>
</div>

<Modal
	bind:isOpen={isModalOpen}
	onClose={handleCloseModal}
	title="Crear Nuevo Usuario"
>
	<CreateUserForm onSuccess={handleSuccess} />
</Modal>
