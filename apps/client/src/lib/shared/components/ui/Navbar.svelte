<script lang="ts">
	import cx from "clsx"
	import type { Component } from "svelte"

	import {
		HomeIcon,
		UserGroupIcon,
		ComputerDesktopIcon,
		ArrowLeftStartOnRectangleIcon,
		UserCircleIcon,
	} from "@fvilers/heroicons-svelte/20/solid"

	import { appStore } from "$lib/shared/stores/app.store.svelte"
	import { useLogoutMutation } from "$lib/auth/queries"
	import { afterNavigate, goto } from "$app/navigation"
	import { auth } from "$lib/auth/store.svelte"

	// Determinar el prefijo de ruta según el rol del usuario
	const userRole = $derived(auth.user?.role ?? "student")

	// Admin y Secretary tienen acceso completo al dashboard
	const isAdminOrSecretary = $derived(
		userRole === "administrator" || userRole === "secretary"
	)

	const rolePrefix = $derived(
		userRole === "administrator"
			? "/admin"
			: userRole === "secretary"
				? "/secretary"
				: ""
	)

	// Rutas según el rol
	const routes = $derived(
		isAdminOrSecretary
			? [
					{ title: "Inicio", path: rolePrefix, icon: HomeIcon },
					{ title: "Usuarios", path: `${rolePrefix}/users`, icon: UserGroupIcon },
					{
						title: "Cursos",
						path: `${rolePrefix}/courses`,
						icon: ComputerDesktopIcon,
					},
				]
			: [
					// Estudiantes y profesores verán otras rutas (por implementar)
					{ title: "Inicio", path: "/", icon: HomeIcon },
				]
	)

	const bottomActions = [
		{ title: "Mi perfil", path: "/profile", icon: UserCircleIcon },
	]

	const logoutMutation = useLogoutMutation()

	const handleLogout = () => {
		logoutMutation.mutate(undefined, {})

		appStore.clear()
		auth.logout()
	}

	afterNavigate(navigate => {
		const route = navigate.to?.url.pathname ?? ""

		// Normalizar rutas para comparación (quitar trailing slash)
		const normalizedRoute =
			route.endsWith("/") && route.length > 1 ? route.slice(0, -1) : route

		routes.forEach(({ path, title }) => {
			const normalizedPath =
				path.endsWith("/") && path.length > 1 ? path.slice(0, -1) : path

			if (normalizedPath === normalizedRoute) {
				appStore.setRoute(normalizedPath)
				appStore.setTitle(title)
			}
		})
	})
</script>

<nav
	class="bg-surface border-border fixed left-0 top-0 flex h-full w-16 flex-col border-r transition-all duration-300 lg:w-64"
>
	<div class="border-border border-b p-4 lg:px-6 lg:py-10">
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
			{#each routes as route (route.path)}
				{@render navItem(route)}
			{/each}
		</nav>
	</div>

	<div
		class="flex flex-col items-center justify-center p-4 lg:items-start lg:justify-start"
	>
		{#each bottomActions as action (action.title)}
			{@render navItem(action)}
		{/each}

		<button
			onclick={() => handleLogout()}
			title="Cerrar sesión"
			class={"text-text-secondary hover:text-text-primary hover:bg-hover-bg flex items-center justify-center rounded-lg px-2 py-3 text-base font-medium transition-all duration-200 lg:justify-start lg:gap-3 lg:px-3"}
		>
			<ArrowLeftStartOnRectangleIcon class="h-6 w-6 flex-shrink-0" />
			<span class="hidden lg:block">Cerrar sesión</span>
		</button>
	</div>

	<div class="border-border flex items-center justify-between border-t p-2 lg:p-4">
		<div class="text-text-muted text-center text-xs lg:text-left lg:text-sm">
			<span class="hidden lg:block">INF-MGR v1.0</span>
			<span class="lg:hidden">v1.0</span>
		</div>
	</div>
</nav>

{#snippet navItem(route: { title: string; path: string; icon: Component })}
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
{/snippet}
