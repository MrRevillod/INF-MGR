<script lang="ts">
	import Navbar from "$lib/components/ui/Navbar.svelte"

	let { children } = $props()
</script>

<Navbar />

<section
	class="bg-background ml-16 min-h-screen transition-all duration-300 lg:ml-64"
>
	<div class="px-4 py-8 lg:p-8">
		{@render children()}
	</div>
</section>
