import { createFileRoute } from "@tanstack/react-router"
import type { ReactElement } from "react"

import { Guard } from "#/components/guard"
import { SectionBoundary } from "#/components/section-boundary"
import { Typography } from "#/components/typography"
import { PERMISSION } from "#/constants/permission"
import { CampaignsTable } from "#/routes/_authenticated.release/_components/campaigns-table"
import { GatesTable } from "#/routes/_authenticated.release/_components/gates-table"

const ReleaseScreen = (): ReactElement => (
	<main className="flex flex-col gap-8 p-8">
		<Typography variant="title">Release</Typography>
		<Typography variant="paragraph">
			Forcing an update stops the sync exchange and nothing else. A blocked
			build still reads and writes its own diary.
		</Typography>
		<Guard permissions={[PERMISSION.RELEASE_READ]}>
			<SectionBoundary>
				<GatesTable />
			</SectionBoundary>
			<Typography variant="section">Campaigns</Typography>
			<SectionBoundary>
				<CampaignsTable />
			</SectionBoundary>
		</Guard>
	</main>
)

export const Route = createFileRoute("/_authenticated/release/")({
	component: ReleaseScreen,
})
