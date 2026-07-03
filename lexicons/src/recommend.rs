use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Recommend {
    #[serde(rename = "$type")]
    pub r#type: Option<String>,
    pub document: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
