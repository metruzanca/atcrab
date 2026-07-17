use atcrab::lexicons::standard_site::Document;
use atcrab::Repo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repo::new("metru.dev").await?;
    println!("DID:  {}", repo.did);
    println!("PDS:  {}", repo.pds);

    let docs = repo.fetch_collection::<Document>().await?;

    println!("\nFound {} document(s):", docs.records.len());
    for record in &docs.records {
        println!(
            "  [{}] https://pdsls.dev/{}",
            record.value.title, record.uri
        );
    }

    Ok(())
}
