import * as v from "valibot"
import type { Course } from "../courses/schemas"

export interface User {
	id: string
	rut: string
	name: string
	email: string
	roles: Role[]
	createdAt: string
	deletedAt: string | null
}

const RoleDto = v.union([
	v.literal("administrator"),
	v.literal("student"),
	v.literal("teacher"),
	v.literal("coordinator"),
	v.literal("secretary"),
])

export type Role = v.InferInput<typeof RoleDto>

export const CreateUserSchema = v.object({
	rut: v.string(),
	name: v.string(),
	email: v.string(),
	password: v.string(),
	confirmPassword: v.string(),
	roles: v.array(RoleDto),
})

export type CreateUserSchemaType = v.InferInput<typeof CreateUserSchema>

export const UpdateUserSchema = v.object({
	email: v.optional(v.string()),
	password: v.optional(v.string()),
	confirmPassword: v.optional(v.string()),
	roles: v.optional(v.array(RoleDto)),
})

export type UpdateUserSchemaType = v.InferInput<typeof UpdateUserSchema>
