# AT Crab

Rust AT Protocol libraries.

> **Status:** This is a personal project hacked together to scratch an itch. It will remain v0.x until the API is stable, test coverage is high, and I'm confident others can depend on it. At that point I plan to publish to [crates.io](https://crates.io). AI agents contributed to the implementation, but I've steered the API design toward idiomatic Rust and a human-ergonomic developer experience.
>
> Do not use this in production. See https://atproto.com/sdks for alternatives.

## Features

- **Handle & DID resolution** - resolve any handle to a DID via DNS (`_atproto`) or `.well-known`, then resolve the PDS endpoint from PLC or `did:web` documents
- **Read records** - fetch records from any collection with built-in pagination and automatic cursor handling
- **Write records** - create, update, and delete records in your own PDS repository
  - **Auth** - login with app passwords, auto-loaded from `.env` or environment variables (`ATP_PASSWORD`)
- **Blob upload** - upload images and other binary data to your PDS
- **Built-in lexicon types** - ready-to-use types for [standard.site](https://standard.site) and [Leaflet](https://leaflet.pub) lexicons, with full typed content blocks

## Adding as a dependency

```toml
[dependencies]
atcrab = { git = "https://github.com/metruzanca/atcrab" }
```

## Usage

### Fetch with built-in types (standard.site)

Built-in lexicon types implement `Collection` so the NSID is inferred:

```rust
use atcrab::lexicons::standard_site;
use atcrab::Repo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repo::new("metru.dev").await?;
    let docs = repo.fetch::<standard_site::Document>().await?;
    for record in &docs.records {
        println!("{} ({})", record.value.title, record.uri);
    }
    Ok(())
}
```

**Included standard.site types:**

| Lexicon | Type |
|---|---|
| `site.standard.document` | `standard_site::Document`, `standard_site::Content`, `standard_site::Contributor` |
| `site.standard.publication` | `standard_site::Publication`, `standard_site::Preferences` |
| `site.standard.graph.subscription` | `standard_site::Subscription` |
| `site.standard.graph.recommend` | `standard_site::Recommend` |
| `site.standard.theme.basic` | `standard_site::BasicTheme` |
| `site.standard.theme.color` | `standard_site::Rgb`, `standard_site::Rgba` |

`Document.content` is a tagged `Content` enum — when the document was published via Leaflet, the `Content::Leaflet(LeafletContent)` variant provides fully typed pages and blocks.

### Fetch with built-in types (Leaflet)

```rust
use atcrab::lexicons::leaflet_pub;
use atcrab::Repo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repo::new("metru.dev").await?;
    let docs = repo.fetch::<leaflet_pub::Document>().await?;
    for record in &docs.records {
        println!("{}", record.value.title);
    }
    Ok(())
}
```

**Included Leaflet types:**

| Lexicon | Type |
|---|---|
| `pub.leaflet.document` | `leaflet_pub::Document` |
| `pub.leaflet.publication` | `leaflet_pub::Publication`, `leaflet_pub::Preferences`, `leaflet_pub::PublicationTheme` |
| `pub.leaflet.publicationPage` | `leaflet_pub::PublicationPage` |
| `pub.leaflet.comment` | `leaflet_pub::Comment` |
| `pub.leaflet.graph.subscription` | `leaflet_pub::Subscription` |
| `pub.leaflet.interactions.recommend` | `leaflet_pub::Recommend` |
| `pub.leaflet.poll.definition` | `leaflet_pub::PollDefinition` |
| `pub.leaflet.poll.vote` | `leaflet_pub::PollVote` |

**Content blocks** — `leaflet_pub::Content` contains `pages: Vec<leaflet_pub::Page>` (`LinearDocument` / `Canvas`), each with `blocks` of `leaflet_pub::Block` (22 typed variants including `Text`, `Header`, `Image`, `Blockquote`, `UnorderedList`, etc.). Rich text facets are typed as `leaflet_pub::Facet` with `leaflet_pub::FacetFeature` (11 feature variants: `Link`, `Bold`, `Italic`, `Underline`, `Code`, etc.).

**Shared ATProto types:** `Blob`, `BlobLink`, `StrongRef`, `SelfLabel`

### Fetch with custom types

Implement `Collection` on your own types for the same inferred API:

```rust
use atcrab::{Collection, Repo};
use serde::Deserialize;

#[derive(Deserialize)]
struct Post { text: String }

impl Collection for Post {
    const NSID: &'static str = "app.bsky.feed.post";
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repo::new("metru.dev").await?;
    let posts = repo.fetch::<Post>().await?;
    println!("{} posts", posts.records.len());
    Ok(())
}
```

### Explicit collection

If you don't want to implement `Collection`, pass the NSID directly:

```rust
// first page
let posts = repo.fetch_collection::<Post>("app.bsky.feed.post").await?;
// next page from cursor
let posts = repo.fetch_collection_cursor::<Post>("app.bsky.feed.post", cursor).await?;
// all pages
let all   = repo.fetch_all_collection::<Post>("app.bsky.feed.post").await?;
```

## Examples

```sh
cargo run --example basic               # fetch standard.site documents with built-in types
cargo run --example custom_schema       # fetch posts with a user-defined type
cargo run --example pagination          # cursor-based pagination
cargo run --example render              # pretty-print leaflet content blocks with facets
cargo run --example blog                # join publications with their documents
cargo run --example leaflet_overview    # list all publications and their post previews
cargo run --example leaflet_render      # render a specific publication's posts in plaintext
cargo run --example create_record       # create, update, and delete a record
cargo run --example upload_blob         # upload a blob to your PDS
```
