use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::did;
use crate::error::Error;
use crate::handle;
use crate::lexicons::Collection;
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

    fn list_records_url(&self, collection: &str, cursor: Option<&str>) -> String {
        let mut url = format!(
            "{}/xrpc/com.atproto.repo.listRecords?repo={}&collection={}",
            self.pds, self.did, collection
        );
        if let Some(c) = cursor {
            url.push_str("&cursor=");
            url.push_str(c);
        }
        url
    }

    pub async fn fetch<T: DeserializeOwned>(
        &self,
        collection: &str,
    ) -> Result<ListRecords<T>, Error> {
        let url = self.list_records_url(collection, None);
        let resp = self.client.get(&url).send().await?;
        let body = resp.json().await?;
        Ok(body)
    }

    pub async fn fetch_cursor<T: DeserializeOwned>(
        &self,
        collection: &str,
        cursor: &str,
    ) -> Result<ListRecords<T>, Error> {
        let url = self.list_records_url(collection, Some(cursor));
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
            let url = self.list_records_url(collection, cursor.as_deref());
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

    pub async fn fetch_collection<T: Collection>(
        &self,
    ) -> Result<ListRecords<T>, Error> {
        self.fetch::<T>(T::NSID).await
    }

    pub async fn fetch_collection_cursor<T: Collection>(
        &self,
        cursor: &str,
    ) -> Result<ListRecords<T>, Error> {
        self.fetch_cursor::<T>(T::NSID, cursor).await
    }

    pub async fn fetch_all_collection<T: Collection>(
        &self,
    ) -> Result<Vec<Record<T>>, Error> {
        self.fetch_all::<T>(T::NSID).await
    }
}
