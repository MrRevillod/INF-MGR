import * as v from "valibot"

export const CourseSchema = v.object({
	id: v.string(),
	code: v.string(),
	name: v.string(),
	description: v.optional(v.string()),
	teacherId: v.string(),
})

export const EnrollmentSchema = v.object({
	id: v.string(),
	studentId: v.string(),
	courseId: v.string(),
	enrolledAt: v.optional(v.string()),
	course: CourseSchema,
})

export const EnrollmentsResponseSchema = v.object({
	data: v.array(EnrollmentSchema),
})

export type Enrollment = v.InferOutput<typeof EnrollmentSchema>
export type Course = v.InferOutput<typeof CourseSchema>
export type EnrollmentsResponse = v.InferOutput<typeof EnrollmentsResponseSchema>
