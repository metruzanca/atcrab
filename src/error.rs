use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("DNS resolution failed: {0}")]
    Dns(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("missing _atproto TXT record for handle")]
    MissingAtprotoRecord,

    #[error("no ATProto PDS service found in DID document")]
    MissingPdsService,

    #[error("unsupported DID method: {0}")]
    UnsupportedDidMethod(String),

    #[error("invalid handle: {0}")]
    InvalidHandle(String),

    #[error("auth error: {0}")]
    Auth(String),

    #[error("HTTP {}: {}", .0, .1)]
    Status(reqwest::StatusCode, String),
}
