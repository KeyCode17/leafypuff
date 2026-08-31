import { QueryErrorResetBoundary } from "@tanstack/react-query"
import { Fragment, Suspense, type ReactElement, type ReactNode } from "react"
import { ErrorBoundary } from "react-error-boundary"

import { ErrorMessage } from "#/components/error-message"
import { Skeleton } from "#/components/skeleton"

type TSectionBoundaryProps = {
	message?: string
	pending?: ReactNode
	children: ReactNode
}

const DefaultMessage = "This section could not be loaded."

export const SectionBoundary = (props: TSectionBoundaryProps): ReactElement => (
	<QueryErrorResetBoundary>
		{({ reset }) => (
			<ErrorBoundary
				onReset={reset}
				fallbackRender={({ resetErrorBoundary }) => (
					<ErrorMessage
						message={props.message ?? DefaultMessage}
						onRetry={resetErrorBoundary}
					/>
				)}
			>
				<Suspense
					fallback={
						<Fragment>
							{props.pending ?? <Skeleton className="h-24 w-full" />}
						</Fragment>
					}
				>
					{props.children}
				</Suspense>
			</ErrorBoundary>
		)}
	</QueryErrorResetBoundary>
)
