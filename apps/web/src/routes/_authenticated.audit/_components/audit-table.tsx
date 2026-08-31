import type { ColumnDef } from "@tanstack/react-table"
import { useStore } from "@tanstack/react-store"
import type { ReactElement } from "react"

import { DataTable } from "#/components/data-table"
import { Typography } from "#/components/typography"
import {
	applyPagination,
	applySorting,
	createTableStore,
} from "#/lib/table-state-store"
import type { TAuditEvent } from "#/routes/_authenticated.audit/_apis/audit-api"
import { useGetAuditEvents } from "#/routes/_authenticated.audit/_hooks/use-get-audit-events"

const tableStore = createTableStore()

const formatInstant = (millis: number): string =>
	new Date(millis).toISOString().replace("T", " ").slice(0, 19)

/// Plain ColumnDef rather than createColumnHelper: the helper narrows TValue per column, and a
/// heterogeneous set of accessors then refuses to sit in one ColumnDef<TRow, unknown>[] without
/// a cast. Reading row.original keeps the table honestly typed with no `as`.
const columns: ColumnDef<TAuditEvent>[] = [
	{
		id: "recorded_at_ms",
		header: "When",
		cell: ({ row }) => (
			<Typography variant="caption">
				{formatInstant(row.original.recorded_at_ms)}
			</Typography>
		),
	},
	{
		id: "action",
		header: "Action",
		cell: ({ row }) => (
			<Typography variant="paragraph">{row.original.action}</Typography>
		),
	},
	{
		id: "actor_id",
		header: "Actor",
		cell: ({ row }) => (
			<Typography variant="caption">{row.original.actor_id}</Typography>
		),
	},
	{
		id: "detail",
		header: "Detail",
		cell: ({ row }) => (
			<Typography variant="caption">{row.original.detail}</Typography>
		),
	},
]

export const AuditTable = (): ReactElement => {
	const events = useGetAuditEvents()
	const state = useStore(tableStore, (held) => held)
	const rows = events.data ?? []

	return (
		<DataTable
			columns={columns}
			rows={rows}
			rowCount={rows.length}
			sorting={state.sorting}
			onSortingChange={(next) => applySorting(tableStore, next)}
			pagination={state.pagination}
			onPaginationChange={(next) => applyPagination(tableStore, next)}
			emptyMessage="Nothing has been recorded yet"
		/>
	)
}
