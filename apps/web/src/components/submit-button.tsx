import type { ReactElement } from "react"
import { match } from "ts-pattern"

type TSubmitButtonProps = {
	label: string
	pendingLabel: string
	pending: boolean
	disabled: boolean
}

export const SubmitButton = (props: TSubmitButtonProps): ReactElement => (
	<button
		type="submit"
		disabled={props.disabled || props.pending}
		className="border px-4 py-2 text-sm uppercase tracking-wide disabled:opacity-50"
	>
		{match(props.pending)
			.with(true, () => props.pendingLabel)
			.otherwise(() => props.label)}
	</button>
)
