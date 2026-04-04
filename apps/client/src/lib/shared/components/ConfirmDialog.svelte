<script lang="ts">
	interface Props {
		isOpen: boolean
		title: string
		message: string
		confirmText?: string
		cancelText?: string
		variant?: "danger" | "warning" | "info"
		onConfirm: () => void
		onCancel: () => void
	}

	let {
		isOpen = $bindable(false),
		title,
		message,
		confirmText = "Confirmar",
		cancelText = "Cancelar",
		variant = "danger",
		onConfirm,
		onCancel,
	}: Props = $props()

	const variantStyles = {
		danger: "bg-red-600 hover:bg-red-700",
		warning: "bg-yellow-600 hover:bg-yellow-700",
		info: "bg-blue-600 hover:bg-blue-700",
	}
</script>

{#if isOpen}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
		onclick={onCancel}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="w-full max-w-lg rounded-lg bg-white p-6 shadow-xl"
			onclick={e => e.stopPropagation()}
			role="document"
		>
			<h2 class="mb-4 text-xl font-semibold text-gray-900">{title}</h2>
			<p class="mb-6 text-sm leading-relaxed text-gray-600">{message}</p>

			<div class="flex justify-end gap-3">
				<button
					type="button"
					onclick={onCancel}
					class="rounded border border-gray-300 bg-white px-4 py-2 text-gray-700 hover:bg-gray-50"
				>
					{cancelText}
				</button>
				<button
					type="button"
					onclick={onConfirm}
					class="rounded px-4 py-2 text-white {variantStyles[variant]}"
				>
					{confirmText}
				</button>
			</div>
		</div>
	</div>
{/if}
