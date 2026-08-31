use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorBody {
    pub code: String,
    pub detail: String,
}

#[derive(Serialize)]
pub struct Envelope<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub error: Option<ErrorBody>,
}

impl<T: Serialize> Envelope<T> {
    pub fn ok(message: &str, data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.to_owned(),
            error: None,
        }
    }
}

impl Envelope<()> {
    pub fn failed(message: &str, code: &str, detail: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: message.to_owned(),
            error: Some(ErrorBody {
                code: code.to_owned(),
                detail: detail.to_owned(),
            }),
        }
    }
}
