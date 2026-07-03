use atcrab::lexicons::Document;
use atcrab::Repo;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repo::new("metru.dev").await?;

    let docs = repo.fetch_all_collection::<Document>().await?;

    for (i, record) in docs.iter().enumerate() {
        let doc = &record.value;

        println!("{}", "─".repeat(70));

        // title
        println!("# {}", doc.title);

        // date
        let date = doc
            .published_at
            .split('T')
            .next()
            .unwrap_or(&doc.published_at);
        println!("{}", date);

        // tags
        if let Some(tags) = &doc.tags {
            if !tags.is_empty() {
                print!("tags:");
                for tag in tags {
                    print!(" #{tag}");
                }
                println!();
            }
        }

        // description
        if let Some(desc) = &doc.description {
            if !desc.is_empty() {
                println!();
                println!("> {}", desc);
            }
        }

        // content blocks
        if let Some(content) = &doc.content {
            render_content(content, 0);
        }

        // plaintext fallback
        if let Some(text) = &doc.text_content {
            if !text.is_empty() {
                println!();
                for line in text.lines() {
                    println!("  {}", line);
                }
            }
        }

        println!();
        println!("uri: {}", record.uri);
        if i < docs.len() - 1 {
            println!();
        }
    }

    if docs.is_empty() {
        println!("No documents found.");
    } else {
        println!("{}", "─".repeat(70));
        println!("{} document(s)", docs.len());
    }

    Ok(())
}

fn render_content(value: &Value, depth: usize) {
    let pages = value.get("pages").and_then(|p| p.as_array());

    match pages {
        Some(pages) => {
            for page in pages {
                let blocks = page.get("blocks").and_then(|b| b.as_array());

                if let Some(blocks) = blocks {
                    println!();
                    for block_value in blocks {
                        let block = block_value.get("block");
                        if let Some(block) = block {
                            render_block(block, depth);
                        }
                    }
                }
            }
        }
        None => {
            // Try rendering as a single block directly
            if value.get("$type").and_then(|t| t.as_str()).is_some() {
                render_block(value, depth);
                return;
            }
            // Fallback: unknown content format
            println!();
            println!("  (content type not recognized)");
        }
    }
}

fn render_block(block: &Value, depth: usize) {
    let indent = "  ".repeat(depth + 1);

    let block_type = block
        .get("$type")
        .and_then(|t| t.as_str())
        .unwrap_or("");

    match block_type {
        t if t.ends_with(".blocks.header") => {
            let plaintext = block
                .get("plaintext")
                .and_then(|t| t.as_str())
                .unwrap_or("");
            let level = block.get("level").and_then(|l| l.as_u64()).unwrap_or(1);

            let prefix = match level {
                1 => "##",
                2 => "###",
                _ => "####",
            };

            let facets = block.get("facets").and_then(|f| f.as_array());
            if let Some(facets) = facets {
                let text = apply_facets(plaintext, facets);
                println!();
                println!("{indent}{prefix} {text}");
            } else {
                println!();
                println!("{indent}{prefix} {plaintext}");
            }
        }
        t if t.ends_with(".blocks.text") => {
            let plaintext = block
                .get("plaintext")
                .and_then(|t| t.as_str())
                .unwrap_or("");
            if !plaintext.is_empty() {
                println!("{indent}{}", plaintext);
            } else {
                println!();
            }
        }
        t if t.ends_with(".blocks.page") => {
            let id = block.get("id").and_then(|i| i.as_str()).unwrap_or("");
            println!("{indent}(sub-page: {id})");
        }
        _ => {
            // Unknown block type - try plaintext
            if let Some(plaintext) = block.get("plaintext").and_then(|t| t.as_str()) {
                println!("{indent}{}", plaintext);
            }
        }
    }
}

fn apply_facets(text: &str, facets: &[Value]) -> String {
    let mut output = String::new();
    let mut last_end: usize = 0;

    // Sort facets by byte start position
    let mut facets: Vec<_> = facets.iter().collect();
    facets.sort_by_key(|f| {
        f.get("index")
            .and_then(|i| i.get("byteStart"))
            .and_then(|b| b.as_u64())
            .unwrap_or(0)
    });

    for facet in &facets {
        let index = facet.get("index");
        let start = index
            .and_then(|i| i.get("byteStart"))
            .and_then(|b| b.as_u64())
            .map(|n| n as usize)
            .unwrap_or(0);
        let end = index
            .and_then(|i| i.get("byteEnd"))
            .and_then(|b| b.as_u64())
            .map(|n| n as usize)
            .unwrap_or(0);

        if let Some(features) = facet.get("features").and_then(|f| f.as_array()) {
            for feature in features {
                let link = feature.get("uri").and_then(|u| u.as_str());

                // Add text before this facet
                if start > last_end && start < text.len() {
                    output.push_str(&text[last_end..start]);
                }

                // Add the linked segment
                if end <= text.len() && start < end {
                    let segment = &text[start..end];
                    if let Some(uri) = link {
                        output.push_str(&format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", uri, segment));
                    } else {
                        output.push_str(segment);
                    }
                }

                last_end = end;
            }
        }
    }

    // Add remaining text
    if last_end < text.len() {
        output.push_str(&text[last_end..]);
    }

    output
}
