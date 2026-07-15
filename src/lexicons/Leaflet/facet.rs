use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Facet {
    pub index: ByteSlice,
    pub features: Vec<FacetFeature>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ByteSlice {
    pub byte_start: u32,
    pub byte_end: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "$type")]
pub enum FacetFeature {
    #[serde(rename = "pub.leaflet.richtext.facet#link")]
    Link { uri: String },
    #[serde(rename = "pub.leaflet.richtext.facet#didMention")]
    DidMention { did: String },
    #[serde(rename = "pub.leaflet.richtext.facet#atMention")]
    AtMention { at_uri: String, href: Option<String> },
    #[serde(rename = "pub.leaflet.richtext.facet#code")]
    Code {},
    #[serde(rename = "pub.leaflet.richtext.facet#highlight")]
    Highlight { color: Option<serde_json::Value> },
    #[serde(rename = "pub.leaflet.richtext.facet#underline")]
    Underline {},
    #[serde(rename = "pub.leaflet.richtext.facet#strikethrough")]
    Strikethrough {},
    #[serde(rename = "pub.leaflet.richtext.facet#id")]
    Id { id: Option<String> },
    #[serde(rename = "pub.leaflet.richtext.facet#bold")]
    Bold {},
    #[serde(rename = "pub.leaflet.richtext.facet#italic")]
    Italic {},
    #[serde(rename = "pub.leaflet.richtext.facet#footnote")]
    Footnote {
        #[serde(rename = "footnoteId")]
        footnote_id: String,
        #[serde(rename = "contentPlaintext")]
        content_plaintext: String,
        #[serde(rename = "contentFacets")]
        content_facets: Option<Vec<Facet>>,
    },
}
