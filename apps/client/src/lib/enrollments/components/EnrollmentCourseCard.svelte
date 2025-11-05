<script lang="ts">
	import { getCourseQuery } from "$lib/enrollments/querys"

	interface Props {
		courseId: string
		onclick: (courseId: string) => void
	}

	const { courseId, onclick }: Props = $props()

	const courseQuery = getCourseQuery(courseId)
	const { data: courseRes, isLoading } = $derived(courseQuery)
</script>

{#if isLoading}
	<div class="animate-pulse rounded-lg border border-gray-200 bg-gray-50 p-4">
		<div class="h-4 w-3/4 rounded bg-gray-300"></div>
		<div class="mt-2 h-3 w-1/2 rounded bg-gray-200"></div>
	</div>
{:else if courseRes?.data}
	<button
		onclick={() => onclick(courseId)}
		class="w-full rounded-lg border border-gray-200 bg-white p-4 text-left transition-all hover:border-blue-300 hover:shadow-md"
	>
		<h3 class="font-semibold text-gray-900">{courseRes.data.name}</h3>
		{#if courseRes.data.description}
			<p class="mt-1 text-sm text-gray-600">{courseRes.data.description}</p>
		{/if}
		<div class="mt-2 text-xs text-gray-500">
			Código: {courseRes.data.code}
		</div>
	</button>
{:else}
	<div class="rounded-lg border border-red-200 bg-red-50 p-4 text-sm text-red-600">
		Error al cargar el curso
	</div>
{/if}
