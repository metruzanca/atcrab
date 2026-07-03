use atcrab::Repo;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Post {
    text: String,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repo::new("metru.dev").await?;
    let collection = "app.bsky.feed.post";

    let page = repo.fetch::<Post>(collection).await?;
    println!("fetch()  → {} records on first page", page.records.len());
    for p in &page.records {
        println!("           {}  {}", p.value.created_at, p.value.text);
    }
    if let Some(cursor) = &page.cursor {
        println!("           cursor available: {cursor}");
    }

    let all = repo.fetch_all::<Post>(collection).await?;
    println!(
        "fetch_all() → {} total records across all pages",
        all.len()
    );

    Ok(())
}
