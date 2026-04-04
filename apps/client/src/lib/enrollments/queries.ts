import type { ApiResponse } from "$api/utils"
import type { Enrollment, Course } from "./schemas"

import { protectedApi } from "$api/client"
import { createQuery } from "@tanstack/svelte-query"
import { TryFn } from "$api/utils"

export const getStudentEnrollmentsQuery = (studentId: string) => {
	return createQuery<ApiResponse<Enrollment[]>, ApiResponse>(() => ({
		queryKey: ["enrollments", "student", studentId],
		queryFn: () =>
			TryFn(() => protectedApi.get<Enrollment[]>(`/courses/student/${studentId}`)),
		enabled: !!studentId,
	}))
}

export const getCourseEnrollmentsQuery = (courseId: string) => {
	return createQuery<ApiResponse<Enrollment[]>, ApiResponse>(() => ({
		queryKey: ["enrollments", "course", courseId],
		queryFn: () =>
			TryFn(() => protectedApi.get<Enrollment[]>(`/courses/${courseId}/students`)),
		enabled: !!courseId,
	}))
}

export const getStudentCoursesQuery = (studentId: string) => {
	return createQuery<ApiResponse<Course[]>, ApiResponse>(() => ({
		queryKey: ["courses", "student", studentId],
		queryFn: () =>
			TryFn(() => protectedApi.get<Course[]>(`/students/${studentId}/courses`)),
		enabled: !!studentId,
	}))
}

export const getEnrollmentsByCourseQuery = (courseId: string) => {
	return createQuery<ApiResponse<Enrollment[]>, ApiResponse>(() => ({
		queryKey: ["enrollments", "course", courseId],
		queryFn: () =>
			TryFn(() => protectedApi.get<Enrollment[]>(`/courses/${courseId}/students`)),
		enabled: !!courseId,
	}))
}
