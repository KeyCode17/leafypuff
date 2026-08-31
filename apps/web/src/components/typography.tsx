import type { ReactElement, ReactNode } from "react"
import { match } from "ts-pattern"

type TTypographyVariant =
	| "title"
	| "sub-title"
	| "section"
	| "paragraph"
	| "caption"
	| "label"
	| "muted"

type TTypographyProps = {
	variant: TTypographyVariant
	id?: string
	className?: string
	children?: ReactNode
}

const VARIANT_CLASS: Record<TTypographyVariant, string> = {
	title: "text-2xl font-bold uppercase tracking-wide",
	"sub-title": "text-lg font-medium",
	section: "text-sm font-medium uppercase tracking-wide",
	paragraph: "text-sm font-light leading-relaxed",
	caption: "text-xs",
	label: "text-xs uppercase tracking-wide",
	muted: "text-sm text-muted-foreground",
}

export const Typography = (props: TTypographyProps): ReactElement => {
	const className = [VARIANT_CLASS[props.variant], props.className]
		.filter(Boolean)
		.join(" ")
	const content = props.children ?? "-"

	return match(props.variant)
		.with("title", () => (
			<h1 id={props.id} className={className}>
				{content}
			</h1>
		))
		.with("sub-title", () => (
			<h2 id={props.id} className={className}>
				{content}
			</h2>
		))
		.with("section", () => (
			<h3 id={props.id} className={className}>
				{content}
			</h3>
		))
		.otherwise(() => (
			<p id={props.id} className={className}>
				{content}
			</p>
		))
}
