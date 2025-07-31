<script lang="ts">
	interface Props {
		title: string
		description: string
	}

	const { title, description }: Props = $props()
</script>

<details
	class="group rounded-md border border-[color:var(--color-border)] bg-[color:var(--color-surface)] transition-all duration-300"
>
	<summary
		class="flex cursor-pointer list-none items-center justify-between px-4
		py-3 font-semibold text-[color:var(--color-text-primary)] transition-colors duration-300 hover:bg-[color:var(--color-hover-bg)]"
	>
		<span>{title}</span>
		<svg
			class="ml-2 h-4 w-4 transform text-[color:var(--color-text-muted)] transition-transform duration-300 group-open:rotate-180"
			fill="none"
			stroke="currentColor"
			viewBox="0 0 24 24"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M19 9l-7 7-7-7"
			/>
		</svg>
	</summary>

	<div class="px-4 py-4 text-[color:var(--color-text-secondary)]">
		{description}
	</div>
</details>
