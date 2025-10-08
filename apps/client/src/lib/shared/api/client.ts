import axios from "axios"

const axiosOpts = {
	baseURL: "/api",
	includeCredentials: false,
}

// Axios instance for unprotected (public) server routes
export const api = axios.create({ ...axiosOpts })

export const protectedApi = axios.create({ ...axiosOpts })

protectedApi.interceptors.request.use(config => {
	if (typeof window !== "undefined") {
		const access = localStorage.getItem("ACCESS")
		const refresh = localStorage.getItem("REFRESH")

		if (access && refresh) {
			if (!config.headers) {
				config.headers = {}
			}

			config.headers["Authorization"] = `Bearer ${access},${refresh}`
		}
	}

	return config
})

// Axios instance for protected server routes
// Includes a response interceptor to automatically refresh the session when it expires
// export const protectedApi = axios.create({ ...axiosOpts })

protectedApi.interceptors.response.use(
	async response => response,
	async error => {
		// If the error is not a 401 (unauthorized), reject the promise
		if (error?.response?.status !== 401) {
			return Promise.reject(error)
		}

		// If the error is 401, the session maybe have expired
		const originalRequest = error.config

		// So We can try to refresh the session making a req to "/auth/refresh"
		// if the refresh req is successfull, retry the original request
		return api
			.post(
				"/auth/refresh",
				{},
				{
					headers: { Authorization: originalRequest.headers["Authorization"] },
				}
			)
			.then(response => {
				if (response.status === 200) {
					console.log("Refresh response:", response.data)

					const { access_token, refresh_token } = response.data.data

					if (!access_token || !refresh_token) {
						return Promise.reject(new Error("Invalid refresh response"))
					}

					localStorage.setItem("ACCESS", access_token)
					localStorage.setItem("REFRESH", refresh_token)

					delete originalRequest.headers["Authorization"]
					return protectedApi(originalRequest)
				}
			})
			.catch(error => {
				console.error("Refresh error:", error)
				return Promise.reject(error)
			})
	}
)
