import { useMutation, useQueryClient } from "@tanstack/react-query"

import {
	postRestoreAccount,
	postSuspendAccount,
} from "#/routes/_authenticated.accounts/_apis/accounts-api"
import { ACCOUNTS_QUERY_KEY } from "#/routes/_authenticated.accounts/_hooks/use-get-accounts"

export const ACCOUNT_MUTATION_KEY = {
	SUSPEND: ["post-suspend-account"],
	RESTORE: ["post-restore-account"],
} as const

export const usePostSuspendAccount = () => {
	const queryClient = useQueryClient()
	return useMutation({
		mutationKey: ACCOUNT_MUTATION_KEY.SUSPEND,
		mutationFn: postSuspendAccount,
		onSuccess: () =>
			queryClient.invalidateQueries({ queryKey: ACCOUNTS_QUERY_KEY.LIST }),
	})
}

export const usePostRestoreAccount = () => {
	const queryClient = useQueryClient()
	return useMutation({
		mutationKey: ACCOUNT_MUTATION_KEY.RESTORE,
		mutationFn: postRestoreAccount,
		onSuccess: () =>
			queryClient.invalidateQueries({ queryKey: ACCOUNTS_QUERY_KEY.LIST }),
	})
}
