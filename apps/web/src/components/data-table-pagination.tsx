import type { ReactElement } from "react"

import { Typography } from "#/components/typography"

type TDataTablePaginationProps = {
	pageIndex: number
	pageCount: number
	rowCount: number
	canPrevious: boolean
	canNext: boolean
	onPrevious: () => void
	onNext: () => void
}

export const DataTablePagination = (
	props: TDataTablePaginationProps,
): ReactElement => (
	<div className="flex items-center justify-between gap-4 pt-3">
		<Typography variant="muted">
			{`Page ${props.pageIndex + 1} of ${props.pageCount} · ${props.rowCount} total`}
		</Typography>
		<div className="flex gap-2">
			<button
				type="button"
				disabled={!props.canPrevious}
				onClick={props.onPrevious}
				className="border px-3 py-1 text-sm disabled:opacity-40"
			>
				Previous
			</button>
			<button
				type="button"
				disabled={!props.canNext}
				onClick={props.onNext}
				className="border px-3 py-1 text-sm disabled:opacity-40"
			>
				Next
			</button>
		</div>
	</div>
)
