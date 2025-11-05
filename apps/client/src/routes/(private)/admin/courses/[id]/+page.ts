import type { Course } from "$lib/courses/schemas"
import type { PageLoad } from "./$types"

export const load: PageLoad = async ({ params }) => {
	return {
		course: null as Course | null,
		courseId: params.id,
	}
}
