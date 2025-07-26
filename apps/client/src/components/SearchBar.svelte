<script lang="ts">
	import { MagnifyingGlassIcon } from "@fvilers/heroicons-svelte/20/solid"

	interface SearchBarProps {
		search: string
		placeholder?: string
		onSearchChange: (searchValue: string) => void
	}

	const {
		search,
		placeholder = "Buscar...",
		onSearchChange,
	}: SearchBarProps = $props()

	const handleInput = (e: Event) => {
		const input = e.target as HTMLInputElement
		const searchValue = input.value.trim().toLocaleLowerCase()
		onSearchChange(searchValue)
	}
</script>

<div class="relative">
	<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
		<MagnifyingGlassIcon class="text-text-muted h-5 w-5" />
	</div>
	<input
		type="text"
		value={search}
		oninput={handleInput}
		{placeholder}
		class="bg-surface border-border focus:ring-focus-ring hover:border-border-hover placeholder:text-text-muted block w-full rounded-lg border py-2.5
		       pl-10 pr-4 text-sm
		       transition-colors duration-200 focus:border-transparent
		       focus:ring-2"
	/>
</div>
