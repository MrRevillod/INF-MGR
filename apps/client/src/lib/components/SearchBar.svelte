<script lang="ts">
	import { MagnifyingGlassIcon } from "@fvilers/heroicons-svelte/20/solid"

	interface SearchBarProps {
		search: string
		placeholder?: string
	}

	let { search = $bindable(), placeholder = "Buscar..." }: SearchBarProps = $props()

	let inputValue = $state("")

	$effect(() => {
		search = inputValue.trim().toLowerCase()
	})
</script>

<div class="relative">
	<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
		<MagnifyingGlassIcon class="text-text-muted h-5 w-5" />
	</div>
	<input
		type="text"
		bind:value={inputValue}
		{placeholder}
		class="bg-surface border-border focus:ring-focus-ring hover:border-border-hover placeholder:text-text-muted block w-full rounded-lg border py-2.5
		       pl-10 pr-4 text-sm
		       transition-colors duration-200 focus:border-transparent
		       focus:ring-2"
	/>
</div>
