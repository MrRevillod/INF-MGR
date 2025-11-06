<script lang="ts">
	import type { PageProps } from "./$types"

	import { goto } from "$app/navigation"
	import PageTitle from "$lib/shared/components/ui/PageTitle.svelte"
	import UpdateUserForm from "$lib/users/components/UpdateUserForm.svelte"
	import DeleteUserButton from "$lib/users/components/DeleteUserButton.svelte"
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
		goto("/admin/users")
	}

	function handleBack() {
		goto("/admin/users")
	}

	function handleCourseClick(courseId: string) {
		goto(`/admin/courses/${courseId}`)
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
			{#if page.user?.role !== "administrator"}
				<DeleteUserButton user={page.user} />
			{/if}
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

			<div class="rounded-lg border border-gray-200 bg-white p-6 shadow-sm">
				<!-- ESTUDIANTE: Mostrar enrollments -->
				{#if page.user?.role === "student"}
					{#if enrollmentsData.isLoading}
						<div class="py-8 text-center">
							<div
								class="mx-auto h-8 w-8 animate-spin rounded-full border-4 border-gray-200 border-t-blue-600"
							></div>
							<p class="mt-3 text-sm text-gray-500">Cargando cursos...</p>
						</div>
					{:else if !enrollmentsData.data?.data || enrollmentsData.data.data.length === 0}
						<div class="py-8 text-center text-gray-500">
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
							<p class="font-medium">No hay cursos inscritos</p>
							<p class="mt-1 text-sm">
								Este estudiante aún no está inscrito en ningún curso
							</p>
						</div>
					{:else}
						<div class="space-y-4">
							{#each enrollmentsData.data.data as enrollment (enrollment.id)}
								{@const course = enrollment.course}
								{@const scores = enrollment.studentScores || []}
								{@const average = calculateWeightedAverage(
									scores,
									course?.evaluations
								)}

								<div
									class="rounded-lg border border-gray-200 bg-white p-4 transition-all hover:border-blue-300 hover:shadow-md"
								>
									<!-- Header del curso con promedio -->
									<div
										class="flex cursor-pointer items-start justify-between"
										onclick={() => handleCourseClick(course.id)}
										role="button"
										tabindex="0"
										onkeydown={e => {
											if (e.key === "Enter" || e.key === " ") {
												handleCourseClick(course.id)
											}
										}}
									>
										<div class="flex-1">
											<h3 class="font-semibold text-gray-900">{course.name}</h3>
											<div
												class="mt-1 flex items-center gap-3 text-xs text-gray-500"
											>
												<span>Código: {course.code}</span>
												<span>•</span>
												<span>Año: {course.year}</span>
												{#if course.teacher}
													<span>•</span>
													<span>Prof: {course.teacher.name}</span>
												{/if}
											</div>
										</div>
										<div class="ml-4">
											{#if average !== null}
												<span
													class="inline-flex items-center rounded-full px-3 py-1.5 text-sm font-bold {average >=
													4.0
														? 'bg-green-100 text-green-800'
														: 'bg-red-100 text-red-800'}"
												>
													{average.toFixed(1)}
												</span>
											{:else}
												<span
													class="inline-flex items-center rounded-full bg-gray-100 px-3 py-1.5 text-xs text-gray-600"
												>
													Sin calificar
												</span>
											{/if}
										</div>
									</div>

									<!-- Evaluaciones y calificaciones -->
									{#if course.evaluations && course.evaluations.length > 0}
										<div class="mt-4 space-y-2 border-t border-gray-100 pt-3">
											{#each course.evaluations as evaluation}
												{@const score = scores.find(
													s => s.evaluationId === evaluation.id
												)}
												<div
													class="flex items-center justify-between border-l-2 border-blue-200 py-1.5 pl-3"
												>
													<div class="flex items-center gap-2">
														<div class="h-1.5 w-1.5 rounded-full bg-blue-500"></div>
														<span class="text-sm text-gray-700">
															{evaluation.name}
														</span>
														<span class="text-xs text-gray-500">
															({evaluation.weight}%)
														</span>
													</div>
													<div>
														{#if score && score.score > 0}
															<span
																class="inline-flex rounded px-2 py-0.5 text-sm font-semibold {score.score >=
																4.0
																	? 'bg-green-50 text-green-700'
																	: 'bg-red-50 text-red-700'}"
															>
																{score.score.toFixed(1)}
															</span>
														{:else}
															<span class="text-xs text-gray-400">
																No calificado
															</span>
														{/if}
													</div>
												</div>
											{/each}
										</div>
									{/if}

									<!-- Información de práctica si existe -->
									{#if enrollment.practice}
										<div
											class="mt-3 rounded-lg border border-blue-100 bg-blue-50 p-3"
										>
											<div class="flex items-center gap-2">
												<span class="text-sm font-medium text-blue-900">
													📋 Práctica Profesional Asignada
												</span>
											</div>
										</div>
									{/if}
								</div>
							{/each}
						</div>
					{/if}

					<!-- PROFESOR: Mostrar cursos que imparte -->
				{:else if page.user?.role === "teacher"}
					{#if teacherCoursesData.isLoading}
						<div class="py-8 text-center">
							<div
								class="mx-auto h-8 w-8 animate-spin rounded-full border-4 border-gray-200 border-t-blue-600"
							></div>
							<p class="mt-3 text-sm text-gray-500">Cargando cursos...</p>
						</div>
					{:else if !teacherCoursesData.data?.data || teacherCoursesData.data.data.length === 0}
						<div class="py-8 text-center text-gray-500">
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
							<p class="font-medium">No imparte cursos</p>
							<p class="mt-1 text-sm">Este profesor no tiene cursos asignados</p>
						</div>
					{:else}
						<div class="space-y-3">
							{#each teacherCoursesData.data.data as course (course.id)}
								<div
									class="cursor-pointer rounded-lg border border-gray-200 bg-white p-4 transition-all hover:border-blue-300 hover:shadow-md"
									onclick={() => handleCourseClick(course.id)}
									role="button"
									tabindex="0"
									onkeydown={e => {
										if (e.key === "Enter" || e.key === " ") {
											handleCourseClick(course.id)
										}
									}}
								>
									<h3 class="font-semibold text-gray-900">{course.name}</h3>
									<div class="mt-2 text-xs text-gray-500">
										Código: {course.code}
									</div>
									<div class="mt-2 text-xs text-gray-400">
										Año: {course.year}
									</div>
								</div>
							{/each}
						</div>
					{/if}

					<!-- OTROS ROLES: Sin cursos -->
				{:else}
					<div class="py-8 text-center text-gray-500">
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
								d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
							/>
						</svg>
						<p class="font-medium">Este rol no tiene cursos asignados</p>
					</div>
				{/if}
			</div>
		</section>
	</section>
</section>
