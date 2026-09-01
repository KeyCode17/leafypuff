import {
	queryOptions,
	useMutation,
	useQuery,
	useQueryClient,
} from "@tanstack/react-query"

import {
	getCampaigns,
	getReleaseGates,
	postReleaseGate,
} from "#/routes/_authenticated.release/_apis/release-api"

const STALE_TIME_MS = 30_000

export const RELEASE_QUERY_KEY = {
	GATES: ["get-release-gates"],
	CAMPAIGNS: ["get-campaigns"],
} as const

export const RELEASE_MUTATION_KEY = {
	SET_GATE: ["post-release-gate"],
} as const

export const gatesQueryOptions = queryOptions({
	queryKey: RELEASE_QUERY_KEY.GATES,
	queryFn: getReleaseGates,
	staleTime: STALE_TIME_MS,
})

export const campaignsQueryOptions = queryOptions({
	queryKey: RELEASE_QUERY_KEY.CAMPAIGNS,
	queryFn: getCampaigns,
	staleTime: STALE_TIME_MS,
})

export const useGetReleaseGates = () => useQuery(gatesQueryOptions)

export const useGetCampaigns = () => useQuery(campaignsQueryOptions)

export const usePostReleaseGate = () => {
	const queryClient = useQueryClient()
	return useMutation({
		mutationKey: RELEASE_MUTATION_KEY.SET_GATE,
		mutationFn: postReleaseGate,
		onSuccess: () =>
			queryClient.invalidateQueries({ queryKey: RELEASE_QUERY_KEY.GATES }),
	})
}
