import type { User } from "$users/schemas"
import type { TableColumn } from "$lib/shared/components/Table.svelte"

export const spanishRoles = {
	student: "Estudiante",
	teacher: "Profesor (a)",
	administrator: "Administrador (a)",
	secretary: "Secretario (a)",
	coordinator: "Coordinador (a)",
}

export const tableColumns: TableColumn<User>[] = [
	{ key: "rut", label: "RUT" },
	{ key: "name", label: "Nombre" },
	{ key: "email", label: "Correo electrónico" },
	{ key: "role", label: "Rol" },
]

export const RutFormatter = (rut: string): string => {
	const match = rut.match(/^(\d{1,2})(\d{3})(\d{3})-(\w)$/)
	if (!match) return rut

	return `${match[1]}.${match[2]}.${match[3]}-${match[4].toUpperCase()}`
}
