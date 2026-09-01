#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    AccountList,
    AccountRead,
    AccountSuspend,
    AccountRestore,
    EntryMetadataRead,
    EntryCountRead,
    MediaUsageRead,
    CatalogRead,
    CatalogWrite,
    CatalogPublish,
    ReleaseRead,
    ReleaseWrite,
    AuditRead,
    RoleRead,
    RoleWrite,
    DataRequestRead,
    DataRequestFulfil,
}

impl Permission {
    pub const ALL: [Self; 17] = [
        Self::AccountList,
        Self::AccountRead,
        Self::AccountSuspend,
        Self::AccountRestore,
        Self::EntryMetadataRead,
        Self::EntryCountRead,
        Self::MediaUsageRead,
        Self::CatalogRead,
        Self::CatalogWrite,
        Self::CatalogPublish,
        Self::ReleaseRead,
        Self::ReleaseWrite,
        Self::AuditRead,
        Self::RoleRead,
        Self::RoleWrite,
        Self::DataRequestRead,
        Self::DataRequestFulfil,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccountList => "account:list",
            Self::AccountRead => "account:read",
            Self::AccountSuspend => "account:suspend",
            Self::AccountRestore => "account:restore",
            Self::EntryMetadataRead => "entry:metadata.read",
            Self::EntryCountRead => "entry:count.read",
            Self::MediaUsageRead => "media:usage.read",
            Self::CatalogRead => "catalog:read",
            Self::CatalogWrite => "catalog:write",
            Self::CatalogPublish => "catalog:publish",
            Self::ReleaseRead => "release:read",
            Self::ReleaseWrite => "release:write",
            Self::AuditRead => "audit:read",
            Self::RoleRead => "role:read",
            Self::RoleWrite => "role:write",
            Self::DataRequestRead => "data_request:read",
            Self::DataRequestFulfil => "data_request:fulfil",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|held| held.as_str() == raw)
    }
}
