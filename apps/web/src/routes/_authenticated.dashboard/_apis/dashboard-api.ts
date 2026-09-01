import { request } from "#/lib/api-client"

export type TServiceOverview = {
	account_count: number
	verified_account_count: number
	suspended_account_count: number
	entry_count: number
	tombstoned_entry_count: number
	device_count: number
	devices_synced_last_day: number
	field_conflict_count: number
	media_object_count: number
	media_bytes: number
}

export const getServiceOverview = (): Promise<TServiceOverview> =>
	request<TServiceOverview>({
		path: "/v1/admin/overview",
		method: "GET",
		authenticated: true,
	})
