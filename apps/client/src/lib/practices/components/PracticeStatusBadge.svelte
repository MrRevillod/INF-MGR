<script lang="ts">
	import type { PracticeStatus } from "$lib/practices/schemas"

	interface Props {
		status: PracticeStatus
	}

	let { status }: Props = $props()

	const statusConfig = {
		pending: {
			text: "Pendiente",
			bgColor: "bg-yellow-100",
			textColor: "text-yellow-800",
			borderColor: "border-yellow-200",
		},
		approved: {
			text: "Aprobada",
			bgColor: "bg-green-100",
			textColor: "text-green-800",
			borderColor: "border-green-200",
		},
		declined: {
			text: "Rechazada",
			bgColor: "bg-red-100",
			textColor: "text-red-800",
			borderColor: "border-red-200",
		},
	}

	const config = $derived(statusConfig[status])
</script>

<span
	class="inline-flex items-center rounded-full border px-3 py-1 text-xs font-medium {config.bgColor} {config.textColor} {config.borderColor}"
>
	{config.text}
</span>
