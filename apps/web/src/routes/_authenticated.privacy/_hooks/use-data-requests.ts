import {
	queryOptions,
	useMutation,
	useQuery,
	useQueryClient,
} from "@tanstack/react-query"

import {
	getOpenDataRequests,
	postFulfilDataRequest,
} from "#/routes/_authenticated.privacy/_apis/privacy-api"

const STALE_TIME_MS = 15_000

export const PRIVACY_QUERY_KEY = {
	OPEN: ["get-open-data-requests"],
} as const

export const PRIVACY_MUTATION_KEY = {
	FULFIL: ["post-fulfil-data-request"],
} as const

export const openRequestsQueryOptions = queryOptions({
	queryKey: PRIVACY_QUERY_KEY.OPEN,
	queryFn: getOpenDataRequests,
	staleTime: STALE_TIME_MS,
})

export const useGetOpenDataRequests = () => useQuery(openRequestsQueryOptions)

export const usePostFulfilDataRequest = () => {
	const queryClient = useQueryClient()
	return useMutation({
		mutationKey: PRIVACY_MUTATION_KEY.FULFIL,
		mutationFn: postFulfilDataRequest,
		onSuccess: () =>
			queryClient.invalidateQueries({ queryKey: PRIVACY_QUERY_KEY.OPEN }),
	})
}
