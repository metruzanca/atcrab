use serde::{Deserialize, Serialize};
use crate::lexicons::theme::BasicTheme;
use crate::lexicons::types::{Blob, SelfLabel};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    #[serde(rename = "$type")]
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
    #[serde(rename = "$type")]
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
