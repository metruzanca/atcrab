use serde::{Deserialize, Serialize};
use crate::lexicons::nsid::{Collection, PUB_LEAFLET_PUBLICATION_PAGE};
use crate::lexicons::leaflet_pub::content::Content;

impl Collection for PublicationPage {
    const NSID: &'static str = PUB_LEAFLET_PUBLICATION_PAGE;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicationPage {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub publication: String,
    pub path: String,
    pub title: Option<String>,
    #[serde(rename = "publishedAt")]
    pub published_at: Option<String>,
    pub content: Content,
}
