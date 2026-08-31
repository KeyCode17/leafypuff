import { useForm } from "@tanstack/react-form"
import type { ReactElement } from "react"

import { ErrorMessage } from "#/components/error-message"
import { SubmitButton } from "#/components/submit-button"
import { TextField } from "#/components/text-field"
import { Typography } from "#/components/typography"
import { usePostSignIn } from "#/routes/login/_hooks/use-post-sign-in"
import { signInSchema } from "#/routes/login/_schema/sign-in-schema"

type TSignInFormProps = {
	onCodeSent: (email: string) => void
}

export const SignInForm = (props: TSignInFormProps): ReactElement => {
	const signIn = usePostSignIn(props.onCodeSent)
	const form = useForm({
		defaultValues: { email: "", password: "" },
		validators: { onChange: signInSchema, onSubmit: signInSchema },
		onSubmit: ({ value }) => signIn.mutateAsync(value),
	})

	return (
		<form
			className="flex flex-col gap-4"
			onSubmit={(event) => {
				event.preventDefault()
				void form.handleSubmit()
			}}
		>
			<Typography variant="title">Sign in</Typography>
			<form.Field name="email">
				{(field) => (
					<TextField
						name={field.name}
						label="Email"
						type="email"
						autoComplete="email"
						value={field.state.value}
						error={field.state.meta.errors[0]?.message}
						onChange={field.handleChange}
						onBlur={field.handleBlur}
					/>
				)}
			</form.Field>
			<form.Field name="password">
				{(field) => (
					<TextField
						name={field.name}
						label="Password"
						type="password"
						autoComplete="current-password"
						value={field.state.value}
						error={field.state.meta.errors[0]?.message}
						onChange={field.handleChange}
						onBlur={field.handleBlur}
					/>
				)}
			</form.Field>
			{signIn.isError && <ErrorMessage message={signIn.error.message} />}
			<form.Subscribe
				selector={(state) => [state.canSubmit, state.isSubmitting]}
			>
				{([canSubmit, isSubmitting]) => (
					<SubmitButton
						label="Send me a code"
						pendingLabel="Sending"
						pending={isSubmitting === true}
						disabled={canSubmit === false}
					/>
				)}
			</form.Subscribe>
		</form>
	)
}
