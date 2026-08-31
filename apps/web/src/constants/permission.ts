export const PERMISSION = {
	ACCOUNT_LIST: "account:list",
	ACCOUNT_READ: "account:read",
	ACCOUNT_SUSPEND: "account:suspend",
	ACCOUNT_RESTORE: "account:restore",
	ENTRY_METADATA_READ: "entry:metadata.read",
	ENTRY_COUNT_READ: "entry:count.read",
	MEDIA_USAGE_READ: "media:usage.read",
	CATALOG_READ: "catalog:read",
	CATALOG_WRITE: "catalog:write",
	CATALOG_PUBLISH: "catalog:publish",
	RELEASE_READ: "release:read",
	RELEASE_WRITE: "release:write",
	AUDIT_READ: "audit:read",
	ROLE_READ: "role:read",
	ROLE_WRITE: "role:write",
	DATA_REQUEST_READ: "data_request:read",
	DATA_REQUEST_FULFIL: "data_request:fulfil",
} as const

export type TPermission = (typeof PERMISSION)[keyof typeof PERMISSION]
