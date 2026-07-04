use serde::{Deserialize, Serialize};
use crate::lexicons::nsid::Collection;
use crate::lexicons::nsid::SITE_STANDARD_GRAPH_SUBSCRIPTION;

impl Collection for Subscription {
    const NSID: &'static str = SITE_STANDARD_GRAPH_SUBSCRIPTION;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub publication: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
}
