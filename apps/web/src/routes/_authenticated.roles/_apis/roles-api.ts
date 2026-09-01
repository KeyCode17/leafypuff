import { request } from "#/lib/api-client"

export type TRole = {
	id: string
	name: string
	description: string | null
	permissions: string[]
}

export type TGranted = {
	permissions: string[]
}

export const getRoles = (): Promise<TRole[]> =>
	request<TRole[]>({
		path: "/v1/admin/roles",
		method: "GET",
		authenticated: true,
	})

export const getGrantedPermissions = (): Promise<TGranted> =>
	request<TGranted>({
		path: "/v1/admin/permissions",
		method: "GET",
		authenticated: true,
	})

export const postAssignRole = (payload: {
	account_id: string
	role_id: string
}): Promise<void> =>
	request<null>({
		path: "/v1/admin/roles/assign",
		method: "POST",
		body: payload,
		authenticated: true,
	}).then(() => undefined)
