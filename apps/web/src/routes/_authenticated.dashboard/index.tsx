import { createFileRoute } from "@tanstack/react-router"
import type { ReactElement } from "react"

import { Guard } from "#/components/guard"
import { SectionBoundary } from "#/components/section-boundary"
import { Typography } from "#/components/typography"
import { PERMISSION } from "#/constants/permission"
import { OverviewFigures } from "#/routes/_authenticated.dashboard/_components/overview-figures"
import { overviewQueryOptions } from "#/routes/_authenticated.dashboard/_hooks/use-dashboard"

const DashboardScreen = (): ReactElement => (
	<main aria-labelledby="dashboard-heading" className="flex flex-col gap-6 p-8">
		<Typography variant="title" id="dashboard-heading">
			Dashboard
		</Typography>
		<Typography variant="paragraph">
			Every figure is a count or a byte total. None of them is derived from
			anything the server can read.
		</Typography>
		<Guard permissions={[PERMISSION.ENTRY_COUNT_READ]}>
			<SectionBoundary>
				<OverviewFigures />
			</SectionBoundary>
		</Guard>
	</main>
)

export const Route = createFileRoute("/_authenticated/dashboard/")({
	loader: ({ context }) =>
		context.queryClient.ensureQueryData(overviewQueryOptions),
	component: DashboardScreen,
})
