<script lang="ts">
	import type { Enrollment } from "$lib/enrollments/schemas"
	import { protectedApi } from "$api/client"
	import { toast } from "svelte-sonner"
	import { useQueryClient } from "@tanstack/svelte-query"
	import Button from "$lib/shared/components/ui/Button.svelte"

	interface Props {
		enrollment: Enrollment
	}

	let { enrollment }: Props = $props()

	let isOpen = $state(false)
	let isSubmitting = $state(false)
	const queryClient = useQueryClient()

	// Campos del formulario
	let enterpriseName = $state("")
	let description = $state("")
	let location = $state("")
	let supervisorName = $state("")
	let supervisorEmail = $state("")
	let supervisorPhone = $state("")
	let startDate = $state("")
	let endDate = $state("")

	const openModal = (e: MouseEvent) => {
		e.preventDefault()
		e.stopPropagation()
		isOpen = true
	}

	const closeModal = () => {
		isOpen = false
		resetForm()
	}

	const resetForm = () => {
		enterpriseName = ""
		description = ""
		location = ""
		supervisorName = ""
		supervisorEmail = ""
		supervisorPhone = ""
		startDate = ""
		endDate = ""
	}

	const handleSubmit = async (e: Event) => {
		e.preventDefault()

		// Validaciones
		if (
			!enterpriseName ||
			!description ||
			!location ||
			!supervisorName ||
			!supervisorEmail ||
			!supervisorPhone ||
			!startDate ||
			!endDate
		) {
			toast.error("Todos los campos son obligatorios")
			return
		}

		if (new Date(endDate) <= new Date(startDate)) {
			toast.error("La fecha de fin debe ser posterior a la fecha de inicio")
			return
		}

		isSubmitting = true

		try {
			const practiceData = {
				enterpriseName,
				description,
				location,
				supervisorName,
				supervisorEmail,
				supervisorPhone,
				startDate: new Date(startDate).toISOString(),
				endDate: new Date(endDate).toISOString(),
			}

			const response = await protectedApi.post(
				`/enrollments/${enrollment.id}/practice`,
				practiceData
			)

			if (response.data.success) {
				toast.success("Práctica inscrita exitosamente")
				// Invalidar queries de enrollments y courses para refrescar los datos
				queryClient.invalidateQueries({ queryKey: ["enrollments"] })
				queryClient.invalidateQueries({ queryKey: ["courses"] })
				closeModal()
			} else {
				toast.error("Error al inscribir la práctica")
			}
		} catch (error: any) {
			const errorMessage =
				error?.response?.data?.message || "Error al inscribir la práctica"
			toast.error(errorMessage)
		} finally {
			isSubmitting = false
		}
	}
</script>

<button
	onclick={openModal}
	type="button"
	class="inline-flex items-center gap-1.5 rounded-md bg-indigo-50 px-3 py-1.5 text-sm font-medium text-indigo-700 hover:bg-indigo-100"
>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		class="h-4 w-4"
		fill="none"
		viewBox="0 0 24 24"
		stroke="currentColor"
	>
		<path
			stroke-linecap="round"
			stroke-linejoin="round"
			stroke-width="2"
			d="M12 4v16m8-8H4"
		/>
	</svg>
	Inscribir Práctica
</button>

{#if isOpen}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 p-4"
		style="z-index: 9999;"
		onclick={closeModal}
	>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="w-full max-w-xl rounded-lg bg-white p-6 shadow-xl"
			onclick={e => e.stopPropagation()}
		>
			<div class="mb-4 flex items-center justify-between">
				<h3 class="text-lg font-semibold text-gray-900">Inscribir Práctica</h3>
				<button
					type="button"
					onclick={closeModal}
					class="text-gray-400 hover:text-gray-500"
					aria-label="Cerrar modal"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						/>
					</svg>
				</button>
			</div>

			<form onsubmit={handleSubmit} class="space-y-3">
				<div class="grid grid-cols-2 gap-3">
					<div class="col-span-2">
						<label
							for="enterpriseName"
							class="mb-1 block text-sm font-medium text-gray-700"
						>
							Empresa
						</label>
						<input
							type="text"
							id="enterpriseName"
							bind:value={enterpriseName}
							class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
							placeholder="Nombre de la empresa"
							required
						/>
					</div>

					<div class="col-span-2">
						<label
							for="description"
							class="mb-1 block text-sm font-medium text-gray-700"
						>
							Descripción
						</label>
						<textarea
							id="description"
							bind:value={description}
							rows="2"
							class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
							placeholder="Descripción de la práctica"
							required
						></textarea>
					</div>

					<div class="col-span-2">
						<label
							for="location"
							class="mb-1 block text-sm font-medium text-gray-700"
						>
							Ubicación
						</label>
						<input
							type="text"
							id="location"
							bind:value={location}
							class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
							placeholder="Santiago, Chile"
							required
						/>
					</div>

					<div class="col-span-2">
						<label
							for="supervisorName"
							class="mb-1 block text-sm font-medium text-gray-700"
						>
							Nombre Supervisor
						</label>
						<input
							type="text"
							id="supervisorName"
							bind:value={supervisorName}
							class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
							placeholder="Juan Pérez"
							required
						/>
					</div>

					<div class="col-span-2">
						<label
							for="supervisorEmail"
							class="mb-1 block text-sm font-medium text-gray-700"
						>
							Email Supervisor
						</label>
						<input
							type="email"
							id="supervisorEmail"
							bind:value={supervisorEmail}
							class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
							placeholder="supervisor@empresa.com"
							required
						/>
					</div>

					<div class="col-span-2">
						<label
							for="supervisorPhone"
							class="mb-1 block text-sm font-medium text-gray-700"
						>
							Teléfono Supervisor
						</label>
						<input
							type="tel"
							id="supervisorPhone"
							bind:value={supervisorPhone}
							class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
							placeholder="+56912345678"
							required
						/>
					</div>

					<div>
						<label
							for="startDate"
							class="mb-1 block text-sm font-medium text-gray-700"
						>
							Fecha Inicio
						</label>
						<input
							type="date"
							id="startDate"
							bind:value={startDate}
							class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
							required
						/>
					</div>

					<div>
						<label
							for="endDate"
							class="mb-1 block text-sm font-medium text-gray-700"
						>
							Fecha Fin
						</label>
						<input
							type="date"
							id="endDate"
							bind:value={endDate}
							class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
							required
						/>
					</div>
				</div>

				<div class="flex justify-end gap-2 pt-2">
					<Button variant="secondary" onclick={closeModal} text="Cancelar" />
					<button
						type="submit"
						disabled={isSubmitting}
						class="rounded-md bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-700 disabled:cursor-not-allowed disabled:opacity-50"
					>
						{isSubmitting ? "Inscribiendo..." : "Inscribir"}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
