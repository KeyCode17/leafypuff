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
import type { TCatalogBundle } from "#/routes/_authenticated.catalog/_apis/catalog-api"
import { PublishAction } from "#/routes/_authenticated.catalog/_components/publish-action"
import { useGetCatalogBundles } from "#/routes/_authenticated.catalog/_hooks/use-get-catalog-bundles"

const tableStore = createTableStore()

const formatInstant = (millis: number): string =>
	new Date(millis).toISOString().replace("T", " ").slice(0, 19)

const columns: ColumnDef<TCatalogBundle>[] = [
	{
		id: "version",
		header: "Version",
		cell: ({ row }) => (
			<Typography variant="paragraph">
				{String(row.original.version)}
			</Typography>
		),
	},
	{
		id: "created_at_ms",
		header: "Drafted",
		cell: ({ row }) => (
			<Typography variant="caption">
				{formatInstant(row.original.created_at_ms)}
			</Typography>
		),
	},
	{
		id: "published_at_ms",
		header: "Published",
		cell: ({ row }) => (
			<Typography variant="caption">
				{row.original.published_at_ms === null
					? undefined
					: formatInstant(row.original.published_at_ms)}
			</Typography>
		),
	},
	{
		id: "actions",
		header: "",
		cell: ({ row }) => <PublishAction bundle={row.original} />,
	},
]

export const CatalogTable = (): ReactElement => {
	const bundles = useGetCatalogBundles()
	const state = useStore(tableStore, (held) => held)
	const rows = bundles.data ?? []

	return (
		<DataTable
			columns={columns}
			rows={rows}
			rowCount={rows.length}
			sorting={state.sorting}
			onSortingChange={(next) => applySorting(tableStore, next)}
			pagination={state.pagination}
			onPaginationChange={(next) => applyPagination(tableStore, next)}
			emptyMessage="No bundle has been drafted yet"
		/>
	)
}
