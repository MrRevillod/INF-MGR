<script lang="ts">
	import { goto } from "$app/navigation"
	import { page } from "$app/state"
	import { getUsers } from "$lib/api/users"

	import { createQuery } from "@tanstack/svelte-query"

	const role = $derived(page.params.role)
	const search = $derived(page.url.searchParams.get("search") ?? "")

	function onSearchChange(e: Event) {
		const input = e.target as HTMLInputElement
		const searchValue = input.value.trim().toLocaleLowerCase()

		goto(`/users/${role}?search=${encodeURIComponent(searchValue)}`, {
			keepFocus: true,
			noScroll: true,
		})
	}

	let query = $derived(
		createQuery({
			queryKey: ["users", role, search],
			queryFn: () => getUsers({ role, search }),
			enabled: !!role,
			staleTime: 1000 * 60 * 1,
		})
	)
</script>

<main>
	<header>
		<h1>Usuarios del sistema</h1>
	</header>

	<section class="filters">
		<input
			type="text"
			value={search}
			oninput={onSearchChange}
			placeholder="Buscar por nombre, correo o RUT"
		/>
	</section>

	<section class="table">
		<table>
			<thead>
				<tr>
					<th>RUT</th>
					<th>Nombre</th>
					<th>Correo electrónico</th>
					<th>Roles</th>
				</tr>
			</thead>
			<tbody>
				{#each $query?.data?.data ?? [] as user (user.rut)}
					<tr>
						<td>{user.rut}</td>
						<td>{user.name}</td>
						<td>{user.email}</td>
						<td
							>[
							{#if Array.isArray(user.roles) && user.roles.length > 0}
								{#each user.roles as userRole (userRole)}
									<span class="role">{userRole}</span>
								{/each}
							{:else}
								<span class="role">Sin roles</span>
							{/if}]
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</section>
</main>

<style>
	main {
		width: 100%;
		display: flex;
		flex-direction: column;
	}

	.table {
		width: 100%;
		margin-top: 1.5rem;
		margin-bottom: 2rem;
		margin-left: 0;
		align-self: stretch;
	}

	table {
		width: 100%;
		border-collapse: separate;
		border-spacing: 0 0.5rem;
		font-size: 1rem;
	}

	thead tr {
		text-align: left;
	}

	thead th {
		padding: 0.75rem 1.25rem 0.5rem 0.5rem;
		font-weight: 600;
		text-align: start;
		letter-spacing: 0.02em;
	}

	tbody td {
		padding: 0.75rem 1.25rem 0.75rem 0.5rem;
		vertical-align: top;
	}

	tr {
		background: none;
		border-radius: 0.5rem;
	}

	tr:not(:last-child) td {
		border-bottom: 1px solid #e0e0e0;
	}

	.role {
		display: inline-block;
		margin-right: 0.5rem;
		padding: 0.15em 0.7em;
		border-radius: 0.4em;
		font-size: 0.95em;
		background: #f5f5f5;
	}
</style>
