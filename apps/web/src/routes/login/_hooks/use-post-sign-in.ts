import { useMutation } from "@tanstack/react-query"

import { postSignIn } from "#/routes/login/_apis/auth-api"

export const AUTH_MUTATION_KEY = {
	SIGN_IN: ["post-sign-in"],
	VERIFY_SIGN_IN: ["post-verify-sign-in"],
} as const

export const usePostSignIn = (onIssued: (email: string) => void) =>
	useMutation({
		mutationKey: AUTH_MUTATION_KEY.SIGN_IN,
		mutationFn: postSignIn,
		onSuccess: (_data, variables) => onIssued(variables.email),
	})
