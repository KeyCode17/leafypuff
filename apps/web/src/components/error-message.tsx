import type { ReactElement } from "react"

import { Typography } from "#/components/typography"

type TErrorMessageProps = {
	message: string
	onRetry?: () => void
}

export const ErrorMessage = (props: TErrorMessageProps): ReactElement => (
	<div role="alert" className="flex flex-col items-start gap-2 py-6">
		<Typography variant="paragraph">{props.message}</Typography>
		{props.onRetry && (
			<button
				type="button"
				onClick={props.onRetry}
				className="text-sm underline"
			>
				Try again
			</button>
		)}
	</div>
)
