use reqwest::Client;
use serde::Deserialize;

use crate::error::Error;
use crate::lexicons::types::Blob;

#[derive(Deserialize)]
struct UploadBlobResponse {
    blob: Blob,
}

pub async fn upload_blob(
    client: &Client,
    pds: &str,
    token: &str,
    data: Vec<u8>,
    mime_type: &str,
) -> Result<Blob, Error> {
    let url = format!("{}/xrpc/com.atproto.repo.uploadBlob", pds);
    let resp = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", mime_type)
        .body(data)
        .send()
        .await?;

    let status = resp.status();
    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err(Error::Status(status, text));
    }

    let body: UploadBlobResponse = resp.json().await?;
    Ok(body.blob)
}
