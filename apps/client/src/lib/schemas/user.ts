import { type } from "arktype"
import type { Nullable } from "$lib/types"

export interface User {
	id: string
	rut: string
	name: string
	email: string
	roles: Role[]
	deletedAt: Nullable<string>
}

const RoleDto = type(
	"'administrator' | 'student' | 'teacher' | 'coordinator' | 'secretary'"
)

export const CreateUserSchema = type({
	rut: "string",
	name: "string",
	email: "string",
	password: "string",
	confirmPassword: "string",
	roles: RoleDto.array(),
})

export const UpdateUserSchema = type({
	email: "string?",
	password: "string?",
	confirmPassword: "string?",
	roles: RoleDto.array().optional(),
})

export type Role = typeof RoleDto.infer
export type CreateUserSchemaType = typeof CreateUserSchema.infer
export type UpdateUserSchemaType = typeof UpdateUserSchema.infer
