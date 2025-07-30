import * as v from "valibot"

export interface Asignature {
	id: string
	year: number
	name: string
	code: string
	evaluations: Evaluation[]
	status: AsignatureStatus
	teacherId: string
}

const EvaluationSchema = v.object({
	id: v.string(),
	name: v.string(),
	weight: v.number(),
})

export type Evaluation = v.InferInput<typeof EvaluationSchema>

const AsignatureStatusSchema = v.union([v.literal("inprogress"), v.literal("ended")])
export type AsignatureStatus = v.InferInput<typeof AsignatureStatusSchema>

export const AsignatureSchema = v.object({
	id: v.string(),
	year: v.number(),
	name: v.string(),
	code: v.string(),
	evaluations: v.array(EvaluationSchema),
	status: AsignatureStatusSchema,
	teacherId: v.string(),
})

export type AsignatureType = v.InferInput<typeof AsignatureSchema>
