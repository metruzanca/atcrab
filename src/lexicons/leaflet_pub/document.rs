use serde::{Deserialize, Serialize};
use crate::lexicons::nsid::{Collection, PUB_LEAFLET_DOCUMENT};
use crate::lexicons::types::{Blob, StrongRef};

impl Collection for Document {
    const NSID: &'static str = PUB_LEAFLET_DOCUMENT;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub title: String,
    #[serde(rename = "postRef")]
    pub post_ref: Option<StrongRef>,
    pub description: Option<String>,
    #[serde(rename = "publishedAt")]
    pub published_at: Option<String>,
    pub publication: Option<String>,
    pub author: String,
    pub theme: Option<serde_json::Value>,
    pub preferences: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<Blob>,
    pub pages: Vec<serde_json::Value>,
}
