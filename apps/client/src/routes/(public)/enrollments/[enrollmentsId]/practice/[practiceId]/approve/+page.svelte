<script lang="ts">
	import type { PageProps } from "./$types"

	let { data }: PageProps = $props()
</script>

<div class="flex min-h-screen items-center justify-center bg-gray-50 px-4">
	<div class="w-full max-w-md rounded-lg bg-white p-8 shadow-lg">
		{#if data.success}
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
				<h1 class="mb-2 text-2xl font-bold text-gray-900">Práctica Aprobada</h1>
				<p class="text-gray-600">
					La práctica ha sido aprobada exitosamente. El estudiante recibirá una
					notificación por correo electrónico con los próximos pasos.
				</p>
			</div>
		{:else}
			<div class="text-center">
				<div
					class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-red-100"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-8 w-8 text-red-600"
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
				</div>
				<h1 class="mb-2 text-2xl font-bold text-gray-900">Error</h1>
				<p class="text-gray-600">
					No se pudo aprobar la práctica. Por favor, contacte con la secretaría de la
					carrera.
				</p>
			</div>
		{/if}
	</div>
</div>
