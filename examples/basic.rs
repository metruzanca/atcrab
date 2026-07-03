use atcrab::{Repo, lexicons::Document};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repo::new("metru.dev").await?;
    println!("DID:  {}", repo.did);
    println!("PDS:  {}", repo.pds);

    let docs = repo.fetch::<Document>("site.standard.document").await?;

    println!("\nFound {} document(s):", docs.records.len());
    for record in &docs.records {
        println!("{:#?}", record);
        println!("  [{}] {}", record.value.title, record.uri);
    }

    Ok(())
}
