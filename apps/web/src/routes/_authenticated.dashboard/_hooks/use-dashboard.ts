import { queryOptions, useSuspenseQuery } from "@tanstack/react-query"

import {
	type TServiceOverview,
	getServiceOverview,
} from "#/routes/_authenticated.dashboard/_apis/dashboard-api"

const STALE_TIME_MS = 60_000

export const DASHBOARD_QUERY_KEY = {
	OVERVIEW: ["get-service-overview"],
} as const

export const overviewQueryOptions = queryOptions({
	queryKey: DASHBOARD_QUERY_KEY.OVERVIEW,
	queryFn: getServiceOverview,
	staleTime: STALE_TIME_MS,
})

export const useDashboard = (): { overview: TServiceOverview } => {
	const { data } = useSuspenseQuery(overviewQueryOptions)
	return { overview: data }
}
