import { createFileRoute, useNavigate } from "@tanstack/react-router"
import { useStore } from "@tanstack/react-store"
import { Store } from "@tanstack/store"
import type { ReactElement } from "react"
import { match } from "ts-pattern"

import { SignInForm } from "#/routes/login/_components/sign-in-form"
import { VerifyCodeForm } from "#/routes/login/_components/verify-code-form"

const challengedEmailStore = new Store<string | null>(null)

const LoginScreen = (): ReactElement => {
	const navigate = useNavigate()
	const challenged = useStore(challengedEmailStore, (state) => state)

	return (
		<main className="mx-auto flex min-h-dvh w-full max-w-sm flex-col justify-center gap-6 px-6">
			{match(challenged)
				.with(null, () => (
					<SignInForm
						onCodeSent={(email) => challengedEmailStore.setState(() => email)}
					/>
				))
				.otherwise((email) => (
					<VerifyCodeForm
						email={email}
						onSignedIn={() => {
							challengedEmailStore.setState(() => null)
							void navigate({ to: "/dashboard" })
						}}
					/>
				))}
		</main>
	)
}

export const Route = createFileRoute("/login/")({
	component: LoginScreen,
})
