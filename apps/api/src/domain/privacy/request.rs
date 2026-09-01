use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestKind {
    Export,
    Erasure,
}

impl RequestKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Export => "export",
            Self::Erasure => "erasure",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "export" => Some(Self::Export),
            "erasure" => Some(Self::Erasure),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestStatus {
    Received,
    Fulfilled,
}

impl RequestStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Received => "received",
            Self::Fulfilled => "fulfilled",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "received" => Some(Self::Received),
            "fulfilled" => Some(Self::Fulfilled),
            _ => None,
        }
    }
}

/// A request outlives the account it names. An erasure that deleted its own record would leave
/// nobody able to show the erasure happened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataRequest {
    pub id: Uuid,
    pub account_id: Uuid,
    pub email: Option<String>,
    pub kind: RequestKind,
    pub status: RequestStatus,
    pub requested_at_ms: i64,
    pub fulfilled_at_ms: Option<i64>,
    pub fulfilled_by: Option<Uuid>,
}
