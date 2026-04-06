<script lang="ts">
	import { api } from "$lib/shared/api/client"
	import type { PageProps } from "./$types"

	let { data }: PageProps = $props()

	let fileInput: HTMLInputElement
	let selectedFile: File | null = $state(null)
	let uploading = $state(false)
	let success = $state(false)
	let error = $state(false)

	function handleFileChange(event: Event) {
		const target = event.target as HTMLInputElement
		if (target.files && target.files.length > 0) {
			selectedFile = target.files[0]
		}
	}

	async function handleSubmit() {
		if (!selectedFile) return

		uploading = true
		error = false

		try {
			const formData = new FormData()
			formData.append("auth_doc", selectedFile)

			const response = await fetch(
				`/api/enrollments/${data.enrollmentId}/practice/${data.practiceId}/authorize`,
				{
					method: "POST",
					body: formData,
				}
			)

			if (response.ok) {
				success = true
			} else {
				error = true
			}
		} catch (e) {
			error = true
		} finally {
			uploading = false
		}
	}
</script>

<svelte:head>
	<title>Autorización de Práctica</title>
</svelte:head>

<div class="flex min-h-screen items-center justify-center bg-gray-50 px-4 py-12">
	<div class="w-full max-w-md rounded-lg bg-white p-8 shadow-lg">
		{#if success}
			<div class="text-center">
				<div
					class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-green-100"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-8 w-8 text-green-600"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M5 13l4 4L19 7"
						/>
					</svg>
				</div>
				<h1 class="mb-2 text-2xl font-bold text-gray-900">Documento Enviado</h1>
				<p class="text-gray-600">
					El documento de autorización ha sido enviado exitosamente. El estudiante y
					la secretaría recibirán una notificación.
				</p>
				<p class="mt-4 text-sm text-gray-500">
					Puede cerrar esta ventana de forma segura.
				</p>
			</div>
		{:else}
			<div>
				<h1 class="mb-6 text-2xl font-bold text-gray-900">
					Autorización de Práctica
				</h1>
				<p class="mb-6 text-gray-600">
					Por favor, suba el documento de autorización firmado y timbrado.
				</p>

				<div class="mb-6">
					<label for="file" class="mb-2 block text-sm font-medium text-gray-700">
						Documento PDF
					</label>
					<input
						id="file"
						type="file"
						accept=".pdf"
						bind:this={fileInput}
						onchange={handleFileChange}
						class="w-full rounded-md border border-gray-300 p-2 text-sm file:mr-4 file:rounded-md file:border-0 file:bg-blue-50 file:px-4 file:py-2 file:text-sm file:font-semibold file:text-blue-700 hover:file:bg-blue-100"
					/>
					{#if selectedFile}
						<p class="mt-2 text-sm text-gray-500">Archivo: {selectedFile.name}</p>
					{/if}
				</div>

				{#if error}
					<div class="mb-4 rounded-md bg-red-50 p-4">
						<p class="text-sm text-red-800">
							Error al enviar el documento. Por favor, intente nuevamente o contacte
							con la secretaría.
						</p>
					</div>
				{/if}

				<button
					type="button"
					onclick={handleSubmit}
					disabled={!selectedFile || uploading}
					class="w-full rounded-md bg-blue-600 px-4 py-2 font-semibold text-white hover:bg-blue-700 disabled:cursor-not-allowed disabled:bg-gray-300"
				>
					{uploading ? "Enviando..." : "Enviar Documento"}
				</button>
			</div>
		{/if}
	</div>
</div>
