<script lang="ts">
	import type { PageData } from "./$types"
	import { getTeacherCoursesQuery } from "$lib/courses/queries"
	import CourseCard from "$lib/courses/components/CourseCard.svelte"

	let { data }: { data: PageData } = $props()

	const coursesQuery = getTeacherCoursesQuery(data.user!.id)
	const courses = $derived(coursesQuery.data?.data ?? [])
</script>

<div class="container mx-auto px-4 py-8">
	<div class="mb-8">
		<h1 class="text-3xl font-bold text-gray-900">
			Bienvenido, {data.user!.name}
		</h1>
		<p class="mt-2 text-gray-600">Panel de profesor</p>
	</div>

	<div class="mb-8">
		<div class="flex items-center justify-between">
			<h2 class="text-2xl font-semibold text-gray-900">Tus Cursos</h2>
			<a
				href="/teacher/courses"
				class="text-blue-600 hover:text-blue-800 hover:underline"
			>
				Ver todos →
			</a>
		</div>
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
			{#each courses.slice(0, 6) as course (course.id)}
				<CourseCard
					{course}
					onclick={() => (window.location.href = `/teacher/courses/${course.id}`)}
				/>
			{/each}
		</div>
		{#if courses.length > 6}
			<div class="mt-6 text-center">
				<a
					href="/teacher/courses"
					class="inline-block rounded-lg bg-blue-600 px-6 py-3 text-white hover:bg-blue-700"
				>
					Ver todos los cursos ({courses.length})
				</a>
			</div>
		{/if}
	{/if}
</div>
