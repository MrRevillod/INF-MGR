<script lang="ts">
	import type { PageData } from "./$types"
	import { goto } from "$app/navigation"
	import PageTitle from "$lib/shared/components/ui/PageTitle.svelte"
	import Button from "$lib/shared/components/ui/Button.svelte"

	const { data }: { data: PageData } = $props()

	let courseId = $state("")

	function handleViewCourse() {
		if (courseId.trim()) {
			goto(`/student/courses/${courseId.trim()}`)
		}
	}
</script>

<section class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<PageTitle
			title={`¡Hola, ${data.user?.name ?? ""}! 👋`}
			description="Panel de estudiante"
		/>
	</div>

	<!-- Card de Información del Estudiante -->
	<div class="rounded-lg border border-blue-200 bg-blue-50 p-6 shadow-sm">
		<div class="flex items-start justify-between">
			<div class="flex-1">
				<h2 class="text-lg font-semibold text-gray-900">Mi Información</h2>
				<div class="mt-3 space-y-2 text-sm">
					<p class="text-gray-700">
						<span class="font-medium">Nombre:</span>
						{data.user?.name ?? ""}
					</p>
					<p class="text-gray-700">
						<span class="font-medium">Email:</span>
						{data.user?.email ?? ""}
					</p>
					<p class="text-gray-700">
						<span class="font-medium">RUT:</span>
						{data.user?.rut ?? ""}
					</p>
				</div>
			</div>
		</div>
	</div>

	<!-- Ver Curso por ID -->
	<div class="rounded-lg border border-gray-200 bg-white shadow-sm">
		<div class="border-b border-gray-200 bg-gray-50 px-6 py-4">
			<h2 class="text-lg font-semibold text-gray-900">
				📚 Ver Información de un Curso
			</h2>
			<p class="mt-1 text-sm text-gray-500">
				Ingresa el ID del curso para ver sus detalles
			</p>
		</div>

		<div class="p-6">
			<form
				onsubmit={e => {
					e.preventDefault()
					handleViewCourse()
				}}
				class="space-y-4"
			>
				<div>
					<label for="courseId" class="block text-sm font-medium text-gray-700">
						ID del Curso
					</label>
					<div class="mt-1 flex gap-3">
						<input
							id="courseId"
							type="text"
							bind:value={courseId}
							placeholder="Ejemplo: 550e8400-e29b-41d4-a716-446655440000"
							class="block flex-1 rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 sm:text-sm"
						/>
						<Button
							onclick={handleViewCourse}
							variant="primary"
							disabled={!courseId.trim()}
							text="Ver Curso"
						/>
					</div>
					<p class="mt-2 text-xs text-gray-500">
						Solicita el ID del curso a tu profesor o secretaría académica
					</p>
				</div>
			</form>
		</div>
	</div>

	<!-- Información sobre limitaciones actuales -->
	<div class="rounded-lg border border-yellow-200 bg-yellow-50 p-6 shadow-sm">
		<div class="flex items-start gap-3">
			<svg
				class="h-6 w-6 flex-shrink-0 text-yellow-600"
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
				<h3 class="text-sm font-semibold text-yellow-800">
					Funcionalidades Actuales
				</h3>
				<div class="mt-2 space-y-1 text-sm text-yellow-700">
					<p>
						• Actualmente solo puedes ver los detalles de un curso individual
						ingresando su ID
					</p>
					<p>• Las siguientes funcionalidades están en desarrollo:</p>
					<ul class="ml-6 mt-1 list-disc space-y-1">
						<li>Ver listado de tus cursos inscritos</li>
						<li>Ver tus inscripciones y notas</li>
						<li>Ver promedio general</li>
					</ul>
				</div>
			</div>
		</div>
	</div>
</section>
