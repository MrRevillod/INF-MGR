<script lang="ts">
	import "../app.css"

	import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query"

	let { children } = $props()

	const queryClient = new QueryClient()
</script>

<QueryClientProvider client={queryClient}>
	<main class="bg-background max-h-screen min-h-screen">
		{@render children()}
	</main>
</QueryClientProvider>
