use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Session {
    pub access_jwt: String,
    pub refresh_jwt: String,
    pub did: String,
    pub handle: String,
}

#[derive(Deserialize)]
struct CreateSessionResponse {
    #[serde(rename = "accessJwt")]
    access_jwt: String,
    #[serde(rename = "refreshJwt")]
    refresh_jwt: String,
    did: String,
    handle: String,
}

pub async fn create_session(
    pds: &str,
    identifier: &str,
    password: &str,
) -> Result<Session, Error> {
    let url = format!("{}/xrpc/com.atproto.server.createSession", pds);
    let body = serde_json::json!({
        "identifier": identifier,
        "password": password,
    });
    let client = reqwest::Client::new();
    let resp = client.post(&url).json(&body).send().await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(Error::Auth(format!("createSession failed ({}): {}", status, text)));
    }

    let session: CreateSessionResponse = resp.json().await?;
    Ok(Session {
        access_jwt: session.access_jwt,
        refresh_jwt: session.refresh_jwt,
        did: session.did,
        handle: session.handle,
    })
}
