<script lang="ts" generics="T extends { id: string }">
	import {
		ChevronLeftIcon,
		ChevronRightIcon,
	} from "@fvilers/heroicons-svelte/20/solid"

	export interface TableColumn<T> {
		key: keyof T
		label: string
		formatter?: (value: T) => string
	}

	export interface TableProps<T> {
		data: T[] | null
		columns: TableColumn<T>[]
		isLoading?: boolean
		isError?: boolean
		pagination: {
			currentPage: number
			totalPages?: number
			totalUsers?: number
			hasNext?: boolean
			hasPrevious?: boolean
			onPageChange: (page: number) => void
		}
		onDetailsClick?: (item: T) => void
	}

	let {
		data,
		columns,
		isLoading = false,
		isError = false,
		pagination,
		onDetailsClick,
	}: TableProps<T> = $props()
</script>

<div class="bg-surface border-border overflow-hidden rounded-lg border shadow-sm">
	<table class="w-full">
		<thead class="border-border border-b bg-gray-50">
			<tr>
				{#each columns as column}
					{@render th(column.label)}
				{/each}
				{@render th("Detalles")}
			</tr>
		</thead>
		<tbody class="bg-surface divide-border divide-y">
			{#each data ?? [] as item (item.id)}
				<tr class="hover:bg-hover-bg transition-colors duration-150">
					{#each columns as column}
						<td class="text-text-primary px-6 py-3 text-sm">
							{#if column.formatter}
								{column.formatter(item)}
							{:else}
								{item[column.key]}
							{/if}
						</td>
					{/each}
					<td class="px-6 py-3">
						<button
							type="button"
							onclick={() => onDetailsClick && onDetailsClick(item)}
							class="text-text-muted hover:text-text-primary hover:bg-hover-bg
							focus:ring-focus-ring inline-flex h-8 w-8 cursor-pointer items-center
							justify-center rounded-md transition-colors duration-200 focus:outline-none focus:ring-2"
							aria-label="Ver detalles"
						>
							...
						</button>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>

	<section class="flex w-full items-center justify-end gap-2 p-4">
		<div class="flex items-center gap-4">
			<button
				onclick={() => pagination.onPageChange(pagination.currentPage - 1)}
				disabled={!pagination.hasPrevious || isLoading}
				class="cursor-pointer overflow-hidden rounded-lg disabled:cursor-not-allowed disabled:opacity-50"
			>
				<ChevronLeftIcon class="text-active-bg bg-accent h-9 w-9" />
			</button>
			<span class="text-text-primary text-sm">
				{#if !isLoading}
					Página {pagination.currentPage} de {pagination.totalPages || 1}
				{/if}
			</span>
			<button
				onclick={() => pagination.onPageChange(pagination.currentPage + 1)}
				disabled={!pagination.hasNext || isLoading}
				class="cursor-pointer overflow-hidden rounded-lg disabled:cursor-not-allowed disabled:opacity-50"
			>
				<ChevronRightIcon class="text-active-bg bg-accent h-9 w-9" />
			</button>
		</div>
	</section>

	<!--  Fallbacks for loading and error states -->

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
</div>

{#snippet th(label: string)}
	<th
		class="text-text-muted px-6 py-5 text-left text-sm font-medium uppercase tracking-wider"
	>
		{label}
	</th>
{/snippet}
