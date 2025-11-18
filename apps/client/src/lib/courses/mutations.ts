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

export const updateEnrollmentMutation = (enrollmentId: string) => {
	const queryClient = useQueryClient()

	return createMutation<
		ApiResponse<Enrollment>,
		ApiResponse,
		Record<string, unknown>
	>(() => ({
		mutationKey: ["update-enrollment", enrollmentId],
		mutationFn: data =>
			TryFn<Enrollment>(() =>
				protectedApi.patch<Enrollment>(`/courses/enrollments/${enrollmentId}`, data)
			),
		onSuccess: data => {
			queryClient.invalidateQueries({
				queryKey: ["enrollments"],
			})
			queryClient.invalidateQueries({
				queryKey: ["enrollment", enrollmentId],
			})
			queryClient.invalidateQueries({
				queryKey: ["courses"],
			})
			// Invalidar el curso específico si tenemos el courseId
			if (data.data?.courseId) {
				queryClient.invalidateQueries({
					queryKey: ["course", data.data.courseId],
				})
				queryClient.invalidateQueries({
					queryKey: ["enrollments", "course", data.data.courseId],
				})
			}
		},
	}))
}

export const importStudentsMutation = (courseId: string) => {
	const queryClient = useQueryClient()

	return createMutation<
		ApiResponse<void>,
		ApiResponse,
		{
			students: Array<{ rut: string; name: string; email: string; register: string }>
		}
	>(() => ({
		mutationKey: ["import-students", courseId],
		mutationFn: data =>
			TryFn<void>(() =>
				protectedApi.post(`/imports/course/${courseId}/students`, data)
			),
		onSuccess: () => {
			queryClient.invalidateQueries({
				queryKey: ["enrollments"],
			})
			queryClient.invalidateQueries({
				queryKey: ["courses"],
			})
			queryClient.invalidateQueries({
				queryKey: ["course", courseId],
			})
			queryClient.invalidateQueries({
				queryKey: ["enrollments", "course", courseId],
			})
			queryClient.invalidateQueries({
				queryKey: ["users"],
			})
		},
	}))
}
