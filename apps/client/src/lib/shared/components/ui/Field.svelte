<script lang="ts">
	interface FieldProps {
		label: string
		value?: string | (() => string)
	}

	let { label, value }: FieldProps = $props()
</script>

<div class="border-border flex w-full flex-col border-b pb-2">
	<div class="flex w-full flex-row justify-between">
		<div class="flex-1 pr-4">
			<strong class="text-text-primary">{label}:</strong>
			<span class="text-text-secondary ml-2">
				{value instanceof Function ? value() : value}
			</span>
		</div>
	</div>
</div>
