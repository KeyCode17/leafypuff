import { queryOptions, useQuery } from "@tanstack/react-query"

import { getRoles } from "#/routes/_authenticated.roles/_apis/roles-api"

const STALE_TIME_MS = 60_000

export const ROLES_QUERY_KEY = {
	LIST: ["get-roles"],
} as const

export const rolesQueryOptions = queryOptions({
	queryKey: ROLES_QUERY_KEY.LIST,
	queryFn: getRoles,
	staleTime: STALE_TIME_MS,
})

export const useGetRoles = () => useQuery(rolesQueryOptions)
