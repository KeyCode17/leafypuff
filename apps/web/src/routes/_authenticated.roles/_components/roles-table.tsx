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
import type { TRole } from "#/routes/_authenticated.roles/_apis/roles-api"
import { useGetRoles } from "#/routes/_authenticated.roles/_hooks/use-get-roles"

const tableStore = createTableStore()

const columns: ColumnDef<TRole>[] = [
	{
		id: "name",
		header: "Role",
		cell: ({ row }) => (
			<Typography variant="paragraph">{row.original.name}</Typography>
		),
	},
	{
		id: "description",
		header: "What it is for",
		cell: ({ row }) => (
			<Typography variant="muted">
				{row.original.description ?? undefined}
			</Typography>
		),
	},
	{
		id: "permissions",
		header: "Permissions",
		cell: ({ row }) => (
			<Typography variant="caption">
				{row.original.permissions.join(", ")}
			</Typography>
		),
	},
]

export const RolesTable = (): ReactElement => {
	const roles = useGetRoles()
	const state = useStore(tableStore, (held) => held)
	const rows = roles.data ?? []

	return (
		<DataTable
			columns={columns}
			rows={rows}
			rowCount={rows.length}
			sorting={state.sorting}
			onSortingChange={(next) => applySorting(tableStore, next)}
			pagination={state.pagination}
			onPaginationChange={(next) => applyPagination(tableStore, next)}
			emptyMessage="No roles are defined yet"
		/>
	)
}
