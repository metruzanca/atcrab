use serde::de::DeserializeOwned;

pub trait Collection: DeserializeOwned {
    const NSID: &'static str;
}

pub const SITE_STANDARD_DOCUMENT: &str = "site.standard.document";
pub const SITE_STANDARD_PUBLICATION: &str = "site.standard.publication";
pub const SITE_STANDARD_GRAPH_SUBSCRIPTION: &str = "site.standard.graph.subscription";
pub const SITE_STANDARD_GRAPH_RECOMMEND: &str = "site.standard.graph.recommend";
