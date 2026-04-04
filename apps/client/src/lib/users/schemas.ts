import * as v from "valibot"

export interface User {
	id: string
	rut: string
	name: string
	email: string
	role: Role
	createdAt: string
	deletedAt: string | null
}

const RoleDto = v.union([
	v.literal("student"),
	v.literal("teacher"),
	v.literal("administrator"),
	v.literal("secretary"),
])

export type Role = v.InferInput<typeof RoleDto>

export const CreateUserSchema = v.object({
	rut: v.pipe(
		v.string("El RUT es requerido"),
		v.minLength(1, "El RUT es requerido"),
		v.regex(/^\d{7,8}-[\dkK]$/, "Formato de RUT inválido (ej: 12345678-5)")
	),
	name: v.pipe(
		v.string("El nombre es requerido"),
		v.minLength(5, "El nombre debe tener al menos 5 caracteres"),
		v.maxLength(100, "El nombre no puede exceder 100 caracteres")
	),
	email: v.pipe(
		v.string("El email es requerido"),
		v.email("El email debe ser válido")
	),
	role: RoleDto,
})

export type CreateUserSchemaType = v.InferInput<typeof CreateUserSchema>

export const UpdateUserSchema = v.object({
	email: v.optional(v.string()),
	password: v.optional(v.string()),
	confirmPassword: v.optional(v.string()),
	role: v.optional(RoleDto),
})

export type UpdateUserSchemaType = v.InferInput<typeof UpdateUserSchema>
