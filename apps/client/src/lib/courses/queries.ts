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
