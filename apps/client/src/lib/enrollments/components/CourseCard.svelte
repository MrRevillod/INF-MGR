<script lang="ts">
	interface Props {
		code: string
		name: string
		year: number
		onclick?: () => void
	}

	let { code, name, year, onclick }: Props = $props()
</script>

<button
	{onclick}
	class="w-full rounded-lg border border-gray-200 bg-white p-4 text-left transition-all hover:border-blue-500 hover:shadow-md"
>
	<div class="flex items-start justify-between">
		<div class="flex-1">
			<div class="mb-1 flex items-center gap-2">
				<span
					class="rounded bg-blue-100 px-2 py-1 text-xs font-semibold text-blue-800"
				>
					{code}
				</span>
				<span class="text-xs text-gray-500">{year}</span>
			</div>
			<h3 class="font-medium text-gray-900">{name}</h3>
		</div>
		<svg
			class="h-5 w-5 text-gray-400"
			fill="none"
			viewBox="0 0 24 24"
			stroke="currentColor"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M9 5l7 7-7 7"
			/>
		</svg>
	</div>
</button>
