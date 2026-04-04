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
		goto(`/secretary/courses/${courseId}`)
	}

	function handleViewAllCourses() {
		goto("/secretary/courses")
	}
</script>

<section class="space-y-6">
	<div class="flex items-center justify-between">
		<PageTitle
			title="Panel de Secretaría"
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

	{#if isLoading}
		<div class="flex items-center justify-center py-12">
			<div class="text-gray-500">Cargando cursos...</div>
		</div>
	{:else if !coursesRes?.data || coursesRes.data.length === 0}
		<div
			class="flex flex-col items-center justify-center rounded-lg border-2 border-dashed border-gray-300 py-12"
		>
			<p class="text-gray-500">No hay cursos registrados</p>
		</div>
	{:else}
		<div class="space-y-8">
			{#each sortedCoursesByYear() as [year, courses]}
				<div>
					<h2 class="mb-4 text-xl font-semibold text-gray-900">
						Año {year}
					</h2>
					<div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
						{#each courses as course}
							<CourseCard {course} onclick={() => handleCourseClick(course.id)} />
						{/each}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</section>
