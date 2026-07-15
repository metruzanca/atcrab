use serde::{Deserialize, Serialize};
use crate::lexicons::nsid::Collection;
use crate::lexicons::nsid::SITE_STANDARD_PUBLICATION;
use crate::lexicons::standard::theme::BasicTheme;
use crate::lexicons::types::{Blob, SelfLabel};

impl Collection for Publication {
    const NSID: &'static str = SITE_STANDARD_PUBLICATION;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "showInDiscover")]
    #[serde(default = "default_true")]
    pub show_in_discover: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Publication {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub url: String,
    pub icon: Option<Blob>,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "basicTheme")]
    pub basic_theme: Option<BasicTheme>,
    pub preferences: Option<Preferences>,
    pub labels: Option<Vec<SelfLabel>>,
}
