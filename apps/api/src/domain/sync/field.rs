#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptedField {
    Title,
    Body,
}

impl EncryptedField {
    pub const ALL: [Self; 2] = [Self::Title, Self::Body];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Body => "body",
        }
    }
}
