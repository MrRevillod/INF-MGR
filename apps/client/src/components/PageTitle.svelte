<script>
	import { appStore } from "$lib/stores/app.svelte"

	let { description } = $props()
</script>

<div class="border-border flex flex-row items-center justify-between border-b pb-6">
	<div>
		<h1 class="text-text-primary text-2xl font-semibold">{appStore.pageTitle}</h1>
		<p class="text-text-muted mt-2 text-sm">
			{description ?? "Bienvenido al sistema de gestión académica"}
		</p>
	</div>
</div>
