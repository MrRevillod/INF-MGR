<script lang="ts">
	import { createForm, Form, Field, validate } from "@formisch/svelte"
	import { UpdateUserSchema, type User } from "$users/schemas"
	import { updateUserMutation } from "$users/mutations"
	import { toast } from "svelte-sonner"
	import { RutFormatter } from "$users/utils"

	interface Props {
		user: User | null
		onSuccess?: () => void
	}

	let { user, onSuccess }: Props = $props()

	const mutation = updateUserMutation(user?.id ?? "")

	const form = createForm({
		schema: UpdateUserSchema,
		initialInput: {
			email: user?.email,
			role: user?.role,
		},
	})

	async function onSubmit() {
		const result = await validate(form)

		if (!result.success) {
			toast.error("Por favor corrige los errores del formulario")
			return
		}

		try {
			const apiResult = await mutation.mutateAsync(result.output)

			if (apiResult.success) {
				toast.success("Usuario actualizado exitosamente")
				onSuccess?.()
			} else {
				toast.error(
					typeof apiResult.error === "string"
						? apiResult.error
						: "Error al actualizar el usuario"
				)
			}
		} catch (error) {
			toast.error("Error al actualizar el usuario")
		}
	}
</script>

<div class="flex w-full flex-col gap-6">
	<!-- Campos de solo lectura -->
	<div>
		<label for="user-id" class="block text-sm font-medium text-gray-700">ID</label>
		<input
			id="user-id"
			type="text"
			value={user?.id ?? ""}
			disabled
			class="mt-1 block w-full rounded-md border border-gray-300 bg-gray-100 px-3 py-2 text-gray-500"
		/>
	</div>

	<div>
		<label for="user-rut" class="block text-sm font-medium text-gray-700">RUT</label>
		<input
			id="user-rut"
			type="text"
			value={RutFormatter(user?.rut ?? "")}
			disabled
			class="mt-1 block w-full rounded-md border border-gray-300 bg-gray-100 px-3 py-2 text-gray-500"
		/>
	</div>

	<div>
		<label for="user-name" class="block text-sm font-medium text-gray-700"
			>Nombre</label
		>
		<input
			id="user-name"
			type="text"
			value={user?.name ?? ""}
			disabled
			class="mt-1 block w-full rounded-md border border-gray-300 bg-gray-100 px-3 py-2 text-gray-500"
		/>
	</div>

	<!-- Formulario editable -->
	<Form of={form} onsubmit={onSubmit}>
		<div class="space-y-4">
			<Field of={form} path={["email"]}>
				{#snippet children(field)}
					<div>
						<label for="email" class="block text-sm font-medium text-gray-700"
							>Correo electrónico</label
						>
						<input
							id="email"
							type="email"
							value={field.input ?? ""}
							{...field.props}
							placeholder="correo@ejemplo.com"
							class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
						/>
						{#if field.errors}
							<p class="mt-1 text-sm text-red-600">{field.errors[0]}</p>
						{/if}
					</div>
				{/snippet}
			</Field>

			<Field of={form} path={["role"]}>
				{#snippet children(field)}
					<div>
						<label for="role" class="block text-sm font-medium text-gray-700"
							>Rol</label
						>
						<select
							id="role"
							value={field.input ?? "student"}
							{...field.props}
							class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
						>
							<option value="student">Estudiante</option>
							<option value="teacher">Profesor(a)</option>
							<option value="secretary">Secretario(a)</option>
							<option value="administrator">Administrador(a)</option>
						</select>
						{#if field.errors}
							<p class="mt-1 text-sm text-red-600">{field.errors[0]}</p>
						{/if}
					</div>
				{/snippet}
			</Field>

			<div class="flex justify-end gap-2 pt-4">
				<button
					type="submit"
					disabled={form.isSubmitting}
					class="rounded bg-blue-600 px-4 py-2 text-white hover:bg-blue-700 disabled:opacity-50"
				>
					{form.isSubmitting ? "Actualizando..." : "Actualizar Usuario"}
				</button>
			</div>
		</div>
	</Form>
</div>
