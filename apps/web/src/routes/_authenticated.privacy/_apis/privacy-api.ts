import { request, requestWithoutData } from "#/lib/api-client"

export type TDataRequest = {
	id: string
	account_id: string
	email: string | null
	kind: string
	status: string
	requested_at_ms: number
	fulfilled_at_ms: number | null
}

export const getOpenDataRequests = (): Promise<TDataRequest[]> =>
	request<TDataRequest[]>({
		path: "/v1/admin/data-requests",
		method: "GET",
		authenticated: true,
	})

export const postFulfilDataRequest = (requestId: string): Promise<void> =>
	requestWithoutData({
		path: `/v1/admin/data-requests/${requestId}/fulfil`,
		method: "POST",
		authenticated: true,
	})
