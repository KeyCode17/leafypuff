export type TErrorBody = {
	code: string
	detail: string
}

export type TEnvelope<TData> = {
	success: boolean
	data: TData | null
	message: string
	error: TErrorBody | null
}

export class ApiError extends Error {
	readonly code: string
	readonly status: number

	constructor(status: number, code: string, detail: string) {
		super(detail)
		this.name = "ApiError"
		this.code = code
		this.status = status
	}
}

const isObject = (value: unknown): value is Record<string, unknown> =>
	typeof value === "object" && value !== null

const readError = (value: unknown): TErrorBody | null => {
	if (!isObject(value)) {
		return null
	}
	const code = value.code
	const detail = value.detail
	if (typeof code !== "string" || typeof detail !== "string") {
		return null
	}
	return { code, detail }
}

/**
 * The API answers {success, data, message, error} on every route, including failures. A body that
 * does not have that shape came from something other than the API — a proxy error page, most
 * likely — and is reported as such rather than being read as a successful empty response.
 */
export const readEnvelope = <TData>(
	status: number,
	body: unknown,
): TEnvelope<TData> => {
	if (!isObject(body) || typeof body.success !== "boolean") {
		throw new ApiError(
			status,
			"MALFORMED_RESPONSE",
			"The response is not an api envelope",
		)
	}
	return {
		success: body.success,
		data: body.data === undefined ? null : (body.data as TData),
		message: typeof body.message === "string" ? body.message : "",
		error: readError(body.error),
	}
}
