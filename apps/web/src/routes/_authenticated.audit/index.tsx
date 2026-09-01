import { createFileRoute } from "@tanstack/react-router"
import type { ReactElement } from "react"

import { Guard } from "#/components/guard"
import { SectionBoundary } from "#/components/section-boundary"
import { Typography } from "#/components/typography"
import { PERMISSION } from "#/constants/permission"
import { AuditTable } from "#/routes/_authenticated.audit/_components/audit-table"

const AuditScreen = (): ReactElement => (
	<main className="flex flex-col gap-6 p-8">
		<Typography variant="title">Audit</Typography>
		<Typography variant="paragraph">
			Every row is appended and never changed. Erasing a person nulls the
			identity it points at and leaves the event itself standing.
		</Typography>
		<Guard permissions={[PERMISSION.AUDIT_READ]}>
			<SectionBoundary>
				<AuditTable />
			</SectionBoundary>
		</Guard>
	</main>
)

export const Route = createFileRoute("/_authenticated/audit/")({
	component: AuditScreen,
})
