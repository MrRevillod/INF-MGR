import axios from "axios"
import { goto } from "$app/navigation"
import { auth } from "$lib/auth/store.svelte"

const axiosOpts = {
	baseURL: "/api",
	withCredentials: false,
}

// Cliente público (sin autenticación)
export const api = axios.create({ ...axiosOpts })

// Cliente protegido (con Bearer tokens)
export const protectedApi = axios.create({ ...axiosOpts })

/**
 * Request interceptor: Añade tokens a cada petición
 */
protectedApi.interceptors.request.use(
	config => {
		const tokens = auth.getTokens()

		if (tokens) {
			if (!config.headers) config.headers = {}
			config.headers["Authorization"] =
				`Bearer ${tokens.accessToken},${tokens.refreshToken}`
		}

		return config
	},
	error => Promise.reject(error)
)

/**
 * Response interceptor: Maneja refresh automático en 401
 */
protectedApi.interceptors.response.use(
	response => response,
	async error => {
		// Si no es 401, rechazar directamente
		if (error?.response?.status !== 401) {
			return Promise.reject(error)
		}

		const originalReq = error.config

		// Si ya intentamos hacer refresh, logout
		if (originalReq._retry) {
			auth.reset()
			goto("/auth/login")
			return Promise.reject(error)
		}

		originalReq._retry = true

		try {
			const tokens = auth.getTokens()

			if (!tokens) {
				throw new Error("No tokens available")
			}

			// Intentar refrescar la sesión
			const response = await api.post(
				"/auth/refresh",
				{},
				{
					headers: {
						Authorization: `Bearer ${tokens.accessToken},${tokens.refreshToken}`,
					},
				}
			)

			const { access_token, refresh_token } = response.data.data

			if (!access_token || !refresh_token) {
				throw new Error("Invalid refresh response")
			}

			// Guardar nuevos tokens
			auth.setTokens({
				accessToken: access_token,
				refreshToken: refresh_token,
			})

			// Reintentar la petición original con nuevos tokens
			originalReq.headers["Authorization"] =
				`Bearer ${access_token},${refresh_token}`

			return protectedApi(originalReq)
		} catch (refreshError) {
			console.error("Refresh failed:", refreshError)
			auth.reset()
			goto("/auth/login")
			return Promise.reject(refreshError)
		}
	}
)
