import { type } from "arktype"

export interface Inscription {
	id: string
	userId: string
	asignatureId: string
	practiceId: string
	evaluationScores: StudentEvaluation[]
	status: StudentStatus
}

const StudentStatus = type("('active' | 'inactive' | 'completed' | 'evaluating')")
export type StudentStatus = typeof StudentStatus.infer

const StudentEvaluation = type({
	id: "string",
	score: "number"
})

export type StudentEvaluation = typeof StudentEvaluation.infer

export const CreateInscriptionSchema = type({
	userId: "string",
	asignatureId: "string"
})

export type CreateInscriptionSchemaType = typeof CreateInscriptionSchema.infer

export const UpdateInscriptionSchema = type({
	practiceId: "string?",
	evaluationScores: StudentEvaluation.array().optional(),
	status: StudentStatus.optional()
})
