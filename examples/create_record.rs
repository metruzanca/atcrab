use atcrab::{Collection, CreateRecordOutput, Repo};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Note {
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    text: String,
    #[serde(rename = "createdAt")]
    created_at: String,
}

impl Collection for Note {
    const NSID: &'static str = "net.example.test";
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut repo = Repo::new("metru.dev").await?;
    repo.login().await?;

    let note = Note {
        r#type: Some(Note::NSID.into()),
        text: "hello from atcrab".into(),
        created_at: "2025-01-01T00:00:00.000Z".into(),
    };

    let created: CreateRecordOutput = repo.create_record(Note::NSID, &note).await?;
    println!("Created: {}", created.uri);

    let record = Note {
        r#type: Some(Note::NSID.into()),
        text: "updated!".into(),
        created_at: "2025-01-01T00:00:00.000Z".into(),
    };

    let rkey = created.uri.rsplit('/').next().unwrap();
    let updated = repo.put_record(Note::NSID, rkey, &record, None).await?;
    println!("Updated: {}", updated.uri);

    repo.delete_record(Note::NSID, rkey).await?;
    println!("Deleted: {}", rkey);

    Ok(())
}
