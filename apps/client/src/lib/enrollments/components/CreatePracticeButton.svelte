<script lang="ts">
	import type { Enrollment } from "$lib/enrollments/schemas"
	import { CreatePracticeSchema } from "$lib/enrollments/schemas"
	import { createPracticeMutation } from "$lib/enrollments/mutations"
	import { createForm, Field, validate } from "@formisch/svelte"
	import { toast } from "svelte-sonner"
	import Button from "$lib/shared/components/ui/Button.svelte"
	import Modal from "$lib/shared/components/Modal.svelte"

	interface Props {
		enrollment: Enrollment
	}

	let { enrollment }: Props = $props()

	let isOpen = $state(false)

	const mutation = createPracticeMutation(enrollment.id)

	const form = createForm({
		schema: CreatePracticeSchema,
		initialInput: {
			enterpriseName: "",
			description: "",
			location: "",
			supervisorName: "",
			supervisorEmail: "",
			supervisorPhone: "",
			startDate: "",
			endDate: "",
		},
	})

	const openModal = (e: MouseEvent) => {
		e.preventDefault()
		e.stopPropagation()
		isOpen = true
	}

	const closeModal = () => {
		isOpen = false
	}

	async function onSubmit() {
		const result = await validate(form)

		if (!result.success) {
			toast.error("Por favor corrige los errores del formulario")
			return
		}

		// Validar fechas
		if (new Date(result.output.endDate) <= new Date(result.output.startDate)) {
			toast.error("La fecha de fin debe ser posterior a la fecha de inicio")
			return
		}

		try {
			const apiResult = await mutation.mutateAsync(result.output)

			if (apiResult.success) {
				toast.success("Práctica inscrita exitosamente")
				closeModal()
			} else {
				toast.error(
					typeof apiResult.error === "string"
						? apiResult.error
						: "Error al inscribir la práctica"
				)
			}
		} catch (error: any) {
			const errorMessage =
				error?.response?.data?.message || "Error al inscribir la práctica"
			toast.error(errorMessage)
		}
	}

	function handleFormSubmit(e: SubmitEvent) {
		e.preventDefault()
		onSubmit()
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

<Modal bind:isOpen onClose={closeModal} title="Inscribir Práctica">
	{#snippet children()}
		<form onsubmit={handleFormSubmit} class="space-y-3">
			<div class="grid grid-cols-2 gap-3">
				<Field of={form} path={["enterpriseName"]}>
					{#snippet children(field)}
						<div class="col-span-2">
							<label
								for="enterpriseName"
								class="mb-1 block text-sm font-medium text-gray-700"
							>
								Empresa *
							</label>
							<input
								type="text"
								id="enterpriseName"
								value={field.input ?? ""}
								{...field.props}
								class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
								placeholder="Nombre de la empresa"
							/>
							{#if field.errors}
								<p class="mt-1 text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</div>
					{/snippet}
				</Field>

				<Field of={form} path={["description"]}>
					{#snippet children(field)}
						<div class="col-span-2">
							<label
								for="description"
								class="mb-1 block text-sm font-medium text-gray-700"
							>
								Descripción *
							</label>
							<textarea
								id="description"
								value={field.input ?? ""}
								{...field.props}
								rows="2"
								class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
								placeholder="Descripción de la práctica"
							></textarea>
							{#if field.errors}
								<p class="mt-1 text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</div>
					{/snippet}
				</Field>

				<Field of={form} path={["location"]}>
					{#snippet children(field)}
						<div class="col-span-2">
							<label
								for="location"
								class="mb-1 block text-sm font-medium text-gray-700"
							>
								Ubicación *
							</label>
							<input
								type="text"
								id="location"
								value={field.input ?? ""}
								{...field.props}
								class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
								placeholder="Santiago, Chile"
							/>
							{#if field.errors}
								<p class="mt-1 text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</div>
					{/snippet}
				</Field>

				<Field of={form} path={["supervisorName"]}>
					{#snippet children(field)}
						<div class="col-span-2">
							<label
								for="supervisorName"
								class="mb-1 block text-sm font-medium text-gray-700"
							>
								Nombre Supervisor *
							</label>
							<input
								type="text"
								id="supervisorName"
								value={field.input ?? ""}
								{...field.props}
								class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
								placeholder="Juan Pérez"
							/>
							{#if field.errors}
								<p class="mt-1 text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</div>
					{/snippet}
				</Field>

				<Field of={form} path={["supervisorEmail"]}>
					{#snippet children(field)}
						<div class="col-span-2">
							<label
								for="supervisorEmail"
								class="mb-1 block text-sm font-medium text-gray-700"
							>
								Email Supervisor *
							</label>
							<input
								type="email"
								id="supervisorEmail"
								value={field.input ?? ""}
								{...field.props}
								class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
								placeholder="supervisor@empresa.com"
							/>
							{#if field.errors}
								<p class="mt-1 text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</div>
					{/snippet}
				</Field>

				<Field of={form} path={["supervisorPhone"]}>
					{#snippet children(field)}
						<div class="col-span-2">
							<label
								for="supervisorPhone"
								class="mb-1 block text-sm font-medium text-gray-700"
							>
								Teléfono Supervisor *
							</label>
							<input
								type="tel"
								id="supervisorPhone"
								value={field.input ?? ""}
								{...field.props}
								class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
								placeholder="+56912345678"
							/>
							{#if field.errors}
								<p class="mt-1 text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</div>
					{/snippet}
				</Field>

				<Field of={form} path={["startDate"]}>
					{#snippet children(field)}
						<div>
							<label
								for="startDate"
								class="mb-1 block text-sm font-medium text-gray-700"
							>
								Fecha Inicio *
							</label>
							<input
								type="date"
								id="startDate"
								value={field.input ?? ""}
								{...field.props}
								class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
							/>
							{#if field.errors}
								<p class="mt-1 text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</div>
					{/snippet}
				</Field>

				<Field of={form} path={["endDate"]}>
					{#snippet children(field)}
						<div>
							<label
								for="endDate"
								class="mb-1 block text-sm font-medium text-gray-700"
							>
								Fecha Fin *
							</label>
							<input
								type="date"
								id="endDate"
								value={field.input ?? ""}
								{...field.props}
								class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
							/>
							{#if field.errors}
								<p class="mt-1 text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</div>
					{/snippet}
				</Field>
			</div>

			<div class="flex justify-end gap-2 pt-2">
				<Button variant="secondary" onclick={closeModal} text="Cancelar" />
				<button
					type="submit"
					disabled={form.isSubmitting}
					class="rounded-md bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-700 disabled:cursor-not-allowed disabled:opacity-50"
				>
					{form.isSubmitting ? "Inscribiendo..." : "Inscribir"}
				</button>
			</div>
		</form>
	{/snippet}
</Modal>
