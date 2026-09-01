import { request } from "#/lib/api-client"

export type TAuditEvent = {
	id: string
	actor_id: string
	action: string
	subject_id: string | null
	detail: string
	recorded_at_ms: number
}

export const getAuditEvents = (): Promise<TAuditEvent[]> =>
	request<TAuditEvent[]>({
		path: "/v1/admin/audit",
		method: "GET",
		authenticated: true,
	})
