import * as v from "valibot"

import type { User } from "../users/schemas"

export interface Course {
	id: string
	year: number
	name: string
	code: string
	evaluations: Evaluation[]
	courseStatus: CourseStatus

	teacherId: string
	teacher: User
}

export const EvaluationSchema = v.object({
	id: v.string(),
	name: v.string(),
	weight: v.number(),
})

export type Evaluation = v.InferInput<typeof EvaluationSchema>

export const CourseStatusSchema = v.union([
	v.literal("active"),
	v.literal("completed"),
])

export type CourseStatus = v.InferInput<typeof CourseStatusSchema>

export const StudentScoreSchema = v.object({
	evaluationId: v.string(),
	score: v.number(),
})

export type StudentScore = v.InferInput<typeof StudentScoreSchema>

export const CreateCourseSchema = v.object({
	code: v.pipe(
		v.string(),
		v.minLength(1, "El código es requerido"),
		v.regex(
			/^INFO\d{4}$/,
			"El código debe tener el formato INFO seguido de 4 dígitos (ej: INFO1164)"
		)
	),
	name: v.pipe(v.string(), v.minLength(1, "El nombre es requerido")),
	teacherId: v.pipe(v.string(), v.minLength(1, "El profesor es requerido")),
	year: v.pipe(v.number(), v.minValue(2000, "Año inválido")),
	evaluations: v.pipe(
		v.array(EvaluationSchema),
		v.minLength(1, "Debe haber al menos una evaluación")
	),
})

// Schema para actualizar curso
export const UpdateCourseSchema = CreateCourseSchema

// Schema para inscribir estudiante
export const EnrollStudentSchema = v.object({
	studentId: v.pipe(v.string(), v.minLength(1, "Debe seleccionar un estudiante")),
	courseId: v.string(),
})

// Schema para actualizar inscripción (scores)
export const UpdateEnrollmentSchema = v.object({
	studentScores: v.optional(
		v.pipe(
			v.array(StudentScoreSchema),
			v.minLength(1, "Debe haber al menos un score")
		)
	),
	practiceId: v.optional(v.string()),
})
