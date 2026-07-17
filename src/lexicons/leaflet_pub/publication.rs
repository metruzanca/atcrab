use serde::{Deserialize, Serialize};
use crate::lexicons::nsid::{Collection, PUB_LEAFLET_PUBLICATION};
use crate::lexicons::types::Blob;

impl Collection for Publication {
    const NSID: &'static str = PUB_LEAFLET_PUBLICATION;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Publication {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub name: String,
    #[serde(rename = "base_path")]
    pub base_path: Option<String>,
    pub description: Option<String>,
    pub icon: Option<Blob>,
    pub theme: Option<PublicationTheme>,
    pub preferences: Option<Preferences>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
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
pub struct PublicationTheme {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub background_color: Option<ThemeColor>,
    pub background_image: Option<serde_json::Value>,
    pub wordmark: Option<serde_json::Value>,
    pub page_width: Option<u16>,
    pub primary: Option<ThemeColor>,
    pub page_background: Option<ThemeColor>,
    #[serde(default)]
    pub show_page_background: bool,
    pub accent_background: Option<ThemeColor>,
    pub accent_text: Option<ThemeColor>,
    pub heading_font: Option<String>,
    pub body_font: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ThemeColor {
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
