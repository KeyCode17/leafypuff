import { queryOptions, useQuery } from "@tanstack/react-query"

import { getAuditEvents } from "#/routes/_authenticated.audit/_apis/audit-api"

const STALE_TIME_MS = 15_000

export const AUDIT_QUERY_KEY = {
	LIST: ["get-audit-events"],
} as const

export const auditQueryOptions = queryOptions({
	queryKey: AUDIT_QUERY_KEY.LIST,
	queryFn: getAuditEvents,
	staleTime: STALE_TIME_MS,
})

export const useGetAuditEvents = () => useQuery(auditQueryOptions)
