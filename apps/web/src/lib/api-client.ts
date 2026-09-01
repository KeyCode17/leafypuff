import { env } from "#/lib/env"
import { ApiError, readEnvelope } from "#/lib/envelope"
import { sessionStore } from "#/lib/session-store"

type TRequest = {
	path: string
	method: "GET" | "POST" | "PUT" | "DELETE"
	body?: unknown
	authenticated?: boolean
}

const JSON_CONTENT_TYPE = "application/json"

const headers = (request: TRequest): HeadersInit => {
	const built: Record<string, string> = { "content-type": JSON_CONTENT_TYPE }
	const token = sessionStore.state.accessToken
	if (request.authenticated && token) {
		built.authorization = `Bearer ${token}`
	}
	built["x-device-id"] = sessionStore.state.deviceId
	return built
}

export const request = async <TData>(call: TRequest): Promise<TData> => {
	const response = await fetch(`${env.apiBaseUrl}${call.path}`, {
		method: call.method,
		headers: headers(call),
		body: call.body === undefined ? undefined : JSON.stringify(call.body),
	})

	const parsed = await response.json().catch(() => null)
	const envelope = readEnvelope<TData>(response.status, parsed)

	if (!envelope.success || envelope.data === null) {
		throw new ApiError(
			response.status,
			envelope.error?.code ?? "UNKNOWN",
			envelope.error?.detail ?? envelope.message,
		)
	}
	return envelope.data
}

export const requestWithoutData = async (call: TRequest): Promise<void> => {
	const response = await fetch(`${env.apiBaseUrl}${call.path}`, {
		method: call.method,
		headers: headers(call),
		body: call.body === undefined ? undefined : JSON.stringify(call.body),
	})

	const parsed = await response.json().catch(() => null)
	const envelope = readEnvelope<null>(response.status, parsed)

	if (!envelope.success) {
		throw new ApiError(
			response.status,
			envelope.error?.code ?? "UNKNOWN",
			envelope.error?.detail ?? envelope.message,
		)
	}
}
