use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Android,
    Web,
}

impl Platform {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Android => "android",
            Self::Web => "web",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "android" => Some(Self::Android),
            "web" => Some(Self::Web),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseGate {
    pub platform: Platform,
    pub minimum_build: i32,
    pub force_update: bool,
    pub message: Option<String>,
    pub updated_at_ms: i64,
    pub updated_by: Option<Uuid>,
}

impl ReleaseGate {
    pub const fn blocks(&self, build: i32) -> bool {
        self.force_update && build < self.minimum_build
    }

    pub const fn is_behind(&self, build: i32) -> bool {
        build < self.minimum_build
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Campaign {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub platform: Platform,
    pub starts_at_ms: i64,
    pub ends_at_ms: i64,
    pub published: bool,
    pub created_at_ms: i64,
}

impl Campaign {
    pub const fn is_live(&self, at_ms: i64) -> bool {
        self.published && self.starts_at_ms <= at_ms && at_ms < self.ends_at_ms
    }
}
