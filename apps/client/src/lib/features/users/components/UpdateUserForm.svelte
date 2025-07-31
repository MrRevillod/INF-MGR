<script lang="ts">
	import type { User } from "$users/schemas"
	import type { Conflicts } from "$lib/shared/api/types"

	import { useMutation } from "$lib/shared/hooks/useMutation"
	import { updateUserMutation } from "../mutations"
	import { RutFormatter, formatRoles } from "../utils"

	import Field from "$lib/components/ui/Field.svelte"

	interface Props {
		user: User | null
	}

	const { user }: Props = $props()

	const defaultData = $derived({
		email: user?.email,
		roles: user?.roles,
		password: "",
		confirmPassword: "",
	})

	const { mutate } = $derived(
		useMutation<User, Conflicts>(() =>
			updateUserMutation(user?.id ?? "", defaultData)
		)
	)

	const onSubmit = (data: Record<string, any>) => {
		console.log("Updated data:", data)
		$mutate(data, {
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
	<Field label="Roles" value={formatRoles(user?.roles ?? [])} />

	<Field label="Contraseña" value="********" />
</div>
