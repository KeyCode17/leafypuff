import { useSuspenseQuery } from "@tanstack/react-query"

import {
	type TDashboardSummary,
	dashboardSummaryQueryOptions,
} from "#/routes/_authenticated.dashboard/_apis/dashboard-api"

export type TUseDashboardReturn = {
	summary: TDashboardSummary
}

export const useDashboard = (): TUseDashboardReturn => {
	const { data } = useSuspenseQuery(dashboardSummaryQueryOptions())
	return { summary: data }
}
