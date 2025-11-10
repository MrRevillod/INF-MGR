import type { ApiResponse } from "$api/utils"
import type { Course } from "./schemas"

import { protectedApi } from "$api/client"
import { createQuery } from "@tanstack/svelte-query"
import { TryFn } from "$api/utils"

export const getTeacherCoursesQuery = (teacherId: string) => {
	return createQuery<ApiResponse<Course[]>, ApiResponse>(() => ({
		queryKey: ["courses", "teacher", teacherId],
		queryFn: () =>
			TryFn(() => protectedApi.get<Course[]>(`/courses/teacher/${teacherId}`)),
		enabled: !!teacherId,
	}))
}

export const getAllCoursesQuery = () => {
	return createQuery<ApiResponse<Course[]>, ApiResponse>(() => ({
		queryKey: ["courses", "all"],
		queryFn: () => TryFn(() => protectedApi.get<Course[]>(`/courses`)),
	}))
}

// NUEVO: Query para obtener un curso por ID
export const getCourseQuery = (courseId: string) => {
	return createQuery<ApiResponse<Course>, ApiResponse>(() => ({
		queryKey: ["course", courseId],
		queryFn: () => TryFn(() => protectedApi.get<Course>(`/courses/${courseId}`)),
		enabled: !!courseId,
	}))
}
