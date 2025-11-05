import type { Course } from "./schemas"
import type { Enrollment } from "$lib/enrollments/schemas"
import type { ApiResponse } from "$api/utils"

import { protectedApi } from "$api/client"
import { TryFn } from "$api/utils"
import { createMutation, useQueryClient } from "@tanstack/svelte-query"

export const createCourseMutation = () => {
	const queryClient = useQueryClient()

	return createMutation<ApiResponse<Course>, ApiResponse, Record<string, unknown>>(
		() => ({
			mutationKey: ["create-course"],
			mutationFn: data =>
				TryFn<Course>(() => protectedApi.post<Course>("/courses", data)),
			onSuccess: () => {
				queryClient.invalidateQueries({
					queryKey: ["courses"],
				})
			},
		})
	)
}

export const updateCourseMutation = (id: string) => {
	const queryClient = useQueryClient()

	return createMutation<ApiResponse<Course>, ApiResponse, Record<string, unknown>>(
		() => ({
			mutationKey: ["update-course", id],
			mutationFn: data =>
				TryFn<Course>(() => protectedApi.patch<Course>(`/courses/${id}`, data)),
			onSuccess: () => {
				queryClient.invalidateQueries({
					queryKey: ["course", id],
				})
				queryClient.invalidateQueries({
					queryKey: ["courses"],
				})
			},
		})
	)
}

export const deleteCourseMutation = (id: string) => {
	const queryClient = useQueryClient()

	return createMutation<ApiResponse<void>, ApiResponse, void>(() => ({
		mutationKey: ["delete-course", id],
		mutationFn: () => TryFn<void>(() => protectedApi.delete(`/courses/${id}`)),
		onSuccess: () => {
			queryClient.invalidateQueries({
				queryKey: ["courses"],
			})
		},
	}))
}

export const enrollStudentMutation = () => {
	const queryClient = useQueryClient()

	return createMutation<
		ApiResponse<Enrollment>,
		ApiResponse,
		{ studentId: string; courseId: string }
	>(() => ({
		mutationKey: ["enroll-student"],
		mutationFn: data => {
			// Asegurar que los datos se envíen en el formato correcto (camelCase)
			const payload = {
				studentId: data.studentId,
				courseId: data.courseId,
			}
			return TryFn<Enrollment>(() =>
				protectedApi.post<Enrollment>("/courses/enroll", payload)
			)
		},
		onSuccess: (_, variables) => {
			queryClient.invalidateQueries({
				queryKey: ["enrollments"],
			})
			queryClient.invalidateQueries({
				queryKey: ["courses"],
			})
			queryClient.invalidateQueries({
				queryKey: ["course", variables.courseId],
			})
			queryClient.invalidateQueries({
				queryKey: ["enrollments", "course", variables.courseId],
			})
		},
	}))
}

export const deleteEnrollmentMutation = (enrollmentId: string) => {
	const queryClient = useQueryClient()

	return createMutation<ApiResponse<void>, ApiResponse, void>(() => ({
		mutationKey: ["delete-enrollment", enrollmentId],
		mutationFn: () =>
			TryFn<void>(() => protectedApi.delete(`/courses/enrollments/${enrollmentId}`)),
		onSuccess: () => {
			queryClient.invalidateQueries({
				queryKey: ["enrollments"],
			})
			queryClient.invalidateQueries({
				queryKey: ["courses"],
			})
		},
	}))
}
