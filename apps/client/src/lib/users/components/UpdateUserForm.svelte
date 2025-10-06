<script lang="ts">
	import type { User } from "$users/schemas"

	import { useMutation } from "$lib/shared/hooks/useTanstack"
	import { RutFormatter } from "../utils"
	import { updateUserMutation } from "../mutations"

	import Field from "$lib/shared/components/ui/Field.svelte"

	const { user }: { user: User | null } = $props()

	const defaultData = $derived({
		email: user?.email,
		roles: user?.role,
		password: "",
		confirmPassword: "",
	})

	const { mutate } = useMutation<User>(() =>
		updateUserMutation(user?.id ?? "", defaultData)
	)

	const onSubmit = (data: Record<string, unknown>) => {
		console.log("Updated data:", data)
		mutate(data, {
			onSuccess: () => {
				console.log("User updated successfully")
			},
			onError: error => {
				console.error("Error updating user:", error)
			},
		})
	}
</script>

<div class="text-text-muted flex w-5/6 flex-col gap-6 text-base">
	<Field label="ID" value={user?.id ?? ""} />
	<Field label="RUT" value={RutFormatter(user?.rut ?? "")} />
	<Field label="Nombre" value={user?.name ?? ""} />

	<Field label="Correo electrónico" value={user?.email ?? ""} />
	<Field label="Rol" value={user?.role ?? "Sin rol"} />

	<Field label="Contraseña" value="********" />

	<button
		class="bg-primary hover:bg-primary-dark rounded px-4 py-2 text-white"
		onclick={() => onSubmit({})}
	>
		Actualizar Usuario
	</button>
</div>
