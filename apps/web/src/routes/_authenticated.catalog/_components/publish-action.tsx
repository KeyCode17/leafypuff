import type { ReactElement } from "react"
import { match } from "ts-pattern"

import { Guard } from "#/components/guard"
import { Typography } from "#/components/typography"
import { PERMISSION } from "#/constants/permission"
import type { TCatalogBundle } from "#/routes/_authenticated.catalog/_apis/catalog-api"
import { usePostPublishBundle } from "#/routes/_authenticated.catalog/_hooks/use-post-publish-bundle"

type TPublishActionProps = {
	bundle: TCatalogBundle
}

export const PublishAction = (props: TPublishActionProps): ReactElement => {
	const publish = usePostPublishBundle()

	return match(props.bundle.published)
		.with(true, () => <Typography variant="label">Live</Typography>)
		.otherwise(() => (
			<Guard permissions={[PERMISSION.CATALOG_PUBLISH]}>
				<button
					type="button"
					disabled={publish.isPending}
					onClick={() => publish.mutate(props.bundle.id)}
					className="border px-2 py-1 text-xs uppercase tracking-wide"
				>
					Publish
				</button>
			</Guard>
		))
}
