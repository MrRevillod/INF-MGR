import { defineConfig } from "vite"
import { svelte } from "@sveltejs/vite-plugin-svelte"

// https://vite.dev/config/
export default defineConfig({
	plugins: [svelte()],
	server: {
		host: "0.0.0.0",
		port: 5173,
		strictPort: true,
		hmr: {
			port: 5173,
		},
	},
})
