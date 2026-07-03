use serde::Deserialize;

use crate::error::Error;

#[derive(Deserialize)]
struct DidDocument {
    #[allow(dead_code)]
    id: String,
    service: Option<Vec<Service>>,
}

#[derive(Deserialize)]
struct Service {
    #[serde(rename = "type")]
    service_type: String,
    #[serde(rename = "serviceEndpoint")]
    service_endpoint: String,
}

pub async fn resolve_pds(did: &str) -> Result<String, Error> {
    if did.starts_with("did:plc:") {
        resolve_plc(did).await
    } else if did.starts_with("did:web:") {
        resolve_web(did).await
    } else {
        Err(Error::UnsupportedDidMethod(did.to_string()))
    }
}

async fn resolve_plc(did: &str) -> Result<String, Error> {
    let url = format!("https://plc.directory/{}", did);
    let resp = reqwest::get(&url).await?;
    let doc: DidDocument = resp.json().await?;
    find_pds_service(&doc)
}

async fn resolve_web(did: &str) -> Result<String, Error> {
    let domain = did
        .strip_prefix("did:web:")
        .ok_or_else(|| Error::InvalidHandle(did.to_string()))?;
    let url = format!("https://{}/.well-known/did.json", domain);
    let resp = reqwest::get(&url).await?;
    let doc: DidDocument = resp.json().await?;
    find_pds_service(&doc)
}

fn find_pds_service(doc: &DidDocument) -> Result<String, Error> {
    if let Some(services) = &doc.service {
        for svc in services {
            if svc.service_type == "AtprotoPersonalDataServer" {
                return Ok(svc.service_endpoint.clone());
            }
        }
    }
    Err(Error::MissingPdsService)
}
