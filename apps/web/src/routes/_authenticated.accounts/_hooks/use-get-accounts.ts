import { queryOptions, useQuery } from "@tanstack/react-query"

import { getAccounts } from "#/routes/_authenticated.accounts/_apis/accounts-api"

const STALE_TIME_MS = 30_000

export const ACCOUNTS_QUERY_KEY = {
	LIST: ["get-accounts"],
} as const

export const accountsQueryOptions = queryOptions({
	queryKey: ACCOUNTS_QUERY_KEY.LIST,
	queryFn: getAccounts,
	staleTime: STALE_TIME_MS,
})

export const useGetAccounts = () => useQuery(accountsQueryOptions)
