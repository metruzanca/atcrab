use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::auth::{self, Session};
use crate::blob;
use crate::did;
use crate::error::Error;
use crate::handle;
use crate::lexicons::types::Blob;
use crate::lexicons::Collection;
use crate::types::{CreateRecordOutput, ListRecords, PutRecordOutput, Record};

pub struct Repo {
    pub did: String,
    pub pds: String,
    pub session: Option<Session>,
    password: Option<String>,
    client: Client,
}

impl Repo {
    pub async fn new(handle: &str) -> Result<Self, Error> {
        dotenvy::dotenv().ok();
        let did = handle::resolve_handle(handle).await?;
        let pds = did::resolve_pds(&did).await?;

        Ok(Self {
            did,
            pds,
            session: None,
            password: None,
            client: Client::new(),
        })
    }

    pub fn set_password(&mut self, password: &str) -> &mut Self {
        self.password = Some(password.to_string());
        self
    }

    pub async fn login(&mut self) -> Result<(), Error> {
        let password = match &self.password {
            Some(p) => p.clone(),
            None => std::env::var("ATP_PASSWORD")
                .map_err(|_| Error::Auth("ATP_PASSWORD not set; call set_password() or set ATP_PASSWORD in .env".into()))?,
        };
        let session = auth::create_session(&self.pds, &self.did, &password).await?;
        self.did = session.did.clone();
        self.session = Some(session);
        Ok(())
    }

    pub async fn create_record<T: Serialize>(
        &self,
        collection: &str,
        record: &T,
    ) -> Result<CreateRecordOutput, Error> {
        let session = self.session.as_ref().ok_or_else(|| Error::Auth("not logged in".into()))?;
        let url = format!("{}/xrpc/com.atproto.repo.createRecord", self.pds);
        let body = serde_json::json!({
            "repo": self.did,
            "collection": collection,
            "record": record,
        });

        let resp = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", session.access_jwt))
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(Error::Status(status, text));
        }

        Ok(resp.json().await?)
    }

    pub async fn put_record<T: Serialize>(
        &self,
        collection: &str,
        rkey: &str,
        record: &T,
        swap_record: Option<&str>,
    ) -> Result<PutRecordOutput, Error> {
        let session = self.session.as_ref().ok_or_else(|| Error::Auth("not logged in".into()))?;
        let url = format!("{}/xrpc/com.atproto.repo.putRecord", self.pds);

        let mut body = serde_json::json!({
            "repo": self.did,
            "collection": collection,
            "rkey": rkey,
            "record": record,
        });

        if let Some(cid) = swap_record {
            body["swapRecord"] = serde_json::json!(cid);
        }

        let resp = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", session.access_jwt))
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(Error::Status(status, text));
        }

        Ok(resp.json().await?)
    }

    pub async fn delete_record(&self, collection: &str, rkey: &str) -> Result<(), Error> {
        let session = self.session.as_ref().ok_or_else(|| Error::Auth("not logged in".into()))?;
        let url = format!("{}/xrpc/com.atproto.repo.deleteRecord", self.pds);
        let body = serde_json::json!({
            "repo": self.did,
            "collection": collection,
            "rkey": rkey,
        });

        let resp = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", session.access_jwt))
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(Error::Status(status, text));
        }

        Ok(())
    }

    pub async fn upload_blob(
        &self,
        data: Vec<u8>,
        mime_type: &str,
    ) -> Result<Blob, Error> {
        let session = self.session.as_ref().ok_or_else(|| Error::Auth("not logged in".into()))?;
        blob::upload_blob(&self.client, &self.pds, &session.access_jwt, data, mime_type).await
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
