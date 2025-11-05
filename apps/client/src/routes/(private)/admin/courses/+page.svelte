<script lang="ts">
	import { goto } from "$app/navigation"
	import PageTitle from "$lib/shared/components/ui/PageTitle.svelte"
	import Button from "$lib/shared/components/ui/Button.svelte"
	import Modal from "$lib/shared/components/Modal.svelte"
	import CreateCourseForm from "$lib/courses/components/CreateCourseForm.svelte"
	import { getAllCoursesQuery } from "$lib/courses/queries"
	import { getUsersQuery } from "$lib/users/queries"

	const coursesQuery = getAllCoursesQuery()
	const { data: coursesRes, isLoading } = $derived(coursesQuery)

	const usersQuery = getUsersQuery(() => ({ page: 1 }))
	const { data: usersRes } = $derived(usersQuery)

	// Estado para controlar el modal de crear curso
	let showCreateModal = $state(false)

	// Filtrar profesores de la lista de usuarios
	const teachers = $derived(
		usersRes?.data?.users
			?.filter(user => user.role === "teacher")
			.map(user => ({ id: user.id, name: user.name })) ?? []
	)

	function handleCourseClick(courseId: string) {
		goto(`/admin/courses/${courseId}`)
	}

	function handleCreateSuccess() {
		showCreateModal = false
	}
</script>

<section class="space-y-6">
	<div class="flex items-center justify-between">
		<PageTitle
			title="Gestión de Cursos"
			description="Administra todos los cursos del sistema"
		/>
		<Button
			onclick={() => (showCreateModal = true)}
			variant="primary"
			text="+ Crear Curso"
		/>
	</div>

	<div class="rounded-lg border border-gray-200 bg-white shadow-sm">
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
				<p class="mt-1 text-sm">
					Haz clic en "Crear Curso" para agregar el primer curso
				</p>
			</div>
		{:else}
			<div class="overflow-hidden">
				<table class="min-w-full divide-y divide-gray-200">
					<thead class="bg-gray-50">
						<tr>
							<th
								class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
							>
								Código
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
							>
								Nombre
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
							>
								Profesor
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
							>
								Año
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
							>
								Estado
							</th>
							<th class="relative px-6 py-3">
								<span class="sr-only">Acciones</span>
							</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-gray-200 bg-white">
						{#each coursesRes.data as course}
							<tr class="hover:bg-gray-50">
								<td
									class="whitespace-nowrap px-6 py-4 text-sm font-medium text-gray-900"
								>
									{course.code}
								</td>
								<td class="px-6 py-4 text-sm text-gray-900">
									{course.name}
								</td>
								<td class="px-6 py-4 text-sm text-gray-500">
									{course.teacher?.name ?? "Sin profesor"}
								</td>
								<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-500">
									{course.year}
								</td>
								<td class="whitespace-nowrap px-6 py-4 text-sm">
									<span
										class="inline-flex rounded-full px-2 text-xs font-semibold leading-5 {course.courseStatus ===
										'active'
											? 'bg-green-100 text-green-800'
											: 'bg-gray-100 text-gray-800'}"
									>
										{course.courseStatus === "active" ? "Activo" : "Completado"}
									</span>
								</td>
								<td
									class="whitespace-nowrap px-6 py-4 text-right text-sm font-medium"
								>
									<button
										onclick={() => handleCourseClick(course.id)}
										class="text-blue-600 hover:text-blue-900"
									>
										Ver detalles
									</button>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</div>
</section>

<!-- Modal para crear curso -->
{#if showCreateModal}
	<Modal
		bind:isOpen={showCreateModal}
		onClose={() => (showCreateModal = false)}
		title="Crear Nuevo Curso"
	>
		{#snippet children()}
			<CreateCourseForm {teachers} onSuccess={handleCreateSuccess} />
		{/snippet}
	</Modal>
{/if}
