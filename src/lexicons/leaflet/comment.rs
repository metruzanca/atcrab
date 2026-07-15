use serde::{Deserialize, Serialize};
use crate::lexicons::nsid::{Collection, PUB_LEAFLET_COMMENT};

impl Collection for LeafletComment {
    const NSID: &'static str = PUB_LEAFLET_COMMENT;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeafletComment {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub subject: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub reply: Option<CommentReplyRef>,
    pub plaintext: String,
    pub facets: Option<Vec<serde_json::Value>>,
    #[serde(rename = "onPage")]
    pub on_page: Option<String>,
    pub attachment: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommentReplyRef {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub parent: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinearDocumentQuote {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub document: String,
    pub quote: serde_json::Value,
}
