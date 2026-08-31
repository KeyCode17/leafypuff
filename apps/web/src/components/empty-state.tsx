import type { ReactElement } from "react"

import { Typography } from "#/components/typography"

type TEmptyStateProps = {
	message: string
}

export const EmptyState = (props: TEmptyStateProps): ReactElement => (
	<output className="flex flex-col items-center gap-2 py-12">
		<Typography variant="muted">{props.message}</Typography>
	</output>
)
