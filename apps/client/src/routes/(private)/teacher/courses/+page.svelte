<script lang="ts">
	import type { PageData } from "./$types"
	import { getTeacherCoursesQuery } from "$lib/courses/queries"
	import CourseCard from "$lib/courses/components/CourseCard.svelte"

	let { data }: { data: PageData } = $props()

	const coursesQuery = getTeacherCoursesQuery(data.user!.id)

	const courses = $derived(coursesQuery.data?.data || [])
</script>

<div class="container mx-auto px-4 py-8">
	<div class="mb-8">
		<h1 class="text-3xl font-bold text-gray-900">Mis Cursos</h1>
		<p class="mt-2 text-gray-600">
			Cursos que enseñas y gestión de prácticas de estudiantes
		</p>
	</div>

	{#if coursesQuery.isLoading}
		<div class="flex items-center justify-center py-12">
			<div class="text-gray-500">Cargando cursos...</div>
		</div>
	{:else if coursesQuery.isError}
		<div class="rounded-lg border border-red-200 bg-red-50 p-4">
			<p class="text-sm text-red-800">Error al cargar los cursos</p>
		</div>
	{:else if courses.length === 0}
		<div class="rounded-lg border border-gray-200 bg-gray-50 p-8 text-center">
			<p class="text-gray-600">No tienes cursos asignados</p>
		</div>
	{:else}
		<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
			{#each courses as course (course.id)}
				<CourseCard
					{course}
					onclick={() => (window.location.href = `/teacher/courses/${course.id}`)}
				/>
			{/each}
		</div>
	{/if}
</div>
