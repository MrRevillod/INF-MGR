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
	practiceStatus: v.optional(
		v.union([v.literal("pending"), v.literal("approved"), v.literal("declined")])
	),
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

// Schema para crear una práctica
export const CreatePracticeSchema = v.object({
	enterpriseName: v.pipe(
		v.string(),
		v.minLength(1, "El nombre de la empresa es requerido"),
		v.maxLength(200, "El nombre no puede exceder 200 caracteres")
	),
	description: v.pipe(
		v.string(),
		v.minLength(1, "La descripción es requerida"),
		v.maxLength(1000, "La descripción no puede exceder 1000 caracteres")
	),
	location: v.pipe(
		v.string(),
		v.minLength(1, "La ubicación es requerida"),
		v.maxLength(200, "La ubicación no puede exceder 200 caracteres")
	),
	supervisorName: v.pipe(
		v.string(),
		v.minLength(1, "El nombre del supervisor es requerido"),
		v.maxLength(100, "El nombre no puede exceder 100 caracteres")
	),
	supervisorEmail: v.pipe(
		v.string(),
		v.minLength(1, "El email del supervisor es requerido"),
		v.email("El email debe ser válido")
	),
	supervisorPhone: v.pipe(
		v.string(),
		v.minLength(1, "El teléfono del supervisor es requerido"),
		v.regex(/^\+?[\d\s-()]+$/, "Formato de teléfono inválido")
	),
	startDate: v.pipe(v.string(), v.minLength(1, "La fecha de inicio es requerida")),
	endDate: v.pipe(v.string(), v.minLength(1, "La fecha de fin es requerida")),
})

export type CreatePractice = v.InferInput<typeof CreatePracticeSchema>
