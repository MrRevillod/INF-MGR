import * as v from "valibot"

export const CourseEvaluationSchema = v.object({
	id: v.string(),
	name: v.string(),
	weight: v.number(),
})

export const CourseSchema = v.object({
	id: v.string(),
	year: v.number(),
	code: v.string(),
	name: v.string(),
	courseStatus: v.union([v.literal("active"), v.literal("completed")]),
	evaluations: v.array(CourseEvaluationSchema),
	teacherId: v.string(),
	teacher: v.optional(
		v.object({
			id: v.string(),
			rut: v.string(),
			name: v.string(),
			email: v.string(),
			role: v.string(),
		})
	),
})

export const StudentSchema = v.object({
	id: v.string(),
	rut: v.string(),
	name: v.string(),
	email: v.string(),
	role: v.string(),
})

export const PracticeSchema = v.object({
	id: v.string(),
})

export const EnrollmentSchema = v.object({
	id: v.string(),
	studentId: v.string(),
	courseId: v.string(),
	practiceId: v.optional(v.string()),
	course: CourseSchema,
	student: StudentSchema,
	practice: v.optional(PracticeSchema),
	studentScores: v.optional(
		v.array(
			v.object({
				evaluationId: v.string(),
				score: v.number(),
			})
		)
	),
})

export const EnrollmentsResponseSchema = v.object({
	data: v.array(EnrollmentSchema),
})

export type Enrollment = v.InferOutput<typeof EnrollmentSchema>
export type Course = v.InferOutput<typeof CourseSchema>
export type Student = v.InferOutput<typeof StudentSchema>
export type EnrollmentsResponse = v.InferOutput<typeof EnrollmentsResponseSchema>
