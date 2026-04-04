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
	<div class="modal-overlay" onclick={onClose}>
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

<style>
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}
</style>
