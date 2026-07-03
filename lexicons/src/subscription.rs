use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    #[serde(rename = "$type")]
    pub r#type: Option<String>,
    pub publication: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
}
