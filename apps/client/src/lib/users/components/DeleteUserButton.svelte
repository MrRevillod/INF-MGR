<!-- apps/client/src/lib/users/components/DeleteUserButton.svelte -->
<script lang="ts">
	import { goto } from "$app/navigation"
	import { deleteUserMutation } from "$users/mutations"
	import type { User } from "$users/schemas"

	interface Props {
		user: User
	}

	let { user }: Props = $props()

	const mutation = deleteUserMutation(user.id)

	let showConfirm = $state(false)
	let errorMessage = $state("")

	function handleDelete() {
		errorMessage = ""

		mutation.mutate(undefined, {
			onSuccess: () => {
				goto("/admin/users")
			},
			onError: (error: any) => {
				// Manejar el error 500 del backend
				if (error?.status === 500 || error?.message?.includes("foreign key")) {
					errorMessage =
						"No se puede eliminar este usuario porque tiene inscripciones activas en cursos."
				} else {
					errorMessage =
						error?.message || "Error al eliminar el usuario. Intenta de nuevo."
				}
				showConfirm = false
			},
		})
	}
</script>

<div class="danger-zone">
	<h3>Eliminar Usuario</h3>
	<p>Esta acción eliminará permanentemente a <strong>{user.name}</strong>.</p>

	{#if errorMessage}
		<div class="alert-error">
			{errorMessage}
		</div>
	{/if}

	{#if !showConfirm}
		<button class="btn-danger" onclick={() => (showConfirm = true)}>
			Eliminar Usuario
		</button>
	{:else}
		<div class="confirm-actions">
			<p class="confirm-text">¿Estás seguro?</p>
			<button class="btn-cancel" onclick={() => (showConfirm = false)}>
				Cancelar
			</button>
			<button
				class="btn-confirm"
				onclick={handleDelete}
				disabled={mutation.isPending}
			>
				{mutation.isPending ? "Eliminando..." : "Sí, eliminar"}
			</button>
		</div>
	{/if}
</div>

<style>
	.danger-zone {
		margin-top: 2rem;
		padding: 1.5rem;
		border: 2px solid #ef4444;
		border-radius: 8px;
		background-color: #fef2f2;
	}

	h3 {
		margin: 0 0 0.5rem 0;
		color: #dc2626;
		font-size: 1rem;
		font-weight: 600;
	}

	p {
		margin: 0 0 1rem 0;
		color: #6b7280;
		font-size: 0.875rem;
	}

	strong {
		color: #dc2626;
	}

	.alert-error {
		margin-bottom: 1rem;
		padding: 0.75rem 1rem;
		background-color: #fee2e2;
		border: 1px solid #fca5a5;
		border-radius: 6px;
		color: #991b1b;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.btn-danger {
		padding: 0.5rem 1rem;
		background-color: #dc2626;
		color: white;
		border: none;
		border-radius: 6px;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.btn-danger:hover {
		background-color: #b91c1c;
	}

	.confirm-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.confirm-text {
		margin: 0;
		color: #dc2626;
		font-weight: 600;
	}

	.btn-cancel,
	.btn-confirm {
		padding: 0.5rem 1rem;
		border: none;
		border-radius: 6px;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.btn-cancel {
		background-color: #6b7280;
		color: white;
	}

	.btn-cancel:hover {
		background-color: #4b5563;
	}

	.btn-confirm {
		background-color: #dc2626;
		color: white;
	}

	.btn-confirm:hover:not(:disabled) {
		background-color: #b91c1c;
	}

	button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
