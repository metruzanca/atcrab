use serde::{Deserialize, Serialize};
use crate::lexicons::types::{Blob, SelfLabel, StrongRef};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
    #[serde(rename = "$type")]
    pub r#type: Option<String>,
    pub did: String,
    pub role: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[serde(rename = "$type")]
    pub r#type: Option<String>,
    pub site: String,
    pub path: Option<String>,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<Blob>,
    pub content: Option<serde_json::Value>,
    #[serde(rename = "textContent")]
    pub text_content: Option<String>,
    #[serde(rename = "bskyPostRef")]
    pub bsky_post_ref: Option<StrongRef>,
    pub tags: Option<Vec<String>>,
    pub labels: Option<Vec<SelfLabel>>,
    pub links: Option<serde_json::Value>,
    pub contributors: Option<Vec<Contributor>>,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}
