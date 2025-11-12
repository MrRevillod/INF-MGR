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

// Schema para crear evaluaciones (sin ID)
export const CreateEvaluationSchema = v.object({
	name: v.pipe(
		v.string(),
		v.minLength(1, "El nombre de la evaluación es requerido"),
		v.maxLength(100, "El nombre no puede exceder 100 caracteres")
	),
	weight: v.pipe(
		v.number(),
		v.minValue(1, "El porcentaje debe ser mayor a 0"),
		v.maxValue(100, "El porcentaje no puede exceder 100")
	),
})

export type CreateEvaluation = v.InferInput<typeof CreateEvaluationSchema>

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
		v.array(CreateEvaluationSchema),
		v.minLength(1, "Debe haber al menos una evaluación"),
		v.check(evaluations => {
			const totalWeight = evaluations.reduce((sum, ev) => sum + ev.weight, 0)
			return totalWeight === 100
		}, "La suma de los porcentajes debe ser exactamente 100%")
	),
})

// Schema parcial para el formulario (sin evaluations, que se manejan localmente)
export const CreateCourseFormSchema = v.object({
	code: v.pipe(
		v.string(),
		v.minLength(1, "El código es requerido"),
		v.regex(
			/^INFO\d{4}$/,
			"El código debe tener el formato INFO seguido de 4 dígitos (ej: INFO1164)"
		)
	),
	name: v.pipe(
		v.string(),
		v.minLength(1, "El nombre es requerido"),
		v.maxLength(100, "El nombre no puede exceder 100 caracteres")
	),
	teacherId: v.pipe(v.string(), v.minLength(1, "Debe seleccionar un profesor")),
	year: v.pipe(v.number(), v.integer(), v.minValue(2000, "Año inválido")),
})

// Schema para actualizar curso
export const UpdateCourseSchema = CreateCourseSchema

// Schema parcial para el formulario de actualización (sin evaluations, que se manejan localmente)
export const UpdateCourseFormSchema = CreateCourseFormSchema

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
