import * as v from "valibot"

import type { User } from "../users/schemas"

export interface Course {
	id: string
	year: number
	name: string
	code: string
	evaluations: Evaluation[]
	status: CourseStatus

	teacherId: string
	coordinatorId: string

	teacher: User
	coordinator: User
}

export const EvaluationSchema = v.object({
	id: v.string(),
	name: v.string(),
	weight: v.number(),
})

export type Evaluation = v.InferInput<typeof EvaluationSchema>

export const CourseStatusSchema = v.union([
	v.literal("inprogress"),
	v.literal("ended"),
])

export type CourseStatus = v.InferInput<typeof CourseStatusSchema>

// -------------------------------------

export interface Inscription {
	id: string
	userId: string
	asignatureId: string
	practiceId: string | null
	studentScores: StudentScore[]
	course: Course
}

export const StudentScoreSchema = v.object({
	id: v.string(),
	score: v.number(),
})

export type StudentScore = v.InferInput<typeof StudentScoreSchema>

// -------------------------------------------------

export interface Report {
	id: string
	inscriptionId: string
	userId: string
	title: string
	content: string
	createdAt: string
	updatedAt: string
}
