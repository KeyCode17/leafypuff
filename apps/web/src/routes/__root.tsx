import type { QueryClient } from "@tanstack/react-query"
import { Outlet, createRootRouteWithContext } from "@tanstack/react-router"
import type { ReactElement } from "react"

type TRouterContext = {
	queryClient: QueryClient
}

const RootLayout = (): ReactElement => <Outlet />

export const Route = createRootRouteWithContext<TRouterContext>()({
	component: RootLayout,
})
