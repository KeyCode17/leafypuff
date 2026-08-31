import { Store } from "@tanstack/store"

import type { TPermission } from "#/constants/permission"

export const permissionsStore = new Store<readonly TPermission[]>([])

export const setPermissions = (permissions: readonly TPermission[]): void =>
	permissionsStore.setState(() => permissions)

export const clearPermissions = (): void => permissionsStore.setState(() => [])
