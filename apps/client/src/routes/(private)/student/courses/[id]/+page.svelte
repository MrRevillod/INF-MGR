<script lang="ts">
	import { page } from "$app/stores"
	import { goto } from "$app/navigation"
	import PageTitle from "$lib/shared/components/ui/PageTitle.svelte"
	import Button from "$lib/shared/components/ui/Button.svelte"
	import { getCourseQuery } from "$lib/courses/queries"

	// Obtener el ID del curso desde la URL
	const courseId = $derived($page.params.id ?? "")

	const courseQuery = $derived(getCourseQuery(courseId))
	const { data: courseData, isLoading: courseLoading, error } = $derived(courseQuery)

	function handleBack() {
		goto("/student")
	}
</script>

<section class="space-y-6">
	<div class="flex items-center justify-between">
		<PageTitle
			title={courseData?.data?.name ?? "Cargando..."}
			description={courseData?.data
				? `Código: ${courseData.data.code} | Año: ${courseData.data.year}`
				: "Información del curso"}
		/>
		<Button onclick={handleBack} variant="secondary" text="← Volver" />
	</div>

	{#if courseLoading}
		<div class="flex items-center justify-center py-12">
			<div
				class="h-8 w-8 animate-spin rounded-full border-4 border-gray-200 border-t-blue-600"
			></div>
			<p class="ml-3 text-sm text-gray-500">Cargando información del curso...</p>
		</div>
	{:else if error || !courseData?.data}
		<div class="rounded-lg border border-red-200 bg-red-50 p-6">
			<div class="flex items-start gap-3">
				<svg
					class="h-6 w-6 flex-shrink-0 text-red-600"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
					/>
				</svg>
				<div>
					<h3 class="font-semibold text-red-800">No se pudo cargar el curso</h3>
					<p class="mt-1 text-sm text-red-700">
						El curso con ID "{courseId}" no existe o no tienes permisos para verlo.
					</p>
				</div>
			</div>
		</div>
	{:else}
		<!-- Información del curso -->
		<div class="rounded-lg border border-gray-200 bg-white p-6 shadow-sm">
			<h2 class="text-lg font-semibold text-gray-900">📚 Información del Curso</h2>
			<div class="mt-4 space-y-3 text-sm">
				<p>
					<span class="font-medium text-gray-700">Nombre:</span>
					<span class="ml-2 text-gray-900">{courseData.data.name}</span>
				</p>
				<p>
					<span class="font-medium text-gray-700">Código:</span>
					<span class="ml-2 text-gray-900">{courseData.data.code}</span>
				</p>
				<p>
					<span class="font-medium text-gray-700">Año:</span>
					<span class="ml-2 text-gray-900">{courseData.data.year}</span>
				</p>
				<p>
					<span class="font-medium text-gray-700">Profesor:</span>
					<span class="ml-2 text-gray-900">
						{courseData.data.teacher?.name ?? "Sin asignar"}
					</span>
				</p>
				<p>
					<span class="font-medium text-gray-700">Estado:</span>
					<span
						class="ml-2 inline-flex rounded-full px-2 py-1 text-xs font-semibold {courseData
							.data.courseStatus === 'active'
							? 'bg-green-100 text-green-800'
							: 'bg-gray-100 text-gray-800'}"
					>
						{courseData.data.courseStatus === "active" ? "Activo" : "Completado"}
					</span>
				</p>
			</div>
		</div>

		<!-- Evaluaciones del curso -->
		{#if courseData.data.evaluations && courseData.data.evaluations.length > 0}
			<div class="rounded-lg border border-gray-200 bg-white p-6 shadow-sm">
				<h2 class="text-lg font-semibold text-gray-900">
					📝 Evaluaciones del Curso
				</h2>
				<p class="mt-1 text-sm text-gray-500">
					Lista de evaluaciones configuradas para este curso
				</p>

				<div class="mt-6 space-y-3">
					{#each courseData.data.evaluations as evaluation}
						<div class="flex items-center justify-between rounded-lg bg-gray-50 p-4">
							<div>
								<p class="font-medium text-gray-900">{evaluation.name}</p>
								<p class="text-sm text-gray-500">
									Ponderación: {evaluation.weight}%
								</p>
							</div>
							<div
								class="rounded-md border border-gray-200 bg-white px-3 py-1 text-sm font-medium text-gray-700"
							>
								{evaluation.weight}%
							</div>
						</div>
					{/each}
				</div>
			</div>
		{:else}
			<div class="rounded-lg border border-gray-200 bg-white p-6 shadow-sm">
				<h2 class="text-lg font-semibold text-gray-900">
					📝 Evaluaciones del Curso
				</h2>
				<p class="mt-4 text-center text-gray-500">
					No hay evaluaciones registradas para este curso.
				</p>
			</div>
		{/if}

		<!-- Mensaje informativo sobre limitaciones -->
		<div class="rounded-lg border border-blue-200 bg-blue-50 p-6 shadow-sm">
			<div class="flex items-start gap-3">
				<svg
					class="h-6 w-6 flex-shrink-0 text-blue-600"
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
				<div class="flex-1">
					<h3 class="text-sm font-semibold text-blue-800">Información</h3>
					<div class="mt-2 space-y-1 text-sm text-blue-700">
						<p>
							• Actualmente puedes ver la información general del curso y sus
							evaluaciones.
						</p>
						<p>
							• Para ver tus calificaciones y notas personales, esta funcionalidad
							está en desarrollo.
						</p>
						<p>
							• Contacta con tu profesor o secretaría académica para más información
							sobre tus notas.
						</p>
					</div>
				</div>
			</div>
		</div>
	{/if}
</section>
