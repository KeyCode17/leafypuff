import type { ReactElement } from "react"

import {
	FigureGrid,
	type TFigure,
} from "#/routes/_authenticated.dashboard/_components/figure-grid"
import { useDashboard } from "#/routes/_authenticated.dashboard/_hooks/use-dashboard"

const BYTES_IN_A_MEBIBYTE = 1024 * 1024

export const OverviewFigures = (): ReactElement => {
	const { overview } = useDashboard()

	const figures: readonly TFigure[] = [
		{ label: "Accounts", value: String(overview.account_count) },
		{ label: "Verified", value: String(overview.verified_account_count) },
		{ label: "Suspended", value: String(overview.suspended_account_count) },
		{ label: "Entries", value: String(overview.entry_count) },
		{ label: "Tombstoned", value: String(overview.tombstoned_entry_count) },
		{ label: "Devices", value: String(overview.device_count) },
		{
			label: "Synced today",
			value: `${overview.devices_synced_last_day} of ${overview.device_count}`,
		},
		{ label: "Field conflicts", value: String(overview.field_conflict_count) },
		{ label: "Objects", value: String(overview.media_object_count) },
		{
			label: "Storage",
			value: `${(overview.media_bytes / BYTES_IN_A_MEBIBYTE).toFixed(1)} MiB`,
		},
	]

	return <FigureGrid figures={figures} />
}
