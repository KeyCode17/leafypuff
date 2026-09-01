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
import type { TAccountSummary } from "#/routes/_authenticated.accounts/_apis/accounts-api"
import { SuspensionActions } from "#/routes/_authenticated.accounts/_components/suspension-actions"
import { useGetAccounts } from "#/routes/_authenticated.accounts/_hooks/use-get-accounts"

const BYTES_IN_A_MEBIBYTE = 1024 * 1024

const tableStore = createTableStore()

const formatMebibytes = (bytes: number): string =>
	`${(bytes / BYTES_IN_A_MEBIBYTE).toFixed(1)} MiB`

const columns: ColumnDef<TAccountSummary>[] = [
	{
		id: "email",
		header: "Account",
		cell: ({ row }) => (
			<Typography variant="paragraph">{row.original.email}</Typography>
		),
	},
	{
		id: "entry_count",
		header: "Entries",
		cell: ({ row }) => (
			<Typography variant="caption">
				{String(row.original.entry_count)}
			</Typography>
		),
	},
	{
		id: "range",
		header: "Writing since",
		cell: ({ row }) => (
			<Typography variant="caption">
				{row.original.first_entry_date ?? undefined}
			</Typography>
		),
	},
	{
		id: "media_bytes",
		header: "Media",
		cell: ({ row }) => (
			<Typography variant="caption">
				{formatMebibytes(row.original.media_bytes)}
			</Typography>
		),
	},
	{
		id: "actions",
		header: "",
		cell: ({ row }) => <SuspensionActions account={row.original} />,
	},
]

export const AccountsTable = (): ReactElement => {
	const accounts = useGetAccounts()
	const state = useStore(tableStore, (held) => held)
	const rows = accounts.data ?? []

	return (
		<DataTable
			columns={columns}
			rows={rows}
			rowCount={rows.length}
			sorting={state.sorting}
			onSortingChange={(next) => applySorting(tableStore, next)}
			pagination={state.pagination}
			onPaginationChange={(next) => applyPagination(tableStore, next)}
			emptyMessage="Nobody has registered yet"
		/>
	)
}
