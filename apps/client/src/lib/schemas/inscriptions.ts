import * as v from "valibot"

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
