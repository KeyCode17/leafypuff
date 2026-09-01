import { createFileRoute } from "@tanstack/react-router"
import type { ReactElement } from "react"

import { Guard } from "#/components/guard"
import { SectionBoundary } from "#/components/section-boundary"
import { Typography } from "#/components/typography"
import { PERMISSION } from "#/constants/permission"
import { CatalogTable } from "#/routes/_authenticated.catalog/_components/catalog-table"

const CatalogScreen = (): ReactElement => (
	<main className="flex flex-col gap-6 p-8">
		<Typography variant="title">Catalog</Typography>
		<Typography variant="paragraph">
			The twelve moods and eight stickers stay compiled into the app. A bundle
			is how a later addition reaches a device that has already shipped.
		</Typography>
		<Guard permissions={[PERMISSION.CATALOG_READ]}>
			<SectionBoundary>
				<CatalogTable />
			</SectionBoundary>
		</Guard>
	</main>
)

export const Route = createFileRoute("/_authenticated/catalog/")({
	component: CatalogScreen,
})
