<script lang="ts">
	import { page } from "$app/stores"
	import { goto } from "$app/navigation"
	import PageTitle from "$lib/shared/components/ui/PageTitle.svelte"
	import Button from "$lib/shared/components/ui/Button.svelte"
	import Modal from "$lib/shared/components/Modal.svelte"
	import UpdateCourseForm from "$lib/courses/components/UpdateCourseForm.svelte"
	import EnrollStudentForm from "$lib/courses/components/EnrollStudentForm.svelte"
	import DeleteCourseButton from "$lib/courses/components/DeleteCourseButton.svelte"
	import DeleteEnrollmentButton from "$lib/courses/components/DeleteEnrollmentButton.svelte"
	import UpdateEnrollmentButton from "$lib/courses/components/UpdateEnrollmentButton.svelte"
	import ViewAuthorizationButton from "$lib/enrollments/components/ViewAuthorizationButton.svelte"
	import CreatePracticeButton from "$lib/enrollments/components/CreatePracticeButton.svelte"
	import { getCourseQuery } from "$lib/courses/queries"
	import { getCourseEnrollmentsQuery } from "$lib/enrollments/queries"
	import { getUsersQuery } from "$lib/users/queries"

	// Obtener el ID del curso desde la URL
	const courseId = $derived($page.params.id ?? "")

	// Cargar datos del curso
	const courseQuery = $derived(getCourseQuery(courseId))
	const { data: courseRes, isLoading } = $derived(courseQuery)

	// Cargar estudiantes inscritos
	const enrollmentsQuery = $derived(getCourseEnrollmentsQuery(courseId))
	const { data: enrollmentsRes, isLoading: isLoadingEnrollments } =
		$derived(enrollmentsQuery)

	// Cargar profesores (para el formulario de edición)
	const teachersQuery = getUsersQuery(() => ({ page: 1, role: "teacher" }))
	const { data: teachersRes } = $derived(teachersQuery)

	// Cargar estudiantes (para el formulario de inscripción)
	const studentsQuery = getUsersQuery(() => ({ page: 1, role: "student" }))
	const { data: studentsRes } = $derived(studentsQuery)

	// Estados para controlar los modales
	let showEditModal = $state(false)
	let showEnrollModal = $state(false)

	// Curso actual (para evitar problemas de undefined en el modal)
	const currentCourse = $derived(courseRes?.data)

	// Obtener profesores directamente de la consulta filtrada
	const teachers = $derived(
		teachersRes?.data?.users?.map(user => ({ id: user.id, name: user.name })) ?? []
	)

	const enrolledStudentIds = $derived(
		enrollmentsRes?.data?.map(enrollment => enrollment.student.id) ?? []
	)

	// Obtener estudiantes no inscritos
	const availableStudents = $derived(
		studentsRes?.data?.users
			?.filter(student => !enrolledStudentIds.includes(student.id))
			.map(user => ({ id: user.id, name: user.name, rut: user.rut })) ?? []
	)

	function handleBack() {
		goto("/admin/courses")
	}

	function handleStudentClick(studentId: string) {
		goto(`/admin/users/${studentId}`)
	}

	function handleEditSuccess() {
		showEditModal = false
	}

	function handleEnrollSuccess() {
		showEnrollModal = false
	}

	function calculateWeightedAverage(
		scores: Array<{ evaluationId: string; score: number }>,
		evaluations: Array<{ id: string; weight: number }>
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
	{#if isLoading}
		<div class="flex items-center justify-center py-12">
			<div
				class="h-8 w-8 animate-spin rounded-full border-4 border-gray-200 border-t-blue-600"
			></div>
			<p class="ml-3 text-sm text-gray-500">Cargando curso...</p>
		</div>
	{:else if !courseRes?.data}
		<div class="py-12 text-center text-gray-500">
			<p class="font-medium">Curso no encontrado</p>
		</div>
	{:else}
		<!-- Header con título y botones de acción -->
		<div class="flex items-center justify-between">
			<PageTitle
				title={`Curso: ${courseRes.data.name}`}
				description={`Código: ${courseRes.data.code}`}
			/>

			<div class="flex gap-2">
				<Button onclick={handleBack} variant="secondary" text="← Volver" />
				<Button
					onclick={() => (showEditModal = true)}
					variant="primary"
					text="Editar Curso"
				/>
				<DeleteCourseButton
					courseId={courseRes.data.id}
					courseName={courseRes.data.name}
				/>
			</div>
		</div>

		<!-- Información del curso -->
		<section class="rounded-lg border border-gray-200 bg-white shadow-sm">
			<div class="border-b border-gray-200 px-6 py-4">
				<h2 class="text-lg font-semibold text-gray-900">Información del Curso</h2>
			</div>
			<div class="px-6 py-4">
				<dl class="grid grid-cols-1 gap-4 sm:grid-cols-2">
					<div>
						<dt class="text-sm font-medium text-gray-500">Código</dt>
						<dd class="mt-1 text-sm font-semibold text-gray-900">
							{courseRes.data.code}
						</dd>
					</div>
					<div>
						<dt class="text-sm font-medium text-gray-500">Nombre</dt>
						<dd class="mt-1 text-sm text-gray-900">{courseRes.data.name}</dd>
					</div>
					<div>
						<dt class="text-sm font-medium text-gray-500">Profesor</dt>
						<dd class="mt-1 text-sm text-gray-900">
							{courseRes.data.teacher?.name ?? "Sin asignar"}
						</dd>
					</div>
					<div>
						<dt class="text-sm font-medium text-gray-500">Año</dt>
						<dd class="mt-1 text-sm text-gray-900">{courseRes.data.year}</dd>
					</div>
					<div>
						<dt class="text-sm font-medium text-gray-500">Estado</dt>
						<dd class="mt-1">
							<span
								class="inline-flex rounded-full px-2 text-xs font-semibold leading-5 {courseRes
									.data.courseStatus === 'active'
									? 'bg-green-100 text-green-800'
									: 'bg-gray-100 text-gray-800'}"
							>
								{courseRes.data.courseStatus === "active" ? "Activo" : "Completado"}
							</span>
						</dd>
					</div>
				</dl>
			</div>
		</section>

		<!-- Estudiantes inscritos -->
		<section class="rounded-lg border border-gray-200 bg-white shadow-sm">
			<div
				class="flex items-center justify-between border-b border-gray-200 px-6 py-4"
			>
				<div>
					<h2 class="text-lg font-semibold text-gray-900">Estudiantes Inscritos</h2>
					<p class="text-sm text-gray-500">
						{enrollmentsRes?.data?.length ?? 0} estudiante(s) inscrito(s)
					</p>
				</div>
				<Button
					onclick={() => (showEnrollModal = true)}
					variant="primary"
					text="+ Inscribir Estudiante"
				/>
			</div>

			{#if isLoadingEnrollments}
				<div class="flex items-center justify-center py-12">
					<div
						class="h-8 w-8 animate-spin rounded-full border-4 border-gray-200 border-t-blue-600"
					></div>
					<p class="ml-3 text-sm text-gray-500">Cargando estudiantes...</p>
				</div>
			{:else if !enrollmentsRes?.data || enrollmentsRes.data.length === 0}
				<div class="py-12 text-center text-gray-500">
					<p class="font-medium">No hay estudiantes inscritos</p>
					<p class="mt-1 text-sm">
						Haz clic en "Inscribir Estudiante" para agregar estudiantes
					</p>
				</div>
			{:else}
				<div class="overflow-x-auto">
					<table class="min-w-full divide-y divide-gray-200">
						<thead class="bg-gray-50">
							<tr>
								<th
									class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
								>
									RUT
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
								>
									Nombre
								</th>
								<th
									class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
								>
									Email
								</th>
								<!-- Columnas dinámicas de evaluaciones -->
								{#if currentCourse?.evaluations && currentCourse.evaluations.length > 0}
									{#each currentCourse.evaluations as evaluation}
										<th
											class="px-4 py-3 text-center text-xs font-medium uppercase tracking-wider text-gray-500"
											title={`Peso: ${evaluation.weight}%`}
										>
											{evaluation.name}
											<br />
											<span class="text-[10px] font-normal text-gray-400"
												>({evaluation.weight}%)</span
											>
										</th>
									{/each}
									<th
										class="bg-gray-100 px-4 py-3 text-center text-xs font-medium uppercase tracking-wider text-gray-700"
									>
										Promedio
									</th>
								{/if}

								<th class="relative px-6 py-3">
									<span class="sr-only">Acciones</span>
								</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-gray-200 bg-white">
							{#each enrollmentsRes.data as enrollment (enrollment.id)}
								{@const scores = enrollment.studentScores || []}
								{@const average = calculateWeightedAverage(
									scores,
									currentCourse?.evaluations || []
								)}

								<tr class="hover:bg-gray-50">
									<td
										class="whitespace-nowrap px-6 py-4 text-sm font-medium text-gray-900"
									>
										{enrollment.student.rut}
									</td>
									<td class="px-6 py-4 text-sm text-gray-900">
										<button
											onclick={() => handleStudentClick(enrollment.student.id)}
											class="text-blue-600 hover:text-blue-900 hover:underline"
										>
											{enrollment.student.name}
										</button>
									</td>
									<td class="px-6 py-4 text-sm text-gray-500">
										{enrollment.student.email}
									</td>

									<!-- Mostrar calificaciones -->
									{#if currentCourse?.evaluations && currentCourse.evaluations.length > 0}
										{#each currentCourse.evaluations as evaluation}
											{@const score = scores.find(
												s => s.evaluationId === evaluation.id
											)}
											<td class="px-4 py-4 text-center text-sm">
												{#if score && score.score > 0}
													<span
														class="inline-flex rounded-full px-2 py-1 text-xs font-semibold {score.score >=
														4.0
															? 'bg-green-100 text-green-800'
															: 'bg-red-100 text-red-800'}"
													>
														{score.score.toFixed(1)}
													</span>
												{:else}
													<span class="text-xs text-gray-400">-</span>
												{/if}
											</td>
										{/each}

										<!-- Promedio ponderado -->
										<td class="bg-gray-50 px-4 py-4 text-center text-sm font-bold">
											{#if average !== null}
												<span
													class="inline-flex rounded-full px-3 py-1 text-sm font-bold {average >=
													4.0
														? 'bg-green-200 text-green-900'
														: 'bg-red-200 text-red-900'}"
												>
													{average.toFixed(1)}
												</span>
											{:else}
												<span class="text-xs text-gray-400">Sin calificar</span>
											{/if}
										</td>
									{/if}

									<td
										class="whitespace-nowrap px-6 py-4 text-right text-sm font-medium"
									>
										<div class="flex justify-end gap-2">
											{#if currentCourse?.evaluations && currentCourse.evaluations.length > 0}
												<UpdateEnrollmentButton
													{enrollment}
													evaluations={currentCourse.evaluations}
												/>
											{/if}
											{#if enrollment.practiceId}
												<ViewAuthorizationButton {enrollment} />
											{:else}
												<CreatePracticeButton {enrollment} />
											{/if}
											<DeleteEnrollmentButton
												enrollmentId={enrollment.id}
												studentName={enrollment.student.name}
											/>
										</div>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{/if}
		</section>
	{/if}
</section>

<!-- Modal para editar curso -->
{#if showEditModal && currentCourse}
	<Modal
		bind:isOpen={showEditModal}
		onClose={() => (showEditModal = false)}
		title="Editar Curso"
	>
		{#snippet children()}
			<UpdateCourseForm
				course={currentCourse}
				{teachers}
				onSuccess={handleEditSuccess}
			/>
		{/snippet}
	</Modal>
{/if}

<!-- Modal para inscribir estudiante -->
{#if showEnrollModal}
	<Modal
		bind:isOpen={showEnrollModal}
		onClose={() => (showEnrollModal = false)}
		title="Inscribir Estudiante"
	>
		{#snippet children()}
			<EnrollStudentForm
				{courseId}
				students={availableStudents}
				onSuccess={handleEnrollSuccess}
			/>
		{/snippet}
	</Modal>
{/if}
