import {
	flexRender,
	getCoreRowModel,
	useReactTable,
	type ColumnDef,
	type OnChangeFn,
	type PaginationState,
	type SortingState,
} from "@tanstack/react-table"
import type { ReactElement } from "react"
import { match } from "ts-pattern"

import { DataTablePagination } from "#/components/data-table-pagination"
import { EmptyState } from "#/components/empty-state"

type TDataTableProps<TRow> = {
	columns: ColumnDef<TRow, unknown>[]
	rows: TRow[]
	rowCount: number
	sorting: SortingState
	onSortingChange: OnChangeFn<SortingState>
	pagination: PaginationState
	onPaginationChange: OnChangeFn<PaginationState>
	emptyMessage: string
}

export const DataTable = <TRow,>(
	props: TDataTableProps<TRow>,
): ReactElement => {
	const table = useReactTable({
		data: props.rows,
		columns: props.columns,
		rowCount: props.rowCount,
		state: { sorting: props.sorting, pagination: props.pagination },
		onSortingChange: props.onSortingChange,
		onPaginationChange: props.onPaginationChange,
		manualSorting: true,
		manualFiltering: true,
		manualPagination: true,
		getCoreRowModel: getCoreRowModel(),
	})

	return (
		<div className="w-full overflow-x-auto">
			<table className="w-full border-collapse text-sm">
				<thead>
					{table.getHeaderGroups().map((group) => (
						<tr key={group.id}>
							{group.headers.map((header) => (
								<th
									key={header.id}
									scope="col"
									className="px-3 py-2 text-left font-medium uppercase tracking-wide"
								>
									{header.isPlaceholder
										? null
										: flexRender(
												header.column.columnDef.header,
												header.getContext(),
											)}
								</th>
							))}
						</tr>
					))}
				</thead>
				<tbody>
					{match(table.getRowModel().rows.length)
						.with(0, () => (
							<tr>
								<td colSpan={props.columns.length}>
									<EmptyState message={props.emptyMessage} />
								</td>
							</tr>
						))
						.otherwise(() =>
							table.getRowModel().rows.map((row) => (
								<tr key={row.id}>
									{row.getVisibleCells().map((cell) => (
										<td key={cell.id} className="px-3 py-2">
											{flexRender(
												cell.column.columnDef.cell,
												cell.getContext(),
											)}
										</td>
									))}
								</tr>
							)),
						)}
				</tbody>
			</table>

			{table.getPageCount() > 1 && (
				<DataTablePagination
					pageIndex={props.pagination.pageIndex}
					pageCount={table.getPageCount()}
					rowCount={props.rowCount}
					canPrevious={table.getCanPreviousPage()}
					canNext={table.getCanNextPage()}
					onPrevious={() => table.previousPage()}
					onNext={() => table.nextPage()}
				/>
			)}
		</div>
	)
}
