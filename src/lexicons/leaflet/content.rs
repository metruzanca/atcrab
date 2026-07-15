use serde::{Deserialize, Serialize};
use crate::lexicons::types::Blob;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeafletContent {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub pages: Vec<serde_json::Value>,
    #[serde(rename = "blobPages")]
    pub blob_pages: Option<Blob>,
    pub blobs: Option<Vec<Blob>>,
}
