export const useEncodeData = (data: Record<string, any>) => {
	const encodedData = encodeURIComponent(JSON.stringify(data))
	return `data=${encodedData}`
}

export const useDecodeData = (data: string) => {
	try {
		return JSON.parse(decodeURIComponent(data))
	} catch (error) {
		console.error("Failed to decode data:", error)
		return null
	}
}
