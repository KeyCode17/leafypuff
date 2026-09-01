import { createFileRoute } from "@tanstack/react-router"
import type { ReactElement } from "react"

import { Guard } from "#/components/guard"
import { SectionBoundary } from "#/components/section-boundary"
import { Typography } from "#/components/typography"
import { PERMISSION } from "#/constants/permission"
import { AccountsTable } from "#/routes/_authenticated.accounts/_components/accounts-table"

const AccountsScreen = (): ReactElement => (
	<main className="flex flex-col gap-6 p-8">
		<Typography variant="title">Accounts</Typography>
		<Typography variant="paragraph">
			Counts, dates and storage. Nothing anyone wrote is readable here, or
			anywhere else on the server.
		</Typography>
		<Guard permissions={[PERMISSION.ACCOUNT_LIST]}>
			<SectionBoundary>
				<AccountsTable />
			</SectionBoundary>
		</Guard>
	</main>
)

export const Route = createFileRoute("/_authenticated/accounts/")({
	component: AccountsScreen,
})
