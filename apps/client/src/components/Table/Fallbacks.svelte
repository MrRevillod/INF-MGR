<script lang="ts">
	interface FallbacksProps {
		isLoading?: boolean
		isError?: boolean
		data?: any[]
	}

	let { isLoading = false, isError = false, data = [] }: FallbacksProps = $props()
</script>

{#if isLoading}
	<div class="px-6 py-12 text-center">
		<div class="inline-flex items-center gap-3">
			<div
				class="border-t-accent h-6 w-6 animate-spin rounded-full border-2 border-gray-300"
			></div>
			<p class="text-text-muted text-sm">Cargando datos...</p>
		</div>
	</div>
{:else if isError}
	<div class="px-6 py-12 text-center">
		<div class="mb-3 text-red-500">
			<svg
				class="mx-auto h-12 w-12"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"
				/>
			</svg>
		</div>
		<h3 class="text-text-primary mb-2 text-lg font-medium">
			Error al cargar los datos
		</h3>
		<p class="text-text-muted text-sm">
			Ha ocurrido un problema al obtener la información. Por favor, intenta
			nuevamente.
		</p>
	</div>
{:else if !data || data.length === 0}
	<div class="px-6 py-12 text-center">
		<p class="text-text-muted text-sm">No hay datos disponibles</p>
	</div>
{/if}
