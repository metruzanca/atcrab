use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::TokioAsyncResolver;

use crate::error::Error;

pub async fn resolve_handle(handle: &str) -> Result<String, Error> {
    let url = format!("https://{}/.well-known/atproto-did", handle);
    if let Ok(resp) = reqwest::get(&url).await {
        if resp.status().is_success() {
            if let Ok(did) = resp.text().await {
                let did = did.trim().to_string();
                if did.starts_with("did:") {
                    return Ok(did);
                }
            }
        }
    }

    let resolver =
        TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
    let domain = format!("_atproto.{}", handle);
    let response = resolver
        .txt_lookup(domain)
        .await
        .map_err(|e| Error::Dns(e.to_string()))?;

    for record in response.iter() {
        let txt = record.to_string();
        if let Some(did) = txt.strip_prefix("did=") {
            return Ok(did.to_string());
        }
    }

    Err(Error::MissingAtprotoRecord)
}
