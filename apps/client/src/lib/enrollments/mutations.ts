import { createMutation, useQueryClient } from "@tanstack/svelte-query"
import { protectedApi } from "$api/client"
import type { CreatePractice } from "./schemas"

export function createPracticeMutation(enrollmentId: string) {
	const queryClient = useQueryClient()

	return createMutation(() => ({
		mutationFn: async (data: CreatePractice) => {
			// Convertir las fechas a ISO string
			const payload = {
				...data,
				startDate: new Date(data.startDate).toISOString(),
				endDate: new Date(data.endDate).toISOString(),
			}

			const response = await protectedApi.post(
				`/enrollments/${enrollmentId}/practice`,
				payload
			)
			return response.data
		},
		onSuccess: () => {
			// Invalidar queries relacionadas
			queryClient.invalidateQueries({ queryKey: ["enrollments"] })
			queryClient.invalidateQueries({ queryKey: ["courses"] })
		},
	}))
}
