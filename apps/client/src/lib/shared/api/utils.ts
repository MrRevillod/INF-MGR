import type { ApiResponse } from "$api/types"
import type { AxiosResponse } from "axios"

export const UknownError = {
	data: null,
	status: 500,
	timestamp: new Date().toISOString(),
	message: "Error desconocido, por favor intente más tarde.",
}

interface TryHttpParams {
	args?: any
	fn: (args: any) => Promise<AxiosResponse<any>>
}

export const tryHttp = async <T>(props: TryHttpParams): Promise<ApiResponse<T>> => {
	try {
		const response = await props.fn(props.args)
		return response.data as ApiResponse<T>
	} catch (error: any) {
		throw new Error(error?.response?.data ?? UknownError)
	}
}
