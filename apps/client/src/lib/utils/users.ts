import type { User } from "$lib/schemas/user"
import type { TableColumn } from "$components/Table/types"

export const spanishRoles = {
	student: "Estudiante",
	teacher: "Profesor (a)",
	administrator: "Administrador (a)",
	secretary: "Secretario (a)",
	coordinator: "Coordinador (a)",
}

export const formatRoles = (roles: string[]): string => {
	if (!roles || roles.length === 0) {
		return "Sin roles"
	}

	return roles
		.map(role => spanishRoles[role as keyof typeof spanishRoles] ?? role)
		.join(", ")
}

export const tableColumns: TableColumn<User>[] = [
	{ key: "rut", label: "RUT" },
	{ key: "name", label: "Nombre" },
	{ key: "email", label: "Correo electrónico" },
	{
		key: "roles",
		label: "Roles",
		formatter: value => formatRoles(value.roles as string[]),
	},
]

export const RutFormatter = (rut: string): string => {
	const match = rut.match(/^(\d{1,2})(\d{3})(\d{3})-(\w)$/)
	if (!match) return rut

	return `${match[1]}.${match[2]}.${match[3]}-${match[4].toUpperCase()}`
}
