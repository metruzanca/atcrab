use serde::{Deserialize, Serialize};
use super::blocks::Block;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "$type")]
pub enum Page {
    #[serde(rename = "pub.leaflet.pages.linearDocument")]
    LinearDocument(LinearDocument),
    #[serde(rename = "pub.leaflet.pages.canvas")]
    Canvas(Canvas),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinearDocument {
    pub id: Option<String>,
    pub blocks: Vec<LinearBlock>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinearBlock {
    pub block: Block,
    pub alignment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Canvas {
    pub id: Option<String>,
    pub blocks: Vec<CanvasBlock>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CanvasBlock {
    pub block: Block,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: Option<u32>,
    pub rotation: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub block: Vec<u32>,
    pub offset: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub start: Position,
    pub end: Position,
}
