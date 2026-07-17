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

    let page = repo.fetch::<Post>().await?;
    println!("fetch()  → {} records on first page", page.records.len());
    for p in &page.records {
        println!("                        {}  {}", p.value.created_at, p.value.text);
    }
    if let Some(cursor) = &page.cursor {
        println!("                        cursor available: {cursor}");
        let page2 = repo.fetch_cursor::<Post>(cursor).await?;
        println!(
            "fetch_cursor() → {} records on second page",
            page2.records.len()
        );
    }

    let all = repo.fetch_all::<Post>().await?;
    println!(
        "fetch_all() → {} total records across all pages",
        all.len()
    );

    Ok(())
}
