import { useMutation } from "@tanstack/react-query"

import { sessionStore, startSession } from "#/lib/session-store"
import { postVerifySignIn } from "#/routes/login/_apis/auth-api"
import { AUTH_MUTATION_KEY } from "#/routes/login/_hooks/use-post-sign-in"

export const usePostVerifySignIn = (onSignedIn: () => void) =>
	useMutation({
		mutationKey: AUTH_MUTATION_KEY.VERIFY_SIGN_IN,
		mutationFn: (payload: { email: string; code: string }) =>
			postVerifySignIn({
				...payload,
				device_id: sessionStore.state.deviceId,
			}),
		onSuccess: (session) => {
			startSession(session.access_token, session.refresh_token)
			onSignedIn()
		},
	})
