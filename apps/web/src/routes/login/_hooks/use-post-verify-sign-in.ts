import { useMutation } from "@tanstack/react-query"

import { PERMISSION, type TPermission } from "#/constants/permission"

import { setPermissions } from "#/lib/permissions-store"
import { sessionStore, startSession } from "#/lib/session-store"
import { postVerifySignIn } from "#/routes/login/_apis/auth-api"
import { getGrantedPermissions } from "#/routes/_authenticated.roles/_apis/roles-api"
import { AUTH_MUTATION_KEY } from "#/routes/login/_hooks/use-post-sign-in"

export const usePostVerifySignIn = (onSignedIn: () => void) =>
	useMutation({
		mutationKey: AUTH_MUTATION_KEY.VERIFY_SIGN_IN,
		mutationFn: (payload: { email: string; code: string }) =>
			postVerifySignIn({
				...payload,
				device_id: sessionStore.state.deviceId,
			}),
		onSuccess: async (session) => {
			startSession(session.access_token, session.refresh_token)
			const granted = await getGrantedPermissions()
			setPermissions(readPermissions(granted.permissions))
			onSignedIn()
		},
	})

const KNOWN: readonly TPermission[] = Object.values(PERMISSION)

const readPermissions = (granted: string[]): readonly TPermission[] =>
	KNOWN.filter((permission) => granted.includes(permission))
