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
		goto("/admin/users")
	}

	function handleBack() {
		goto("/admin/users")
	}

	function handleCourseClick(courseId: string) {
		goto(`/admin/courses/${courseId}`)
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
						<div class="space-y-3">
							{#each enrollmentsData.data.data as enrollment (enrollment.id)}
								<div
									class="cursor-pointer rounded-lg border border-gray-200 bg-white p-4 transition-all hover:border-blue-300 hover:shadow-md"
									onclick={() => handleCourseClick(enrollment.course.id)}
									role="button"
									tabindex="0"
									onkeydown={e => {
										if (e.key === "Enter" || e.key === " ") {
											handleCourseClick(enrollment.course.id)
										}
									}}
								>
									<h3 class="font-semibold text-gray-900">
										{enrollment.course.name}
									</h3>
									<div class="mt-2 text-xs text-gray-500">
										Código: {enrollment.course.code}
									</div>
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
