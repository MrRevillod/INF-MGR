<script lang="ts">
	interface Props {
		name: string
		code: string
		year?: number
		onclick?: () => void
	}

	let { name, code, year, onclick }: Props = $props()
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<section
	class="flex w-full cursor-pointer flex-col overflow-hidden rounded-md border border-gray-200 bg-white shadow-sm"
	{onclick}
>
	<img src="/asignature.webp" alt={name} class="h-32 w-full object-cover" />
	<div class="space-y-1 p-3 text-sm text-gray-800">
		<p class="font-semibold">{name}</p>
		<p class="text-gray-600">
			{code} - {year ? `Año ${year}` : "Año no especificado"}
		</p>
	</div>
</section>
