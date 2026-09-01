import type { ColumnDef } from "@tanstack/react-table"
import { useStore } from "@tanstack/react-store"
import type { ReactElement } from "react"
import { match } from "ts-pattern"

import { DataTable } from "#/components/data-table"
import { Typography } from "#/components/typography"
import {
	applyPagination,
	applySorting,
	createTableStore,
} from "#/lib/table-state-store"
import type { TCampaign } from "#/routes/_authenticated.release/_apis/release-api"
import { useGetCampaigns } from "#/routes/_authenticated.release/_hooks/use-release"

const tableStore = createTableStore()

const formatDay = (millis: number): string =>
	new Date(millis).toISOString().slice(0, 10)

const columns: ColumnDef<TCampaign>[] = [
	{
		id: "title",
		header: "Title",
		cell: ({ row }) => (
			<Typography variant="paragraph">{row.original.title}</Typography>
		),
	},
	{
		id: "platform",
		header: "Platform",
		cell: ({ row }) => (
			<Typography variant="caption">{row.original.platform}</Typography>
		),
	},
	{
		id: "window",
		header: "Window",
		cell: ({ row }) => (
			<Typography variant="caption">
				{`${formatDay(row.original.starts_at_ms)} to ${formatDay(row.original.ends_at_ms)}`}
			</Typography>
		),
	},
	{
		id: "published",
		header: "State",
		cell: ({ row }) => (
			<Typography variant="caption">
				{match(row.original.published)
					.with(true, () => "Published")
					.otherwise(() => "Draft")}
			</Typography>
		),
	},
]

export const CampaignsTable = (): ReactElement => {
	const campaigns = useGetCampaigns()
	const state = useStore(tableStore, (held) => held)
	const rows = campaigns.data ?? []

	return (
		<DataTable
			columns={columns}
			rows={rows}
			rowCount={rows.length}
			sorting={state.sorting}
			onSortingChange={(next) => applySorting(tableStore, next)}
			pagination={state.pagination}
			onPaginationChange={(next) => applyPagination(tableStore, next)}
			emptyMessage="Nothing is scheduled"
		/>
	)
}
