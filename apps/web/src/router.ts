import { createRouter } from "@tanstack/react-router"

import { getQueryClient } from "#/lib/query-client"
import { routeTree } from "#/routeTree.gen"

export const router = createRouter({
	routeTree,
	context: { queryClient: getQueryClient() },
	defaultPendingMs: 1000,
	defaultPendingMinMs: 500,
})

declare module "@tanstack/react-router" {
	interface Register {
		router: typeof router
	}
}
