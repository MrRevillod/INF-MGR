import axios from "axios"
import { authStore } from "../stores/auth.store.svelte"
import { goto } from "$app/navigation"

const axiosOpts = {
	baseURL: "/api",
	includeCredentials: false,
}

// Axios instance for unprotected (public) server routes
export const api = axios.create({ ...axiosOpts })

export const protectedApi = axios.create({ ...axiosOpts })

protectedApi.interceptors.request.use(
	config => {
		if (typeof window !== "undefined") {
			const access = localStorage.getItem("ACCESS")
			const refresh = localStorage.getItem("REFRESH")

			if (access && refresh) {
				if (!config.headers) {
					config.headers = {}
				}
				config.headers["Authorization"] = `Bearer ${access},${refresh}`
			} else {
				return Promise.reject(new Error("No authentication tokens found"))
			}
		}
		return config
	},
	error => Promise.reject(error)
)

// Axios instance for protected server routes
// Includes a response interceptor to automatically refresh the session when it expires
// export const protectedApi = axios.create({ ...axiosOpts })

protectedApi.interceptors.response.use(
	async response => response,
	async error => {
		// Si el error es del request interceptor (no hay tokens)
		if (error.message === "No authentication tokens found") {
			authStore.reset()
			goto("/auth/login")
			return Promise.reject(error)
		}

		// Si el error no es 401, rechazar
		if (error?.response?.status !== 401) {
			return Promise.reject(error)
		}

		const originalRequest = error.config

		// ✅ Prevenir loop infinito de refresh
		if (originalRequest._retry) {
			authStore.reset()
			localStorage.removeItem("ACCESS")
			localStorage.removeItem("REFRESH")
			goto("/auth/login")
			return Promise.reject(error)
		}

		originalRequest._retry = true

		try {
			const response = await api.post(
				"/auth/refresh",
				{},
				{
					headers: {
						Authorization: originalRequest.headers["Authorization"],
					},
				}
			)

			if (response.status === 200) {
				const { access_token, refresh_token } = response.data.data

				if (!access_token || !refresh_token) {
					throw new Error("Invalid refresh response")
				}

				localStorage.setItem("ACCESS", access_token)
				localStorage.setItem("REFRESH", refresh_token)

				// Actualizar el header con los nuevos tokens
				originalRequest.headers["Authorization"] =
					`Bearer ${access_token},${refresh_token}`

				return protectedApi(originalRequest)
			}
		} catch (refreshError) {
			console.error("Refresh failed:", refreshError)

			authStore.reset()
			localStorage.removeItem("ACCESS")
			localStorage.removeItem("REFRESH")
			goto("/auth/login")

			return Promise.reject(refreshError)
		}
	}
)
