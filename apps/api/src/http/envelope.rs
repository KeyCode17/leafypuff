use serde::Serialize;

#[derive(Serialize)]
pub struct Envelope<T: Serialize> {
    pub message: String,
    pub data: T,
    pub version: String,
}

impl<T: Serialize> Envelope<T> {
    pub fn new(message: &str, data: T) -> Self {
        Self {
            message: message.to_owned(),
            data,
            version: env!("CARGO_PKG_VERSION").to_owned(),
        }
    }
}
