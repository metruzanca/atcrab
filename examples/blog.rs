use atcrab::lexicons::standard::{Document, Publication};
use atcrab::Repo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repo::new("metru.dev").await?;

    let publications = repo.fetch_all::<Publication>().await?;
    let sams_blog = publications
        .iter()
        .find(|r| r.value.name == "Sam's Blog");

    let Some(blog) = sams_blog else {
        println!("No publication named \"Sam's Blog\" found.");
        return Ok(());
    };
    println!("[{}] https://pdsls.dev/{}", blog.value.name, blog.uri);

    let docs = repo.fetch_all::<Document>().await?;
    for record in docs.iter().filter(|r| r.value.site == blog.uri) {
        println!("- {} https://pdsls.dev/{}", record.value.title, record.uri);
    }

    Ok(())
}
