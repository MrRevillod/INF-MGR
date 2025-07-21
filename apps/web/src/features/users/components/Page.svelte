<script lang="ts">
	import RoleSelector from "./RoleSelector.svelte"
	import Searchbar from "../../../shared/components/Searchbar.svelte"

	import type { User } from "../entities.svelte"
	import { Request } from "../../../shared/http/request.svelte"
	import { PlusIcon } from "lucide-svelte"

	let search = $state("")
	let query = $state({ roles: ["student"] })

	const action = new Request<User[]>({
		url: "/users",
		query,
	})

	const roleToSpanish = {
		student: "Estudiante",
		teacher: "Profesor (a)",
		administrator: "Administrador (a)",
		coordinator: "Coordinador (a)",
		secretary: "Secretario (a)",
	}

	let { data: users } = $derived(action)
</script>

<main>
	<header>
		<h1>Usuarios del sistema</h1>
	</header>

	<section class="filters">
		<Searchbar
			bind:search
			label="Buscar por RUT, nombre o correo electrónico"
			data={users ?? []}
		/>

		<RoleSelector bind:roles={query.roles} />

		<section class="add-user-section">
			<label for="add-user-btn">Nuevo</label>
			<button id="add-user-btn">
				<PlusIcon />
			</button>
		</section>
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
				{#each users ?? [] as user (user.rut)}
					<tr>
						<td>{user.rut}</td>
						<td>{user.name}</td>
						<td>{user.email}</td>
						<td
							>[
							{#if Array.isArray(user.roles) && user.roles.length > 0}
								{#each user.roles as role (role)}
									<span class="role"
										>{roleToSpanish[role]}</span
									>
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

	.filters {
		display: flex;
		gap: 5%;
		align-items: flex-end;
		margin-bottom: 1.5rem;
		width: 100%;
		justify-content: space-between;
		align-items: start;
	}

	.add-user-section {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		width: 10%;
		height: 100%;
	}

	.add-user-section label {
		font-weight: 500;
		margin-bottom: 0.25rem;
		font-size: 1rem;
		color: #222;
	}

	.add-user-section button {
		padding: 0.6em 1em;
		width: 100%;
		height: 44px;
		border-radius: 0.4em;
		border: 1px solid #e0e0e0;
		background: #f5f5f5;
		font-size: 1rem;
		transition: border 0.2s;
		box-sizing: border-box;
	}
</style>
