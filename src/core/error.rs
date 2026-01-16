use thiserror::Error;

#[derive(Error, Debug)]
pub enum RongCloudError {
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("API error {code}: {msg}")]
    Api { code: i32, msg: String },

    #[error("Unknown error")]
    Unknown,
}
