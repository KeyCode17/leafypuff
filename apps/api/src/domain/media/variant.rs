#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Original,
    Derivative,
}

impl Variant {
    pub const ALL: [Self; 2] = [Self::Original, Self::Derivative];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Original => "original",
            Self::Derivative => "derivative",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "original" => Some(Self::Original),
            "derivative" => Some(Self::Derivative),
            _ => None,
        }
    }
}
