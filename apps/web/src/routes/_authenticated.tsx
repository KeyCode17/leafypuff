import { Outlet, createFileRoute, redirect } from "@tanstack/react-router"
import type { ReactElement } from "react"

import { isSignedIn } from "#/lib/session-store"

const AuthenticatedLayout = (): ReactElement => <Outlet />

export const Route = createFileRoute("/_authenticated")({
	beforeLoad: () => {
		if (!isSignedIn()) {
			throw redirect({ to: "/login" })
		}
	},
	component: AuthenticatedLayout,
})
