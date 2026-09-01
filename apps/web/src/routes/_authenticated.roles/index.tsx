import { createFileRoute } from "@tanstack/react-router"
import type { ReactElement } from "react"

import { Guard } from "#/components/guard"
import { SectionBoundary } from "#/components/section-boundary"
import { Typography } from "#/components/typography"
import { PERMISSION } from "#/constants/permission"
import { RolesTable } from "#/routes/_authenticated.roles/_components/roles-table"

const RolesScreen = (): ReactElement => (
	<main className="flex flex-col gap-6 p-8">
		<Typography variant="title">Roles</Typography>
		<Guard permissions={[PERMISSION.ROLE_READ]}>
			<SectionBoundary>
				<RolesTable />
			</SectionBoundary>
		</Guard>
	</main>
)

export const Route = createFileRoute("/_authenticated/roles/")({
	component: RolesScreen,
})
