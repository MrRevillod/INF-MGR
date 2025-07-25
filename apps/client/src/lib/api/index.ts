import axios from "axios"

const axiosOpts = {
	baseURL: "/api",
	withCredentials: true, // Sends cookies automatically
}

// Axios instance for unprotected (public) server routes
export const api = axios.create({ ...axiosOpts })

// Axios instance for protected server routes
// Includes a response interceptor to automatically refresh the session when it expires
// export const protectedApi = axios.create({ ...axiosOpts })

// protectedApi.interceptors.response.use(
// 	async response => response,
// 	async error => {
// 		// If the error is not a 401 (unauthorized), reject the promise
// 		if (error?.response?.status !== 401) {
// 			return Promise.reject(error)
// 		}

// 		// If the error is 401, the session maybe have expired
// 		const originalRequest = error.config

// 		// So We can try to refresh the session making a req to "/auth/refresh"
// 		// if the refresh req is successfull, retry the original request
// 		return api
// 			.post("/auth/refresh")
// 			.then(response => {
// 				if (response.status === 200) return protectedApi(originalRequest)
// 			})
// 			.catch(error => Promise.reject(error))
// 	}
// )
