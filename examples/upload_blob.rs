use atcrab::Repo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut repo = Repo::new("metru.dev").await?;
    repo.login().await?;

    let blob = repo
        .upload_blob(b"hello world".to_vec(), "text/plain")
        .await?;
    println!("Uploaded blob: {}", blob.blob_ref.link);
    println!("  mimeType: {}", blob.mime_type);
    println!("  size: {}", blob.size);

    // Unreferenced blobs are garbage collected after a time span. No worries here.
    Ok(())
}
