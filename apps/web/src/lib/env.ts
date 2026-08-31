const FALLBACK_API_BASE_URL = "https://leafypuff-api.daffakaryudi.web.id"

type TEnv = {
	apiBaseUrl: string
}

const read = (value: unknown): string | undefined =>
	typeof value === "string" && value.length > 0 ? value : undefined

export const env: TEnv = {
	apiBaseUrl: read(import.meta.env.VITE_API_BASE_URL) ?? FALLBACK_API_BASE_URL,
}
