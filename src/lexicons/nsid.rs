use serde::de::DeserializeOwned;

pub trait Collection: DeserializeOwned {
    const NSID: &'static str;
}

pub const SITE_STANDARD_DOCUMENT: &str = "site.standard.document";
pub const SITE_STANDARD_PUBLICATION: &str = "site.standard.publication";
pub const SITE_STANDARD_GRAPH_SUBSCRIPTION: &str = "site.standard.graph.subscription";
pub const SITE_STANDARD_GRAPH_RECOMMEND: &str = "site.standard.graph.recommend";

pub const PUB_LEAFLET_DOCUMENT: &str = "pub.leaflet.document";
pub const PUB_LEAFLET_PUBLICATION: &str = "pub.leaflet.publication";
pub const PUB_LEAFLET_PUBLICATION_PAGE: &str = "pub.leaflet.publicationPage";
pub const PUB_LEAFLET_COMMENT: &str = "pub.leaflet.comment";
pub const PUB_LEAFLET_GRAPH_SUBSCRIPTION: &str = "pub.leaflet.graph.subscription";
pub const PUB_LEAFLET_INTERACTIONS_RECOMMEND: &str = "pub.leaflet.interactions.recommend";
pub const PUB_LEAFLET_POLL_DEFINITION: &str = "pub.leaflet.poll.definition";
pub const PUB_LEAFLET_POLL_VOTE: &str = "pub.leaflet.poll.vote";
