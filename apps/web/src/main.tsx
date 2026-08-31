import { QueryClientProvider } from "@tanstack/react-query"
import { RouterProvider } from "@tanstack/react-router"
import { StrictMode } from "react"
import { createRoot } from "react-dom/client"

import { getQueryClient } from "#/lib/query-client"
import { router } from "#/router"

const container = document.getElementById("root")

if (container) {
	createRoot(container).render(
		<StrictMode>
			<QueryClientProvider client={getQueryClient()}>
				<RouterProvider router={router} />
			</QueryClientProvider>
		</StrictMode>,
	)
}
