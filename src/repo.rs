use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::did;
use crate::error::Error;
use crate::handle;
use crate::types::{ListRecords, Record};

pub struct Repo {
    pub did: String,
    pub pds: String,
    client: Client,
}

impl Repo {
    pub async fn new(handle: &str) -> Result<Self, Error> {
        let did = handle::resolve_handle(handle).await?;
        let pds = did::resolve_pds(&did).await?;

        Ok(Self {
            did,
            pds,
            client: Client::new(),
        })
    }

    pub async fn fetch<T: DeserializeOwned>(
        &self,
        collection: &str,
    ) -> Result<ListRecords<T>, Error> {
        let url = format!(
            "{}/xrpc/com.atproto.repo.listRecords?repo={}&collection={}",
            self.pds, self.did, collection
        );

        let resp = self.client.get(&url).send().await?;
        let body = resp.json().await?;
        Ok(body)
    }

    pub async fn fetch_all<T: DeserializeOwned>(
        &self,
        collection: &str,
    ) -> Result<Vec<Record<T>>, Error> {
        let mut all_records = Vec::new();
        let mut cursor: Option<String> = None;

        loop {
            let mut url = format!(
                "{}/xrpc/com.atproto.repo.listRecords?repo={}&collection={}",
                self.pds, self.did, collection
            );
            if let Some(ref c) = cursor {
                url.push_str(&format!("&cursor={}", c));
            }

            let resp = self.client.get(&url).send().await?;
            let page: ListRecords<T> = resp.json().await?;

            all_records.extend(page.records);
            cursor = page.cursor;

            if cursor.is_none() {
                break;
            }
        }

        Ok(all_records)
    }
}
