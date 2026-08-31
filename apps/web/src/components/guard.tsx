import { useStore } from "@tanstack/react-store"
import { Fragment, type ReactElement, type ReactNode } from "react"
import { match } from "ts-pattern"

import type { TPermission } from "#/constants/permission"
import { permissionsStore } from "#/lib/permissions-store"

type TGuardProps = {
	permissions: readonly TPermission[]
	mode?: "all" | "any"
	children: ReactNode
}

export const Guard = (props: TGuardProps): ReactElement => {
	const granted = useStore(permissionsStore, (state) => state)
	const mode = props.mode ?? "all"

	const allowed = match(mode)
		.with("all", () =>
			props.permissions.every((permission) => granted.includes(permission)),
		)
		.with("any", () =>
			props.permissions.some((permission) => granted.includes(permission)),
		)
		.exhaustive()

	return match(allowed)
		.with(true, () => <Fragment>{props.children}</Fragment>)
		.otherwise(() => <Fragment />)
}
