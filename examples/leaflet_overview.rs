use atcrab::lexicons::standard::Content;
use atcrab::lexicons::standard::Document;
use atcrab::lexicons::standard::Publication;
use atcrab::lexicons::Leaflet;
use atcrab::Repo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repo::new("metru.dev").await?;

    let publications = repo.fetch_all_collection::<Publication>().await?;
    let docs = repo.fetch_all_collection::<Document>().await?;

    for pub_record in &publications {
        let p = &pub_record.value;
        println!("[{}] {}", p.name, pub_record.uri);

        let posts: Vec<_> = docs
            .iter()
            .filter(|r| r.value.site == pub_record.uri)
            .collect();

        if posts.is_empty() {
            println!("  (no posts)");
        } else {
            for doc_record in &posts {
                let d = &doc_record.value;
                let preview = render_preview(d);
                println!("  - {} — {}", d.title, preview);
            }
        }
        println!();
    }

    Ok(())
}

fn render_preview(doc: &Document) -> String {
    let Some(content) = &doc.content else {
        return doc.text_content.clone().unwrap_or_default();
    };
    let Content::Leaflet(lc) = content;
    let Some(page) = lc.pages.first() else {
        return String::new();
    };
    let Leaflet::Page::LinearDocument(ld) = page else {
        return String::new();
    };
    let Some(first) = ld.blocks.first() else {
        return String::new();
    };
    let Leaflet::Block::Text(t) = &first.block else {
        return String::new();
    };
    let text = t.plaintext.trim();
    if text.len() > 80 {
        format!("{}...", &text[..77])
    } else {
        text.to_string()
    }
}
