<script lang="ts" generics="T extends { id: string }">
	import type { Component } from "svelte"
	import { DropdownMenu, type WithoutChild } from "bits-ui"

	type Props = DropdownMenu.RootProps & {
		buttonText: string
		rowValue: T
		items: Array<{
			label: string
			icon: Component
			func: (value: T) => void
		}>
		contentProps?: WithoutChild<DropdownMenu.ContentProps>
	}

	let {
		open = $bindable(false),
		rowValue,
		children,
		buttonText,
		items,
		contentProps,
		...restProps
	}: Props = $props()
</script>

<DropdownMenu.Root bind:open {...restProps}>
	<DropdownMenu.Trigger
		class="text-text-muted hover:text-text-primary hover:bg-hover-bg focus:ring-focus-ring inline-flex h-8 w-8 
		       items-center justify-center rounded-md transition-colors duration-200 
		       focus:outline-none focus:ring-2"
	>
		{buttonText}
	</DropdownMenu.Trigger>
	<DropdownMenu.Portal>
		<DropdownMenu.Content
			{...contentProps}
			class="bg-surface border-border data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95
			       z-50 min-w-48 
			       rounded-lg border 
			       py-1 shadow-lg"
		>
			<DropdownMenu.Group aria-label={buttonText}>
				{#each items as item}
					<DropdownMenu.Item
						onclick={() => item.func(rowValue)}
						class="text-text-primary hover:bg-hover-bg focus:bg-hover-bg flex cursor-pointer items-center px-3 
						       py-2 text-sm transition-colors duration-150 
						       focus:outline-none"
					>
						<item.icon class="text-text-muted mr-3 h-4 w-4" />
						{item.label}
					</DropdownMenu.Item>
				{/each}
			</DropdownMenu.Group>
		</DropdownMenu.Content>
	</DropdownMenu.Portal>
</DropdownMenu.Root>
