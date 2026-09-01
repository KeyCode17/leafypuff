import { request, requestWithoutData } from "#/lib/api-client"

export type TCatalogBundle = {
	id: string
	version: number
	payload: unknown
	published: boolean
	published_at_ms: number | null
	created_at_ms: number
}

export const getCatalogBundles = (): Promise<TCatalogBundle[]> =>
	request<TCatalogBundle[]>({
		path: "/v1/admin/catalog",
		method: "GET",
		authenticated: true,
	})

export const postPublishBundle = (bundleId: string): Promise<void> =>
	requestWithoutData({
		path: `/v1/admin/catalog/${bundleId}/publish`,
		method: "POST",
		authenticated: true,
	})
