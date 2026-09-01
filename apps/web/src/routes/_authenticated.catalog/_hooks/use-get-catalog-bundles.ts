import { queryOptions, useQuery } from "@tanstack/react-query"

import { getCatalogBundles } from "#/routes/_authenticated.catalog/_apis/catalog-api"

const STALE_TIME_MS = 30_000

export const CATALOG_QUERY_KEY = {
	LIST: ["get-catalog-bundles"],
} as const

export const catalogQueryOptions = queryOptions({
	queryKey: CATALOG_QUERY_KEY.LIST,
	queryFn: getCatalogBundles,
	staleTime: STALE_TIME_MS,
})

export const useGetCatalogBundles = () => useQuery(catalogQueryOptions)
