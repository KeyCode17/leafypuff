import { request, requestWithoutData } from "#/lib/api-client"

export type TAccountSummary = {
	account_id: string
	email: string
	verified: boolean
	suspended: boolean
	entry_count: number
	first_entry_date: string | null
	last_entry_date: string | null
	media_object_count: number
	media_bytes: number
}

export const getAccounts = (): Promise<TAccountSummary[]> =>
	request<TAccountSummary[]>({
		path: "/v1/admin/accounts",
		method: "GET",
		authenticated: true,
	})

export const postSuspendAccount = (accountId: string): Promise<void> =>
	requestWithoutData({
		path: `/v1/admin/accounts/${accountId}/suspend`,
		method: "POST",
		authenticated: true,
	})

export const postRestoreAccount = (accountId: string): Promise<void> =>
	requestWithoutData({
		path: `/v1/admin/accounts/${accountId}/restore`,
		method: "POST",
		authenticated: true,
	})
