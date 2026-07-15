use serde::{Deserialize, Serialize};
use crate::lexicons::types::{Blob, StrongRef};

use super::facet::Facet;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "$type")]
pub enum Block {
    #[serde(rename = "pub.leaflet.blocks.text")]
    Text(TextBlock),
    #[serde(rename = "pub.leaflet.blocks.header")]
    Header(HeaderBlock),
    #[serde(rename = "pub.leaflet.blocks.image")]
    Image(ImageBlock),
    #[serde(rename = "pub.leaflet.blocks.imageGallery")]
    ImageGallery(ImageGalleryBlock),
    #[serde(rename = "pub.leaflet.blocks.blockquote")]
    Blockquote(BlockquoteBlock),
    #[serde(rename = "pub.leaflet.blocks.code")]
    Code(CodeBlock),
    #[serde(rename = "pub.leaflet.blocks.horizontalRule")]
    HorizontalRule(HorizontalRuleBlock),
    #[serde(rename = "pub.leaflet.blocks.math")]
    Math(MathBlock),
    #[serde(rename = "pub.leaflet.blocks.button")]
    Button(ButtonBlock),
    #[serde(rename = "pub.leaflet.blocks.iframe")]
    Iframe(IframeBlock),
    #[serde(rename = "pub.leaflet.blocks.website")]
    Website(WebsiteBlock),
    #[serde(rename = "pub.leaflet.blocks.bskyPost")]
    BskyPost(BskyPostBlock),
    #[serde(rename = "pub.leaflet.blocks.page")]
    Page(PageBlock),
    #[serde(rename = "pub.leaflet.blocks.poll")]
    Poll(PollBlock),
    #[serde(rename = "pub.leaflet.blocks.postsList")]
    PostsList(PostsListBlock),
    #[serde(rename = "pub.leaflet.blocks.signup")]
    Signup(SignupBlock),
    #[serde(rename = "pub.leaflet.blocks.membersOnlyDelimiter")]
    MembersOnlyDelimiter(MembersOnlyDelimiterBlock),
    #[serde(rename = "pub.leaflet.blocks.orderedList")]
    OrderedList(OrderedListBlock),
    #[serde(rename = "pub.leaflet.blocks.unorderedList")]
    UnorderedList(UnorderedListBlock),
    #[serde(rename = "pub.leaflet.blocks.standardSitePost")]
    StandardSitePost(StandardSitePostBlock),
    #[serde(rename = "pub.leaflet.blocks.standardSitePublication")]
    StandardSitePublication(StandardSitePublicationBlock),
    #[serde(rename = "pub.leaflet.blocks.leafletQuote")]
    LeafletQuote(LeafletQuoteBlock),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextBlock {
    pub plaintext: String,
    pub text_size: Option<String>,
    pub facets: Option<Vec<Facet>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HeaderBlock {
    pub plaintext: String,
    pub level: Option<u8>,
    pub facets: Option<Vec<Facet>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageBlock {
    pub image: Blob,
    pub alt: Option<String>,
    pub aspect_ratio: AspectRatio,
    pub full_bleed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageGalleryBlock {
    pub images: Vec<GalleryImageItem>,
    pub format: Option<String>,
    pub gap: Option<u32>,
    pub max_width: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GalleryImageItem {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub image: Blob,
    pub alt: Option<String>,
    pub aspect_ratio: AspectRatio,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockquoteBlock {
    pub plaintext: String,
    pub facets: Option<Vec<Facet>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CodeBlock {
    pub plaintext: String,
    pub language: Option<String>,
    pub syntax_highlighting_theme: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalRuleBlock {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MathBlock {
    pub tex: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ButtonBlock {
    pub text: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IframeBlock {
    pub url: String,
    pub height: Option<u16>,
    pub aspect_ratio: Option<AspectRatio>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebsiteBlock {
    pub src: String,
    pub preview_image: Option<Blob>,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BskyPostBlock {
    pub post_ref: StrongRef,
    pub client_host: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageBlock {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PollBlock {
    pub poll_ref: StrongRef,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostsListBlock {
    pub view: Option<String>,
    pub highlight_first_post: Option<bool>,
    pub filter_by_tags: Option<Vec<String>>,
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignupBlock {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MembersOnlyDelimiterBlock {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderedListBlock {
    pub start_index: Option<u32>,
    pub children: Vec<ListItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnorderedListBlock {
    pub children: Vec<ListItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListItem {
    pub checked: Option<bool>,
    pub content: Block,
    pub children: Option<Vec<ListItem>>,
    pub unordered_list_children: Option<Box<UnorderedListBlock>>,
    pub ordered_list_children: Option<Box<OrderedListBlock>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StandardSitePostBlock {
    pub uri: String,
    pub cid: Option<String>,
    pub size: Option<String>,
    pub show_publication_theme: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StandardSitePublicationBlock {
    pub uri: String,
    pub cid: Option<String>,
    pub show_publication_theme: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeafletQuoteBlock {
    pub record: StrongRef,
    pub position: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AspectRatio {
    pub width: u32,
    pub height: u32,
}
