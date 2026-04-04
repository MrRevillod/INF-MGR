<script lang="ts">
	import type { PageProps } from "./$types"

	import { goto } from "$app/navigation"
	import PageTitle from "$lib/shared/components/ui/PageTitle.svelte"
	import UpdateUserForm from "$lib/users/components/UpdateUserForm.svelte"
	import Button from "$lib/shared/components/ui/Button.svelte"
	import { spanishRoles } from "$users/utils"
	import { getStudentEnrollmentsQuery } from "$lib/enrollments/queries"
	import { getTeacherCoursesQuery } from "$lib/courses/queries"

	const { data: page }: PageProps = $props()

	// Obtener cursos según el rol del usuario
	const enrollmentsQuery =
		page.user?.role === "student" ? getStudentEnrollmentsQuery(page.user.id) : null

	const teacherCoursesQuery =
		page.user?.role === "teacher" ? getTeacherCoursesQuery(page.user.id) : null

	const enrollmentsData = $derived(
		enrollmentsQuery ? enrollmentsQuery : { data: null, isLoading: false }
	)

	const teacherCoursesData = $derived(
		teacherCoursesQuery ? teacherCoursesQuery : { data: null, isLoading: false }
	)

	function handleUpdateSuccess() {
		goto("/secretary/users")
	}

	function handleBack() {
		goto("/secretary/users")
	}

	function handleCourseClick(courseId: string) {
		goto(`/secretary/courses/${courseId}`)
	}

	function calculateWeightedAverage(
		scores: Array<{ evaluationId: string; score: number }> | undefined,
		evaluations: Array<{ id: string; weight: number }> | undefined
	): number | null {
		if (!scores || scores.length === 0 || !evaluations || evaluations.length === 0) {
			return null
		}

		let weightedSum = 0
		let totalWeight = 0
		let hasScores = false

		for (const evaluation of evaluations) {
			const score = scores.find(s => s.evaluationId === evaluation.id)
			if (score && score.score > 0) {
				// Multiplicar la nota por el peso de la evaluación
				weightedSum += score.score * evaluation.weight
				totalWeight += evaluation.weight
				hasScores = true
			}
		}

		if (!hasScores) {
			return null
		}

		if (totalWeight === 0) {
			return null
		}

		return weightedSum / totalWeight
	}
</script>

<section class="space-y-6">
	<div class="flex items-center justify-between">
		<PageTitle
			title={`Perfil de ${page.user?.name ?? "Cargando..."}`}
			description={`Rol: ${spanishRoles[page.user?.role ?? "student"]}`}
		/>

		<Button onclick={handleBack} variant="secondary" text="← Volver a usuarios" />
	</div>

	<section class="flex w-full flex-row items-start justify-between gap-8">
		<!-- Columna izquierda: Formulario de actualización -->
		<section class="flex w-1/2 flex-col gap-4">
			<div>
				<h2 class="text-lg font-semibold text-gray-900">Editar Usuario</h2>
				<p class="text-sm text-gray-500">Actualiza la información del usuario</p>
			</div>

			<div class="rounded-lg border border-gray-200 bg-white p-6 shadow-sm">
				<UpdateUserForm user={page.user} onSuccess={handleUpdateSuccess} />
			</div>

			<!-- Secretary NO puede eliminar usuarios -->
			<div class="rounded-lg border border-yellow-200 bg-yellow-50 p-4">
				<p class="text-sm text-yellow-800">
					<strong>Nota:</strong> Solo los administradores pueden eliminar usuarios.
				</p>
			</div>
		</section>

		<!-- Columna derecha: Cursos -->
		<section class="flex w-1/2 flex-col gap-4">
			<div>
				<h2 class="text-lg font-semibold text-gray-900">
					{#if page.user?.role === "student"}
						Cursos Inscritos
					{:else if page.user?.role === "teacher"}
						Cursos que Imparte
					{:else}
						Información Adicional
					{/if}
				</h2>
				<p class="text-sm text-gray-500">
					{#if page.user?.role === "student"}
						Listado de cursos del estudiante
					{:else if page.user?.role === "teacher"}
						Cursos a cargo del profesor
					{:else}
						Este usuario no tiene cursos asignados
					{/if}
				</p>
			</div>

			<div class="rounded-lg border border-gray-200 bg-white shadow-sm">
				{#if enrollmentsData.isLoading || teacherCoursesData.isLoading}
					<div class="p-6 text-center text-gray-500">Cargando...</div>
				{:else if page.user?.role === "student" && enrollmentsData.data?.data}
					{#if enrollmentsData.data.data.length === 0}
						<div class="p-6 text-center text-gray-500">No hay cursos inscritos</div>
					{:else}
						<div class="divide-y divide-gray-200">
							{#each enrollmentsData.data.data as enrollment}
								<button
									onclick={() => handleCourseClick(enrollment.course.id)}
									class="w-full p-4 text-left transition-colors hover:bg-gray-50"
								>
									<div class="flex items-start justify-between">
										<div class="flex-1">
											<h3 class="font-medium text-gray-900">
												{enrollment.course.name}
											</h3>
											<p class="text-sm text-gray-500">
												Código: {enrollment.course.code}
											</p>
											<p class="text-sm text-gray-500">
												Profesor: {enrollment.course.teacher?.name ?? "N/A"}
											</p>
										</div>
										<div class="text-right">
											{#if enrollment.studentScores && enrollment.studentScores.length > 0}
												{@const avg = calculateWeightedAverage(
													enrollment.studentScores,
													enrollment.course.evaluations
												)}
												{#if avg !== null}
													<div
														class="text-2xl font-bold {avg >= 4.0
															? 'text-green-600'
															: 'text-red-600'}"
													>
														{avg.toFixed(1)}
													</div>
													<div class="text-xs text-gray-500">Promedio</div>
												{:else}
													<div class="text-sm text-gray-400">Sin notas</div>
												{/if}
											{:else}
												<div class="text-sm text-gray-400">Sin notas</div>
											{/if}
										</div>
									</div>
								</button>
							{/each}
						</div>
					{/if}
				{:else if page.user?.role === "teacher" && teacherCoursesData.data?.data}
					{#if teacherCoursesData.data.data.length === 0}
						<div class="p-6 text-center text-gray-500">
							No tiene cursos asignados
						</div>
					{:else}
						<div class="divide-y divide-gray-200">
							{#each teacherCoursesData.data.data as course}
								<button
									onclick={() => handleCourseClick(course.id)}
									class="w-full p-4 text-left transition-colors hover:bg-gray-50"
								>
									<div>
										<h3 class="font-medium text-gray-900">{course.name}</h3>
										<p class="text-sm text-gray-500">
											Código: {course.code} | Año: {course.year}
										</p>
									</div>
								</button>
							{/each}
						</div>
					{/if}
				{:else}
					<div class="p-6 text-center text-gray-500">
						Este usuario no tiene cursos asignados
					</div>
				{/if}
			</div>
		</section>
	</section>
</section>
