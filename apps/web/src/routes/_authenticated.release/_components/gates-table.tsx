import type { ColumnDef } from "@tanstack/react-table"
import { useStore } from "@tanstack/react-store"
import type { ReactElement } from "react"
import { match } from "ts-pattern"

import { DataTable } from "#/components/data-table"
import { Guard } from "#/components/guard"
import { Typography } from "#/components/typography"
import { PERMISSION } from "#/constants/permission"
import {
	applyPagination,
	applySorting,
	createTableStore,
} from "#/lib/table-state-store"
import type { TReleaseGate } from "#/routes/_authenticated.release/_apis/release-api"
import {
	useGetReleaseGates,
	usePostReleaseGate,
} from "#/routes/_authenticated.release/_hooks/use-release"

const tableStore = createTableStore()

const ForceToggle = ({ gate }: { gate: TReleaseGate }): ReactElement => {
	const setGate = usePostReleaseGate()

	return (
		<Guard permissions={[PERMISSION.RELEASE_WRITE]}>
			<button
				type="button"
				disabled={setGate.isPending}
				onClick={() =>
					setGate.mutate({
						platform: gate.platform,
						minimum_build: gate.minimum_build,
						force_update: !gate.force_update,
						message: gate.message,
					})
				}
				className="border px-2 py-1 text-xs uppercase tracking-wide"
			>
				{match(gate.force_update)
					.with(true, () => "Stop forcing")
					.otherwise(() => "Force update")}
			</button>
		</Guard>
	)
}

const columns: ColumnDef<TReleaseGate>[] = [
	{
		id: "platform",
		header: "Platform",
		cell: ({ row }) => (
			<Typography variant="paragraph">{row.original.platform}</Typography>
		),
	},
	{
		id: "minimum_build",
		header: "Minimum build",
		cell: ({ row }) => (
			<Typography variant="caption">
				{String(row.original.minimum_build)}
			</Typography>
		),
	},
	{
		id: "force_update",
		header: "Forcing",
		cell: ({ row }) => (
			<Typography variant="caption">
				{match(row.original.force_update)
					.with(true, () => "Sync is blocked below the minimum")
					.otherwise(() => "Older builds still sync")}
			</Typography>
		),
	},
	{
		id: "actions",
		header: "",
		cell: ({ row }) => <ForceToggle gate={row.original} />,
	},
]

export const GatesTable = (): ReactElement => {
	const gates = useGetReleaseGates()
	const state = useStore(tableStore, (held) => held)
	const rows = gates.data ?? []

	return (
		<DataTable
			columns={columns}
			rows={rows}
			rowCount={rows.length}
			sorting={state.sorting}
			onSortingChange={(next) => applySorting(tableStore, next)}
			pagination={state.pagination}
			onPaginationChange={(next) => applyPagination(tableStore, next)}
			emptyMessage="No platform has a gate yet"
		/>
	)
}
