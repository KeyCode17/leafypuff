import { createFileRoute } from "@tanstack/react-router"
import type { ReactElement } from "react"

import { Guard } from "#/components/guard"
import { SectionBoundary } from "#/components/section-boundary"
import { Typography } from "#/components/typography"
import { PERMISSION } from "#/constants/permission"
import { RequestsTable } from "#/routes/_authenticated.privacy/_components/requests-table"

const PrivacyScreen = (): ReactElement => (
	<main className="flex flex-col gap-6 p-8">
		<Typography variant="title">Privacy</Typography>
		<Typography variant="paragraph">
			Fulfilling an erasure deletes the account and everything keyed to it, and
			nulls the identity the audit log points at. No audit row is ever updated
			or deleted, so the record of having honoured the request outlives the
			person.
		</Typography>
		<Guard permissions={[PERMISSION.DATA_REQUEST_READ]}>
			<SectionBoundary>
				<RequestsTable />
			</SectionBoundary>
		</Guard>
	</main>
)

export const Route = createFileRoute("/_authenticated/privacy/")({
	component: PrivacyScreen,
})
