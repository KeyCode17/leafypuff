import type { ColumnDef } from "@tanstack/react-table"
import { useStore } from "@tanstack/react-store"
import type { ReactElement } from "react"

import { DataTable } from "#/components/data-table"
import { Guard } from "#/components/guard"
import { Typography } from "#/components/typography"
import { PERMISSION } from "#/constants/permission"
import {
	applyPagination,
	applySorting,
	createTableStore,
} from "#/lib/table-state-store"
import type { TDataRequest } from "#/routes/_authenticated.privacy/_apis/privacy-api"
import {
	useGetOpenDataRequests,
	usePostFulfilDataRequest,
} from "#/routes/_authenticated.privacy/_hooks/use-data-requests"

const tableStore = createTableStore()

const formatInstant = (millis: number): string =>
	new Date(millis).toISOString().replace("T", " ").slice(0, 19)

const FulfilAction = ({ row }: { row: TDataRequest }): ReactElement => {
	const fulfil = usePostFulfilDataRequest()

	return (
		<Guard permissions={[PERMISSION.DATA_REQUEST_FULFIL]}>
			<button
				type="button"
				disabled={fulfil.isPending}
				onClick={() => fulfil.mutate(row.id)}
				className="border px-2 py-1 text-xs uppercase tracking-wide"
			>
				Fulfil
			</button>
		</Guard>
	)
}

const columns: ColumnDef<TDataRequest>[] = [
	{
		id: "kind",
		header: "Kind",
		cell: ({ row }) => (
			<Typography variant="paragraph">{row.original.kind}</Typography>
		),
	},
	{
		id: "email",
		header: "Who asked",
		cell: ({ row }) => (
			<Typography variant="caption">
				{row.original.email ?? undefined}
			</Typography>
		),
	},
	{
		id: "requested_at_ms",
		header: "Asked",
		cell: ({ row }) => (
			<Typography variant="caption">
				{formatInstant(row.original.requested_at_ms)}
			</Typography>
		),
	},
	{
		id: "actions",
		header: "",
		cell: ({ row }) => <FulfilAction row={row.original} />,
	},
]

export const RequestsTable = (): ReactElement => {
	const requests = useGetOpenDataRequests()
	const state = useStore(tableStore, (held) => held)
	const rows = requests.data ?? []

	return (
		<DataTable
			columns={columns}
			rows={rows}
			rowCount={rows.length}
			sorting={state.sorting}
			onSortingChange={(next) => applySorting(tableStore, next)}
			pagination={state.pagination}
			onPaginationChange={(next) => applyPagination(tableStore, next)}
			emptyMessage="Nobody is waiting on anything"
		/>
	)
}
