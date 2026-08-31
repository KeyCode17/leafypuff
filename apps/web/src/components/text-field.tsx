import type { ReactElement } from "react"

import { Typography } from "#/components/typography"

type TTextFieldProps = {
	name: string
	label: string
	type: "text" | "email" | "password"
	value: string
	error?: string
	autoComplete?: string
	onChange: (value: string) => void
	onBlur: () => void
}

export const TextField = (props: TTextFieldProps): ReactElement => (
	<div className="flex flex-col gap-1">
		<label htmlFor={props.name}>
			<Typography variant="label">{props.label}</Typography>
		</label>
		<input
			id={props.name}
			name={props.name}
			type={props.type}
			value={props.value}
			autoComplete={props.autoComplete}
			aria-invalid={props.error !== undefined}
			aria-describedby={
				props.error === undefined ? undefined : `${props.name}-error`
			}
			onChange={(event) => props.onChange(event.target.value)}
			onBlur={props.onBlur}
			className="border px-3 py-2 text-sm"
		/>
		{props.error !== undefined && (
			<Typography id={`${props.name}-error`} variant="caption">
				{props.error}
			</Typography>
		)}
	</div>
)
