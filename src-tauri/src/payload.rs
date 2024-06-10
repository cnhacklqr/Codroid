#[derive(Clone, serde::Serialize)]
pub struct Payload {
    message: String,
}

impl Payload {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
