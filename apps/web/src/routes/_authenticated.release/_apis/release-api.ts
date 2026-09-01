import { request, requestWithoutData } from "#/lib/api-client"

export type TReleaseGate = {
	platform: string
	minimum_build: number
	force_update: boolean
	behind: boolean
	blocked: boolean
	message: string | null
}

export type TCampaign = {
	id: string
	title: string
	body: string
	platform: string
	starts_at_ms: number
	ends_at_ms: number
	published: boolean
}

export const getReleaseGates = (): Promise<TReleaseGate[]> =>
	request<TReleaseGate[]>({
		path: "/v1/admin/release",
		method: "GET",
		authenticated: true,
	})

export const getCampaigns = (): Promise<TCampaign[]> =>
	request<TCampaign[]>({
		path: "/v1/admin/campaigns",
		method: "GET",
		authenticated: true,
	})

export const postReleaseGate = (payload: {
	platform: string
	minimum_build: number
	force_update: boolean
	message: string | null
}): Promise<void> =>
	requestWithoutData({
		path: "/v1/admin/release",
		method: "POST",
		body: payload,
		authenticated: true,
	})
