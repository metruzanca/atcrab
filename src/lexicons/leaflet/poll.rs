use serde::{Deserialize, Serialize};
use crate::lexicons::nsid::{Collection, PUB_LEAFLET_POLL_DEFINITION, PUB_LEAFLET_POLL_VOTE};
use crate::lexicons::types::StrongRef;

impl Collection for LeafletPollDefinition {
    const NSID: &'static str = PUB_LEAFLET_POLL_DEFINITION;
}

impl Collection for LeafletPollVote {
    const NSID: &'static str = PUB_LEAFLET_POLL_VOTE;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeafletPollDefinition {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub name: String,
    pub options: Vec<PollOption>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PollOption {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeafletPollVote {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub poll: StrongRef,
    pub option: Vec<String>,
}
