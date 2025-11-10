<script lang="ts">
	import { createForm, Form, Field, validate } from "@formisch/svelte"
	import { CreateUserSchema } from "$users/schemas"
	import { createUserMutation } from "$users/mutations"
	import { toast } from "svelte-sonner"

	interface Props {
		onSuccess: () => void
	}

	let { onSuccess }: Props = $props()

	const mutation = createUserMutation()

	const form = createForm({
		schema: CreateUserSchema,
		initialInput: {
			rut: "",
			name: "",
			email: "",
			role: "student",
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
				toast.success("Usuario creado exitosamente")
				onSuccess()
			} else {
				toast.error(
					typeof apiResult.error === "string"
						? apiResult.error
						: "Error al crear el usuario"
				)
			}
		} catch (error) {
			toast.error("Error al crear el usuario")
		}
	}
</script>

<Form of={form} onsubmit={onSubmit}>
	<div class="space-y-4">
		<Field of={form} path={["rut"]}>
			{#snippet children(field)}
				<div>
					<label for="rut" class="block text-sm font-medium text-gray-700">RUT</label
					>
					<input
						id="rut"
						type="text"
						value={field.input ?? ""}
						{...field.props}
						placeholder="12345678-5"
						class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
					/>
					{#if field.errors}
						<p class="mt-1 text-sm text-red-600">{field.errors[0]}</p>
					{/if}
				</div>
			{/snippet}
		</Field>

		<Field of={form} path={["name"]}>
			{#snippet children(field)}
				<div>
					<label for="name" class="block text-sm font-medium text-gray-700"
						>Nombre Completo</label
					>
					<input
						id="name"
						type="text"
						value={field.input ?? ""}
						{...field.props}
						placeholder="Juan Pérez"
						class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
					/>
					{#if field.errors}
						<p class="mt-1 text-sm text-red-600">{field.errors[0]}</p>
					{/if}
				</div>
			{/snippet}
		</Field>

		<Field of={form} path={["email"]}>
			{#snippet children(field)}
				<div>
					<label for="email" class="block text-sm font-medium text-gray-700"
						>Email</label
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
				{form.isSubmitting ? "Creando..." : "Crear Usuario"}
			</button>
		</div>
	</div>
</Form>
