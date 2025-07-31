import * as v from "valibot"

export interface User {
	id: string
	rut: string
	name: string
	email: string
	roles: Role[]
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

export interface Inscription {
	id: string
	userId: string
	asignatureId: string
	practiceId: string
	evaluationScores: StudentEvaluation[]
	status: StudentStatus
}

const StudentStatus = v.union([
	v.literal("active"),
	v.literal("inactive"),
	v.literal("completed"),
	v.literal("evaluating"),
])

export type StudentStatus = v.InferInput<typeof StudentStatus>

const StudentEvaluation = v.object({
	id: v.string(),
	score: v.number(),
})

export type StudentEvaluation = v.InferInput<typeof StudentEvaluation>

export const CreateInscriptionSchema = v.object({
	userId: v.string(),
	asignatureId: v.string(),
})

export type CreateInscriptionSchemaType = v.InferInput<
	typeof CreateInscriptionSchema
>

export const UpdateInscriptionSchema = v.object({
	practiceId: v.optional(v.string()),
	evaluationScores: v.optional(v.array(StudentEvaluation)),
	status: v.optional(StudentStatus),
})
