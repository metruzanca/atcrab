use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
    #[serde(rename = "$type")]
    pub r#type: String,
    #[serde(rename = "ref")]
    pub blob_ref: BlobLink,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlobLink {
    #[serde(rename = "$link")]
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StrongRef {
    #[serde(rename = "$type")]
    pub r#type: Option<String>,
    pub uri: String,
    pub cid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "$type")]
pub enum SelfLabel {
    #[serde(rename = "com.atproto.label.defs#selfLabel")]
    Label {
        val: String,
    },
}
