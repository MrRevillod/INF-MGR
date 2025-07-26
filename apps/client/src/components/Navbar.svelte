<script lang="ts">
	import {
		HomeIcon,
		BookOpenIcon,
		PencilIcon,
		ComputerDesktopIcon,
	} from "@fvilers/heroicons-svelte/20/solid"

	import { appStore } from "$lib/stores/app.svelte"
	import { afterNavigate } from "$app/navigation"

	import cx from "clsx"

	const routes = [
		{ title: "Inicio", path: "/", icon: HomeIcon },
		{ title: "Estudiantes", path: "/users/students", icon: PencilIcon },
		{ title: "Personal Académico", path: "/users/staff", icon: BookOpenIcon },
		{ title: "Asignaturas", path: "/asignatures", icon: ComputerDesktopIcon },
	]

	afterNavigate(navigate => {
		const route = navigate.to?.url.pathname ?? ""

		routes.forEach(({ path, title }) => {
			if (path === route) {
				appStore.setRoute(path)
				appStore.setTitle(title)
			}
		})
	})
</script>

<nav
	class="bg-surface border-border fixed left-0 top-0 flex h-full w-16 flex-col border-r transition-all duration-300 lg:w-64"
>
	<div class="border-border border-b p-4 lg:p-6">
		<div class="hidden lg:block">
			<h1 class="text-text-primary text-2xl font-semibold">Dashboard</h1>
			<p class="text-text-muted mt-1 text-base">Sistema de Prácticas y Tesis</p>
		</div>
		<div class="text-center lg:hidden">
			<div class="text-text-primary text-xl font-bold">D</div>
		</div>
	</div>

	<div class="flex-1 px-2 py-6 lg:px-4">
		<nav class="space-y-2">
			{#each routes as route}
				<a
					href={route.path}
					title={route.title}
					class={cx(
						"flex items-center rounded-lg px-2 py-3 text-base font-medium transition-all duration-200 lg:px-3",
						"justify-center lg:justify-start lg:gap-3",
						{
							"bg-accent text-white": appStore.currentRoute === route.path,
							"text-text-secondary hover:text-text-primary hover:bg-hover-bg":
								appStore.currentRoute !== route.path,
						}
					)}
				>
					<route.icon class="h-6 w-6 flex-shrink-0" />
					<span class="hidden lg:block">{route.title}</span>
				</a>
			{/each}
		</nav>
	</div>

	<div class="border-border border-t p-2 lg:p-4">
		<div class="text-text-muted text-center text-xs lg:text-left lg:text-sm">
			<span class="hidden lg:block">INF-MGR v1.0</span>
			<span class="lg:hidden">v1.0</span>
		</div>
	</div>
</nav>
