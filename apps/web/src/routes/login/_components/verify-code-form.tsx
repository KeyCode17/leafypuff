import { useForm } from "@tanstack/react-form"
import type { ReactElement } from "react"

import { ErrorMessage } from "#/components/error-message"
import { SubmitButton } from "#/components/submit-button"
import { TextField } from "#/components/text-field"
import { Typography } from "#/components/typography"
import { usePostVerifySignIn } from "#/routes/login/_hooks/use-post-verify-sign-in"
import { verifyCodeSchema } from "#/routes/login/_schema/sign-in-schema"

type TVerifyCodeFormProps = {
	email: string
	onSignedIn: () => void
}

export const VerifyCodeForm = (props: TVerifyCodeFormProps): ReactElement => {
	const verify = usePostVerifySignIn(props.onSignedIn)
	const form = useForm({
		defaultValues: { code: "" },
		validators: { onChange: verifyCodeSchema, onSubmit: verifyCodeSchema },
		onSubmit: ({ value }) =>
			verify.mutateAsync({ email: props.email, code: value.code }),
	})

	return (
		<form
			className="flex flex-col gap-4"
			onSubmit={(event) => {
				event.preventDefault()
				void form.handleSubmit()
			}}
		>
			<Typography variant="title">Check your inbox</Typography>
			<Typography variant="paragraph">
				{`We sent a six-digit code to ${props.email}.`}
			</Typography>
			<form.Field name="code">
				{(field) => (
					<TextField
						name={field.name}
						label="Code"
						type="text"
						autoComplete="one-time-code"
						value={field.state.value}
						error={field.state.meta.errors[0]?.message}
						onChange={field.handleChange}
						onBlur={field.handleBlur}
					/>
				)}
			</form.Field>
			{verify.isError && <ErrorMessage message={verify.error.message} />}
			<form.Subscribe
				selector={(state) => [state.canSubmit, state.isSubmitting]}
			>
				{([canSubmit, isSubmitting]) => (
					<SubmitButton
						label="Sign in"
						pendingLabel="Signing in"
						pending={isSubmitting === true}
						disabled={canSubmit === false}
					/>
				)}
			</form.Subscribe>
		</form>
	)
}
