import type { User } from "$lib/schemas/user"
import type { TableColumn } from "$components/Table/types"

import { PencilIcon, XMarkIcon } from "@fvilers/heroicons-svelte/20/solid"

export const tableColumns: TableColumn<User>[] = [
	{ key: "rut", label: "RUT" },
	{ key: "name", label: "Nombre" },
	{ key: "email", label: "Correo electrónico" },
	{
		key: "roles",
		label: "Roles",
		formatter: value => {
			const spanishRoles = {
				student: "Estudiante",
				teacher: "Profesor (a)",
				administrator: "Administrador (a)",
				secretary: "Secretario (a)",
				coordinator: "Coordinador (a)",
			}

			if (!value.roles || value.roles.length === 0) {
				return "Sin roles"
			}

			return value.roles.map(role => spanishRoles[role] ?? role).join(", ")
		},
	},
]

export const actions = [
	{
		label: "Editar",
		icon: PencilIcon,
		func: (user: User) => {},
	},
	{
		label: "Eliminar",
		icon: XMarkIcon,
		func: (user: User) => {},
	},
]
