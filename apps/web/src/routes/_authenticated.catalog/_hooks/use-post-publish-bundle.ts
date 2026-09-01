import { useMutation, useQueryClient } from "@tanstack/react-query"

import { postPublishBundle } from "#/routes/_authenticated.catalog/_apis/catalog-api"
import { CATALOG_QUERY_KEY } from "#/routes/_authenticated.catalog/_hooks/use-get-catalog-bundles"

export const CATALOG_MUTATION_KEY = {
	PUBLISH: ["post-publish-bundle"],
} as const

export const usePostPublishBundle = () => {
	const queryClient = useQueryClient()
	return useMutation({
		mutationKey: CATALOG_MUTATION_KEY.PUBLISH,
		mutationFn: postPublishBundle,
		onSuccess: () =>
			queryClient.invalidateQueries({ queryKey: CATALOG_QUERY_KEY.LIST }),
	})
}
