import { type } from "arktype"

export interface Asignature {
	id: string
	year: number
	name: string
	code: string
	evaluations: Evaluation[]
	status: AsignatureStatus
	teacherId: string
}

const Evaluation = type({
	id: "string",
	name: "string",
	weight: "number"
})

export type Evaluation = typeof Evaluation.infer

const AsignatureStatus = type("('inprogress' | 'ended')")
export type AsignatureStatus = typeof AsignatureStatus.infer

export const UpdateAsignatureSchema = type({
	teacherId: "string?",
	evaluations: Evaluation.array().optional(),
	status: AsignatureStatus.optional()
})

export type UpdateAsignatureSchemaType = typeof UpdateAsignatureSchema.infer
