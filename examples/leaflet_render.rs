use atcrab::lexicons::standard_site::Content;
use atcrab::lexicons::standard_site::Document;
use atcrab::lexicons::standard_site::Publication;
use atcrab::lexicons::leaflet_pub;
use atcrab::Repo;

const PUB_URI: &str =
    "at://did:plc:msockseqtdlbyntubnhntlmo/site.standard.publication/3mpr6p5us2227";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repo::new("metru.dev").await?;

    let publications = repo.fetch_all::<Publication>().await?;
    let Some(pub_record) = publications.iter().find(|r| r.uri == PUB_URI) else {
        println!("Publication not found: {PUB_URI}");
        return Ok(());
    };
    println!("# {}", pub_record.value.name);
    if let Some(desc) = &pub_record.value.description {
        if !desc.is_empty() {
            println!("> {desc}");
        }
    }
    println!();

    let docs = repo.fetch_all::<Document>().await?;
    let posts: Vec<_> = docs.iter().filter(|r| r.value.site == PUB_URI).collect();

    if posts.is_empty() {
        println!("(no posts)");
        return Ok(());
    }

    for doc_record in &posts {
        let doc = &doc_record.value;
        println!("## {}", doc.title);
        let date = doc.published_at.split('T').next().unwrap_or(&doc.published_at);
        println!("_{date}_");
        if let Some(desc) = &doc.description {
            if !desc.is_empty() {
                println!();
                println!("> {desc}");
            }
        }
        println!();

        if let Some(content) = &doc.content {
            render_content(content);
        }

        if let Some(text) = &doc.text_content {
            if !text.is_empty() {
                println!("{}", text);
            }
        }

        println!("---\n");
    }

    Ok(())
}

fn render_content(content: &Content) {
    let Content::Leaflet(lc) = content;

    for page in &lc.pages {
        match page {
            leaflet_pub::Page::LinearDocument(ld) => {
                for block in &ld.blocks {
                    render_block(&block.block);
                }
            }
            leaflet_pub::Page::Canvas(c) => {
                for block in &c.blocks {
                    render_block(&block.block);
                }
            }
        }
    }
}

fn render_block(block: &leaflet_pub::Block) {
    match block {
        leaflet_pub::Block::Header(h) => {
            let prefix = match h.level.unwrap_or(1) {
                1 => "#",
                2 => "##",
                3 => "###",
                _ => "####",
            };
            let text = render_facets(&h.plaintext, h.facets.as_deref());
            println!("{prefix} {text}");
            println!();
        }
        leaflet_pub::Block::Text(t) => {
            if !t.plaintext.trim().is_empty() {
                let text = render_facets(&t.plaintext, t.facets.as_deref());
                println!("{text}");
                println!();
            }
        }
        leaflet_pub::Block::Blockquote(bq) => {
            let text = render_facets(&bq.plaintext, bq.facets.as_deref());
            for line in text.lines() {
                println!("> {line}");
            }
            println!();
        }
        leaflet_pub::Block::Image(img) => {
            println!("[image: {}x{}]", img.aspect_ratio.width, img.aspect_ratio.height);
            println!();
        }
        leaflet_pub::Block::HorizontalRule(_) => {
            println!("---");
            println!();
        }
        leaflet_pub::Block::UnorderedList(ul) => {
            for item in &ul.children {
                let text = list_item_text(&item.content);
                println!("  - {text}");
            }
            println!();
        }
        leaflet_pub::Block::OrderedList(ol) => {
            for (i, item) in ol.children.iter().enumerate() {
                let start = ol.start_index.unwrap_or(1) as usize;
                let text = list_item_text(&item.content);
                println!("  {}. {text}", start + i);
            }
            println!();
        }
        leaflet_pub::Block::Code(c) => {
            println!("```{}", c.language.as_deref().unwrap_or(""));
            println!("{}", c.plaintext);
            println!("```");
            println!();
        }
        _ => {}
    }
}

fn list_item_text(block: &leaflet_pub::Block) -> String {
    match block {
        leaflet_pub::Block::Text(t) => render_facets(&t.plaintext, t.facets.as_deref()),
        leaflet_pub::Block::Header(h) => render_facets(&h.plaintext, h.facets.as_deref()),
        _ => String::new(),
    }
}

fn render_facets(plaintext: &str, facets: Option<&[leaflet_pub::Facet]>) -> String {
    let Some(facets) = facets else {
        return plaintext.to_string();
    };
    if facets.is_empty() {
        return plaintext.to_string();
    }

    let mut sorted: Vec<_> = facets.iter().collect();
    sorted.sort_by_key(|f| f.index.byte_start);

    let mut output = String::new();
    let mut last_end = 0usize;
    let text_bytes = plaintext.as_bytes();

    for facet in &sorted {
        let start = facet.index.byte_start as usize;
        let end = facet.index.byte_end as usize;

        if start > last_end && start < text_bytes.len() {
            output.push_str(&plaintext[last_end..start]);
        }
        if end <= text_bytes.len() && start < end {
            let segment = &plaintext[start..end];
            output.push_str(segment);
        }
        last_end = end;
    }

    if last_end < text_bytes.len() {
        output.push_str(&plaintext[last_end..]);
    }

    output
}
