use serde::{Deserialize, Serialize};
use crate::lexicons::nsid::{Collection, PUB_LEAFLET_PUBLICATION};
use crate::lexicons::types::Blob;

impl Collection for LeafletPublication {
    const NSID: &'static str = PUB_LEAFLET_PUBLICATION;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeafletPublication {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub name: String,
    #[serde(rename = "base_path")]
    pub base_path: Option<String>,
    pub description: Option<String>,
    pub icon: Option<Blob>,
    pub theme: Option<LeafletPublicationTheme>,
    pub preferences: Option<LeafletPreferences>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeafletPreferences {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default = "default_true")]
    pub show_in_discover: bool,
    #[serde(default = "default_true")]
    pub show_comments: bool,
    #[serde(default = "default_true")]
    pub show_mentions: bool,
    #[serde(default = "default_true")]
    pub show_prev_next: bool,
    #[serde(default = "default_false")]
    pub show_first_last: bool,
    #[serde(default = "default_true")]
    pub show_recommends: bool,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeafletPublicationTheme {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub background_color: Option<LeafletThemeColor>,
    pub background_image: Option<serde_json::Value>,
    pub wordmark: Option<serde_json::Value>,
    pub page_width: Option<u16>,
    pub primary: Option<LeafletThemeColor>,
    pub page_background: Option<LeafletThemeColor>,
    #[serde(default)]
    pub show_page_background: bool,
    pub accent_background: Option<LeafletThemeColor>,
    pub accent_text: Option<LeafletThemeColor>,
    pub heading_font: Option<String>,
    pub body_font: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum LeafletThemeColor {
    Rgba {
        #[serde(rename = "$type")]
        r#type: String,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    },
    Rgb {
        #[serde(rename = "$type")]
        r#type: String,
        r: u8,
        g: u8,
        b: u8,
    },
}
