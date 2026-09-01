import type { ReactElement } from "react"

import { Typography } from "#/components/typography"

export type TFigure = {
	label: string
	value: string
}

type TFigureGridProps = {
	figures: readonly TFigure[]
}

export const FigureGrid = (props: TFigureGridProps): ReactElement => (
	<dl className="grid grid-cols-2 gap-6 md:grid-cols-4">
		{props.figures.map((figure) => (
			<div key={figure.label} className="flex flex-col gap-1">
				<dt>
					<Typography variant="label">{figure.label}</Typography>
				</dt>
				<dd>
					<Typography variant="sub-title">{figure.value}</Typography>
				</dd>
			</div>
		))}
	</dl>
)
