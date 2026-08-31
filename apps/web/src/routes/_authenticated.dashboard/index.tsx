import { createFileRoute } from "@tanstack/react-router"
import type { ReactElement } from "react"

import { Guard } from "#/components/guard"
import { Typography } from "#/components/typography"
import { PERMISSION } from "#/constants/permission"
import { dashboardSummaryQueryOptions } from "#/routes/_authenticated.dashboard/_apis/dashboard-api"
import { useDashboard } from "#/routes/_authenticated.dashboard/_hooks/use-dashboard"

const DashboardPage = (): ReactElement => {
	const { summary } = useDashboard()

	return (
		<section
			aria-labelledby="dashboard-heading"
			className="flex flex-col gap-6 p-6"
		>
			<Typography variant="title" id="dashboard-heading">
				Dashboard
			</Typography>
			<Guard permissions={[PERMISSION.ENTRY_COUNT_READ]}>
				<dl className="flex flex-col gap-2">
					<Typography variant="label">Accounts</Typography>
					<Typography variant="paragraph">{summary.accounts}</Typography>
					<Typography variant="label">Entries</Typography>
					<Typography variant="paragraph">{summary.entries}</Typography>
				</dl>
			</Guard>
		</section>
	)
}

export const Route = createFileRoute("/_authenticated/dashboard/")({
	loader: ({ context }) =>
		context.queryClient.ensureQueryData(dashboardSummaryQueryOptions()),
	component: DashboardPage,
})
