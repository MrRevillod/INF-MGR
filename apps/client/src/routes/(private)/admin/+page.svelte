<script lang="ts">
	import { goto } from "$app/navigation"
	import PageTitle from "$lib/shared/components/ui/PageTitle.svelte"
	import CourseCard from "$lib/courses/components/CourseCard.svelte"
	import { getAllCoursesQuery } from "$lib/courses/queries"

	const coursesQuery = getAllCoursesQuery()
	const { data: coursesRes, isLoading } = $derived(coursesQuery)

	// Estado para el filtro de año
	let selectedYear = $state<number | null>(null)

	// Obtener años únicos de los cursos
	const availableYears = $derived(
		coursesRes?.data
			? Array.from(new Set(coursesRes.data.map(course => course.year))).sort(
					(a, b) => b - a
				)
			: []
	)

	// Agrupar cursos por año
	const coursesByYear = $derived(() => {
		if (!coursesRes?.data) return {}

		const grouped: Record<number, typeof coursesRes.data> = {}

		coursesRes.data.forEach(course => {
			if (!grouped[course.year]) {
				grouped[course.year] = []
			}
			grouped[course.year].push(course)
		})

		return grouped
	})

	// Filtrar por año seleccionado
	const filteredCourses = $derived(() => {
		const grouped = coursesByYear()
		if (selectedYear === null) return grouped

		return { [selectedYear]: grouped[selectedYear] || [] }
	})

	// Ordenar cursos por año (más reciente primero)
	const sortedCoursesByYear = $derived(() => {
		const courses = filteredCourses()
		return Object.entries(courses).sort(([yearA], [yearB]) => {
			return Number(yearB) - Number(yearA)
		})
	})

	function handleCourseClick(courseId: string) {
		goto(`/admin/courses/${courseId}`)
	}

	function handleViewAllCourses() {
		goto("/admin/courses")
	}
</script>

<section class="space-y-6">
	<div class="flex items-center justify-between">
		<PageTitle
			title="Panel de Administración"
			description="Gestiona los cursos y estudiantes del sistema"
		/>
		<button
			onclick={handleViewAllCourses}
			class="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
		>
			Ver todos los cursos
		</button>
	</div>

	<!-- Filtro por año -->
	<div class="flex gap-2">
		<button
			onclick={() => (selectedYear = null)}
			class="rounded-md px-4 py-2 text-sm font-medium transition-colors {selectedYear ===
			null
				? 'bg-blue-600 text-white'
				: 'border border-gray-300 bg-white text-gray-700 hover:bg-gray-50'}"
		>
			Todos
		</button>
		{#each availableYears as year}
			<button
				onclick={() => (selectedYear = year)}
				class="rounded-md px-4 py-2 text-sm font-medium transition-colors {selectedYear ===
				year
					? 'bg-blue-600 text-white'
					: 'border border-gray-300 bg-white text-gray-700 hover:bg-gray-50'}"
			>
				{year}
			</button>
		{/each}
	</div>

	<!-- Cursos agrupados por año -->
	{#if isLoading}
		<div class="flex items-center justify-center py-12">
			<div
				class="h-8 w-8 animate-spin rounded-full border-4 border-gray-200 border-t-blue-600"
			></div>
			<p class="ml-3 text-sm text-gray-500">Cargando cursos...</p>
		</div>
	{:else if !coursesRes?.data || coursesRes.data.length === 0}
		<div class="py-12 text-center text-gray-500">
			<svg
				class="mx-auto mb-3 h-12 w-12 text-gray-400"
				fill="none"
				viewBox="0 0 24 24"
				stroke="currentColor"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
				/>
			</svg>
			<p class="font-medium">No hay cursos registrados</p>
		</div>
	{:else}
		{#each sortedCoursesByYear() as [year, courses]}
			<div class="space-y-4">
				<h2 class="text-2xl font-bold text-gray-900">Año {year}</h2>
				<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
					{#each courses as course}
						<CourseCard {course} onclick={() => handleCourseClick(course.id)} />
					{/each}
				</div>
			</div>
		{/each}
	{/if}
</section>
