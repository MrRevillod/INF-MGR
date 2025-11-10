<script lang="ts">
	import type { Snippet } from "svelte"

	interface Props {
		isOpen: boolean
		onClose: () => void
		title?: string
		children: Snippet
	}

	let { isOpen = $bindable(false), onClose, title, children }: Props = $props()
</script>

{#if isOpen}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		onclick={onClose}
	>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="w-full max-w-md rounded-lg bg-white p-6 shadow-lg"
			onclick={e => e.stopPropagation()}
		>
			{#if title}
				<h2 class="mb-4 text-xl font-bold">{title}</h2>
			{/if}
			<div>
				{@render children()}
			</div>
		</div>
	</div>
{/if}
