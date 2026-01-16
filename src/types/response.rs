use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RcResponse<T = ()> {
    pub code: i32,
    #[serde(default)]
    pub msg: String,
    // Request ID is typically in keys like "requestId" or "request_id" depending on the endpoint JSON,
    // but sometimes it's injected by the Java SDK logic.
    // The API list shows JSON examples? No, just URLs.
    // I'll stick to a generic optional type for data.
    #[serde(flatten)]
    pub data: Option<T>,
}

impl<T> RcResponse<T> {
    pub fn is_success(&self) -> bool {
        self.code == 200
    }
}
