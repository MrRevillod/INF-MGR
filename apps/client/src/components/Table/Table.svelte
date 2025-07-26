<script lang="ts" generics="T extends { id: string }">
	import type { TableProps, TableColumn } from "./types"

	import {
		ChevronLeftIcon,
		ChevronRightIcon,
	} from "@fvilers/heroicons-svelte/20/solid"

	import Dropdown from "../Dropdown.svelte"
	import Fallbacks from "./Fallbacks.svelte"

	let {
		data,
		columns,
		actions,
		isLoading = false,
		isError = false,
		onPageChange,
	}: TableProps<T> = $props()
</script>

<div class="bg-surface border-border overflow-hidden rounded-lg border shadow-sm">
	<table class="w-full">
		<thead class="border-border border-b bg-gray-50">
			<tr>
				{#each columns as column}
					<th
						class="text-text-muted px-6 py-5 text-left text-sm font-medium uppercase tracking-wider"
					>
						{column.label}
					</th>
				{/each}
				<th
					class="text-text-muted px-6 py-5 text-left text-sm font-medium uppercase tracking-wider"
				>
					Acciones
				</th>
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
						<Dropdown
							buttonText="•••"
							rowValue={item}
							items={actions.map(action => ({
								label: action.label,
								icon: action.icon,
								func: () => action.func(item),
							}))}
						/>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>

	<section class="flex w-full items-end justify-end gap-2">
		<button
			onclick={() => onPageChange("prev")}
			class="overflow-hidden rounded-l-lg"
		>
			<ChevronLeftIcon class="text-active-bg bg-accent h-9 w-9" />
		</button>
		<button 
			onclick={() => onPageChange("next")}
			class="overflow-hidden rounded-r-lg"
		>
			<ChevronRightIcon class="text-active-bg bg-accent h-9 w-9" />
		</button>
	</section>

	<Fallbacks {isLoading} {isError} data={data ?? []} />
</div>
