import type { PageLoad } from "./$types"

export const ssr = false

export const load: PageLoad = async ({ params }) => {
	return {
		enrollmentId: params.enrollmentId,
		practiceId: params.practiceId,
	}
}
