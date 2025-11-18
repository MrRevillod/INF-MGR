import { api } from "$lib/shared/api/client"
import { TryFn } from "$lib/shared/api/utils"
import type { PageLoad } from "./$types"

export const ssr = false

export const load: PageLoad = async ({ params }) => {
	const { enrollmentId, practiceId } = params

	const response = await TryFn(() =>
		api.post<null>(`/enrollments/${enrollmentId}/practice/${practiceId}/approve`)
	)

	return {
		success: response.success,
	}
}
