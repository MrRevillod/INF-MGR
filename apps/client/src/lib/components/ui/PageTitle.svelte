<script lang="ts">
	import { appStore } from "$lib/shared/app.store.svelte"

	interface PageTitleProps {
		title?: string
		description?: string
	}

	let { title, description }: PageTitleProps = $props()
</script>

<header
	class="border-border flex flex-row items-center justify-between border-b pb-6"
>
	<div>
		<h1 class="text-text-primary text-2xl font-semibold">
			{title ?? appStore.pageTitle}
		</h1>
		<p class="text-text-muted mt-2 text-sm">
			{description ?? "Bienvenido al sistema de gestión académica"}
		</p>
	</div>
</header>
