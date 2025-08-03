<script lang="ts">
	let { children } = $props()
</script>

<section
	class="bg-background ml-16 min-h-screen transition-all duration-300 lg:ml-64"
>
	<div class="px-4 py-8 lg:p-8">
		{@render children()}
	</div>
</section>
