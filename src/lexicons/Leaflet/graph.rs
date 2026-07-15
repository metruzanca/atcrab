use serde::{Deserialize, Serialize};
use crate::lexicons::nsid::{
    Collection, PUB_LEAFLET_GRAPH_SUBSCRIPTION, PUB_LEAFLET_INTERACTIONS_RECOMMEND,
};

impl Collection for Subscription {
    const NSID: &'static str = PUB_LEAFLET_GRAPH_SUBSCRIPTION;
}

impl Collection for Recommend {
    const NSID: &'static str = PUB_LEAFLET_INTERACTIONS_RECOMMEND;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub publication: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Recommend {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub subject: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
