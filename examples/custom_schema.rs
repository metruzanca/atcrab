use atcrab::{Collection, Repo};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Post {
    text: String,
    #[serde(rename = "createdAt")]
    created_at: String,
}

impl Collection for Post {
    const NSID: &'static str = "app.bsky.feed.post";
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repo::new("metru.dev").await?;

    let posts = repo.fetch_collection::<Post>().await?;

    println!("Found {} post(s):", posts.records.len());
    for record in &posts.records {
        println!("  {}  {}", record.value.created_at, record.value.text);
    }

    Ok(())
}
