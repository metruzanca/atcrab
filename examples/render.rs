use atcrab::lexicons::standard::{Content, Document};
use atcrab::lexicons::Leaflet;
use atcrab::Repo;

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
        println!("uri: https://pdsls.dev/{}", record.uri);
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

fn render_content(content: &Content, depth: usize) {
    match content {
        Content::Leaflet(lc) => {
            for page in &lc.pages {
                match page {
                    Leaflet::Page::LinearDocument(ld) => {
                        println!();
                        for lb in &ld.blocks {
                            render_block(&lb.block, depth);
                        }
                    }
                    Leaflet::Page::Canvas(c) => {
                        println!();
                        for cb in &c.blocks {
                            render_block(&cb.block, depth);
                        }
                    }
                }
            }
        }
    }
}

fn render_block(block: &Leaflet::Block, depth: usize) {
    let indent = "  ".repeat(depth + 1);

    match block {
        Leaflet::Block::Header(h) => {
            let prefix = match h.level.unwrap_or(1) {
                1 => "##",
                2 => "###",
                _ => "####",
            };
            let text = render_facets(&h.plaintext, h.facets.as_deref());
            println!();
            println!("{indent}{prefix} {text}");
        }
        Leaflet::Block::Text(t) => {
            if !t.plaintext.is_empty() {
                println!("{indent}{}", t.plaintext);
            } else {
                println!();
            }
        }
        Leaflet::Block::Page(p) => {
            println!("{indent}(sub-page: {})", p.id);
        }
        Leaflet::Block::Image(img) => {
            println!(
                "{indent}[image: {}x{}]",
                img.aspect_ratio.width, img.aspect_ratio.height
            );
        }
        Leaflet::Block::Blockquote(bq) => {
            let text = render_facets(&bq.plaintext, bq.facets.as_deref());
            println!("{indent}> {}", text);
        }
        Leaflet::Block::HorizontalRule(_) => {
            println!("{indent}{}", "─".repeat(40));
        }
        Leaflet::Block::UnorderedList(ul) => {
            for item in &ul.children {
                let text = match &item.content {
                    Leaflet::Block::Text(t) => {
                        render_facets(&t.plaintext, t.facets.as_deref())
                    }
                    Leaflet::Block::Header(h) => {
                        render_facets(&h.plaintext, h.facets.as_deref())
                    }
                    _ => String::new(),
                };
                println!("{indent}  - {}", text);
            }
        }
        Leaflet::Block::OrderedList(ol) => {
            for (idx, item) in ol.children.iter().enumerate() {
                let text = match &item.content {
                    Leaflet::Block::Text(t) => {
                        render_facets(&t.plaintext, t.facets.as_deref())
                    }
                    Leaflet::Block::Header(h) => {
                        render_facets(&h.plaintext, h.facets.as_deref())
                    }
                    _ => String::new(),
                };
                let start = ol.start_index.unwrap_or(1) as usize;
                println!("{indent}  {}. {}", start + idx, text);
            }
        }
        _ => {
            // fallback — try showing a text or name field
        }
    }
}

fn render_facets(plaintext: &str, facets: Option<&[Leaflet::Facet]>) -> String {
    let Some(facets) = facets else {
        return plaintext.to_string();
    };
    if facets.is_empty() {
        return plaintext.to_string();
    }

    let text_bytes = plaintext.as_bytes();
    let mut output = String::new();
    let mut last_end: usize = 0;

    let mut sorted: Vec<_> = facets.iter().collect();
    sorted.sort_by_key(|f| f.index.byte_start);

    for facet in &sorted {
        let start = facet.index.byte_start as usize;
        let end = facet.index.byte_end as usize;

        let has_link = facet.features.iter().any(|f| matches!(f, Leaflet::FacetFeature::Link { .. }));

        if start > last_end && start < text_bytes.len() {
            output.push_str(&plaintext[last_end..start]);
        }

        if end <= text_bytes.len() && start < end {
            let segment = &plaintext[start..end];
            if has_link {
                for feature in &facet.features {
                    if let Leaflet::FacetFeature::Link { uri } = feature {
                        output.push_str(&format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", uri, segment));
                    }
                }
            } else {
                output.push_str(segment);
            }
        }

        last_end = end;
    }

    if last_end < text_bytes.len() {
        output.push_str(&plaintext[last_end..]);
    }

    output
}
