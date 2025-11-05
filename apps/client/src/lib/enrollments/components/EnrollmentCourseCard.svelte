<script lang="ts">
	import { getCourseQuery } from "$lib/courses/queries"

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
		<div class="mb-1 flex items-center gap-2">
			<span
				class="rounded bg-blue-100 px-2 py-1 text-xs font-semibold text-blue-800"
			>
				{courseRes.data.code}
			</span>
			<span class="text-xs text-gray-500">{courseRes.data.year}</span>
		</div>
		<h3 class="font-semibold text-gray-900">{courseRes.data.name}</h3>
		<div class="mt-2 flex items-center gap-2">
			<span
				class="inline-flex rounded-full px-2 text-xs font-semibold leading-5 {courseRes
					.data.courseStatus === 'active'
					? 'bg-green-100 text-green-800'
					: 'bg-gray-100 text-gray-800'}"
			>
				{courseRes.data.courseStatus === "active" ? "Activo" : "Completado"}
			</span>
		</div>
	</button>
{:else}
	<div class="rounded-lg border border-red-200 bg-red-50 p-4 text-sm text-red-600">
		Error al cargar el curso
	</div>
{/if}
