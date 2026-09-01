import type { ReactElement } from "react"
import { match } from "ts-pattern"

import { Guard } from "#/components/guard"
import { PERMISSION } from "#/constants/permission"
import type { TAccountSummary } from "#/routes/_authenticated.accounts/_apis/accounts-api"
import {
	usePostRestoreAccount,
	usePostSuspendAccount,
} from "#/routes/_authenticated.accounts/_hooks/use-post-suspension"

type TSuspensionActionsProps = {
	account: TAccountSummary
}

export const SuspensionActions = (
	props: TSuspensionActionsProps,
): ReactElement => {
	const suspend = usePostSuspendAccount()
	const restore = usePostRestoreAccount()

	return match(props.account.suspended)
		.with(true, () => (
			<Guard permissions={[PERMISSION.ACCOUNT_RESTORE]}>
				<button
					type="button"
					disabled={restore.isPending}
					onClick={() => restore.mutate(props.account.account_id)}
					className="border px-2 py-1 text-xs uppercase tracking-wide"
				>
					Restore
				</button>
			</Guard>
		))
		.otherwise(() => (
			<Guard permissions={[PERMISSION.ACCOUNT_SUSPEND]}>
				<button
					type="button"
					disabled={suspend.isPending}
					onClick={() => suspend.mutate(props.account.account_id)}
					className="border px-2 py-1 text-xs uppercase tracking-wide"
				>
					Suspend
				</button>
			</Guard>
		))
}
