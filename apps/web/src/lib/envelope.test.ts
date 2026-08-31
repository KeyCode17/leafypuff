import { describe, expect, it } from "vitest"

import { ApiError, readEnvelope } from "#/lib/envelope"

describe("readEnvelope", () => {
	it("reads a success body", () => {
		const envelope = readEnvelope<{ id: string }>(200, {
			success: true,
			data: { id: "abc" },
			message: "ok",
			error: null,
		})

		expect(envelope.success).toBe(true)
		expect(envelope.data).toEqual({ id: "abc" })
		expect(envelope.error).toBeNull()
	})

	it("reads a failure body and keeps the code", () => {
		const envelope = readEnvelope<null>(401, {
			success: false,
			data: null,
			message: "Request failed",
			error: { code: "INVALID_CREDENTIALS", detail: "Invalid credentials" },
		})

		expect(envelope.success).toBe(false)
		expect(envelope.error?.code).toBe("INVALID_CREDENTIALS")
	})

	it("refuses a body that is not an api envelope", () => {
		expect(() => readEnvelope(502, "<html>bad gateway</html>")).toThrow(
			ApiError,
		)
		expect(() => readEnvelope(200, { id: 1 })).toThrow(ApiError)
		expect(() => readEnvelope(200, null)).toThrow(ApiError)
	})

	it("drops an error object that does not carry a code and a detail", () => {
		const envelope = readEnvelope<null>(500, {
			success: false,
			data: null,
			message: "Request failed",
			error: { code: 7 },
		})

		expect(envelope.error).toBeNull()
	})
})
