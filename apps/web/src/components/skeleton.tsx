import type { ReactElement } from "react"

type TSkeletonProps = {
	className?: string
}

export const Skeleton = (props: TSkeletonProps): ReactElement => (
	<div
		aria-hidden="true"
		className={["animate-pulse bg-[color:var(--soft2)]", props.className]
			.filter(Boolean)
			.join(" ")}
	/>
)
