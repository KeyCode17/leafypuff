import { queryOptions } from "@tanstack/react-query"

export type TDashboardSummary = {
	accounts: number
	entries: number
	storageBytes: number
}

export const dashboardKeys = {
	all: ["dashboard"] as const,
	summary: () => [...dashboardKeys.all, "summary"] as const,
}

const fetchDashboardSummary = async (): Promise<TDashboardSummary> => {
	const response = await fetch("/v1/admin/dashboard/summary")
	if (!response.ok) {
		throw new Error("Couldn’t load the dashboard — try again")
	}
	const body: { data: TDashboardSummary } = await response.json()
	return body.data
}

export const dashboardSummaryQueryOptions = () =>
	queryOptions({
		queryKey: dashboardKeys.summary(),
		queryFn: fetchDashboardSummary,
		staleTime: 5 * 60_000,
	})
