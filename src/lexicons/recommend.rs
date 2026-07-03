use serde::{Deserialize, Serialize};
use crate::lexicons::nsid::Collection;
use crate::lexicons::nsid::SITE_STANDARD_GRAPH_RECOMMEND;

impl Collection for Recommend {
    const NSID: &'static str = SITE_STANDARD_GRAPH_RECOMMEND;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Recommend {
    #[serde(rename = "$type")]
    pub r#type: Option<String>,
    pub document: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
