<script lang="ts">
	import type { Course } from "$lib/courses/schemas"
	import { getCourseEnrollmentsQuery } from "$lib/enrollments/queries"

	interface Props {
		course: Course
		onclick: () => void
	}

	let { course, onclick }: Props = $props()

	// Query reactiva para obtener los enrollments del curso
	const enrollmentsQuery = $derived(getCourseEnrollmentsQuery(course.id))
	const { data: enrollmentsRes } = $derived(enrollmentsQuery)

	// Conteo de estudiantes
	const enrollmentCount = $derived(enrollmentsRes?.data?.length ?? 0)
</script>

<button
	{onclick}
	class="group relative overflow-hidden rounded-lg border border-gray-200 bg-white p-6 shadow-sm transition-all hover:border-blue-300 hover:shadow-md"
>
	<!-- Badge de estado -->
	<div class="absolute right-4 top-4">
		<span
			class="inline-flex rounded-full px-2 py-1 text-xs font-semibold {course.courseStatus ===
			'active'
				? 'bg-green-100 text-green-800'
				: 'bg-gray-100 text-gray-800'}"
		>
			{course.courseStatus === "active" ? "Activo" : "Completado"}
		</span>
	</div>

	<!-- Contenido de la tarjeta -->
	<div class="space-y-3">
		<div>
			<p class="text-sm font-medium text-blue-600">{course.code}</p>
			<h3 class="mt-1 text-lg font-semibold text-gray-900 group-hover:text-blue-600">
				{course.name}
			</h3>
		</div>

		<div class="space-y-2 text-sm text-gray-600">
			<div class="flex items-center">
				<svg
					class="mr-2 h-4 w-4"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
					/>
				</svg>
				<span>{course.teacher?.name ?? "Sin profesor"}</span>
			</div>

			<div class="flex items-center font-semibold text-gray-900">
				<svg
					class="mr-2 h-4 w-4"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
					/>
				</svg>
				<span>{enrollmentCount} estudiante{enrollmentCount !== 1 ? "s" : ""}</span>
			</div>
		</div>
	</div>

	<!-- Indicador de hover -->
	<div
		class="absolute bottom-0 left-0 right-0 h-1 scale-x-0 transform bg-blue-600 transition-transform group-hover:scale-x-100"
	></div>
</button>
