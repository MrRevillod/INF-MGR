<script lang="ts" generics="T extends { id: string }">
	import type { TableProps } from "./types"

	import {
		ChevronLeftIcon,
		ChevronRightIcon,
	} from "@fvilers/heroicons-svelte/20/solid"

	import Fallbacks from "./Fallbacks.svelte"

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

	<Fallbacks {isLoading} {isError} data={data ?? []} />
</div>

{#snippet th(label: string)}
	<th
		class="text-text-muted px-6 py-5 text-left text-sm font-medium uppercase tracking-wider"
	>
		{label}
	</th>
{/snippet}
