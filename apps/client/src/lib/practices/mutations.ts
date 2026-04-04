import type { Practice } from "./schemas"
import type { ApiResponse } from "$api/utils"

import { protectedApi } from "$api/client"
import { TryFn } from "$api/utils"
import { createMutation, useQueryClient } from "@tanstack/svelte-query"

export const createPracticeMutation = (enrollmentId: string) => {
	const queryClient = useQueryClient()

	return createMutation<ApiResponse<Practice>, ApiResponse, Record<string, unknown>>(
		() => ({
			mutationKey: ["create-practice", enrollmentId],
			mutationFn: data =>
				TryFn<Practice>(() =>
					protectedApi.post(`/enrollments/${enrollmentId}/practice`, data)
				),
			onSuccess: () => {
				queryClient.invalidateQueries({
					queryKey: ["enrollments"],
				})
				queryClient.invalidateQueries({
					queryKey: ["enrollment", enrollmentId],
				})
			},
		})
	)
}

export const updatePracticeMutation = (practiceId: string) => {
	const queryClient = useQueryClient()

	return createMutation<ApiResponse<Practice>, ApiResponse, Record<string, unknown>>(
		() => ({
			mutationKey: ["update-practice", practiceId],
			mutationFn: data =>
				TryFn<Practice>(() => protectedApi.patch(`/practices/${practiceId}`, data)),
			onSuccess: () => {
				queryClient.invalidateQueries({
					queryKey: ["practice", practiceId],
				})
				queryClient.invalidateQueries({
					queryKey: ["enrollments"],
				})
			},
		})
	)
}

export const evaluatePracticeMutation = (
	enrollmentId: string,
	practiceId: string,
	evaluationId: string
) => {
	const queryClient = useQueryClient()

	return createMutation<ApiResponse<void>, ApiResponse, Record<string, unknown>>(
		() => ({
			mutationKey: ["evaluate-practice", practiceId, evaluationId],
			mutationFn: data =>
				TryFn<void>(() =>
					protectedApi.post(
						`/enrollments/${enrollmentId}/practice/${practiceId}/evaluate/${evaluationId}`,
						data
					)
				),
			onSuccess: () => {
				queryClient.invalidateQueries({
					queryKey: ["practice", practiceId],
				})
				queryClient.invalidateQueries({
					queryKey: ["enrollment", enrollmentId],
				})
				queryClient.invalidateQueries({
					queryKey: ["enrollments"],
				})
			},
		})
	)
}
