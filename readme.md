# AT Crab

Rust AT Protocol libraries. Loosely inspired by [Mary](https://github.com/mary-ext)'s [atcute](https://github.com/mary-ext/atcute).

> Do not use this in production. See https://atproto.com/sdks for alternatives.

## Adding as a dependency

```toml
[dependencies]
atcrab = { git = "https://github.com/<user>/atcrab" }
```

## Usage

### Fetch a collection

```rust
use atcrab::Repo;
use serde::Deserialize;

#[derive(Deserialize)]
struct Post { text: String }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repo::new("metru.dev").await?;
    let posts = repo.fetch::<Post>("app.bsky.feed.post").await?;
    println!("{} posts", posts.records.len());
    Ok(())
}
```

### Built-in lexicon types

The `lexicons` module provides type definitions for well-known schemas:

```rust
use atcrab::lexicons::Document;
use atcrab::Repo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repo::new("metru.dev").await?;
    let docs = repo.fetch::<Document>("site.standard.document").await?;
    for record in &docs.records {
        println!("{} ({})", record.value.title, record.uri);
    }
    Ok(())
}
```

**Included types:**

| Lexicon | Type |
|---|---|
| `site.standard.document` | `Document`, `Contributor` |
| `site.standard.publication` | `Publication`, `Preferences` |
| `site.standard.graph.subscription` | `Subscription` |
| `site.standard.graph.recommend` | `Recommend` |
| `site.standard.theme.basic` | `BasicTheme` |
| `site.standard.theme.color` | `Rgb`, `Rgba` |

**Shared ATProto types:** `Blob`, `BlobLink`, `StrongRef`, `SelfLabel`

## Examples

```sh
cargo run --example basic           # fetch documents with built-in types
cargo run --example custom_schema   # fetch posts with a user-defined type
cargo run --example pagination      # cursor-based pagination
```

## Project structure

```
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs          # public API
│   ├── repo.rs         # Repo struct, collection fetching
│   ├── handle.rs       # Handle → DID resolution
│   ├── did.rs          # DID → PDS endpoint resolution
│   ├── types.rs        # Record<T>, ListRecords<T>
│   ├── error.rs        # Error types
│   └── lexicons/       # Lexicon type definitions
│       ├── types.rs    # Shared ATProto types
│       ├── document.rs
│       ├── publication.rs
│       ├── subscription.rs
│       ├── recommend.rs
│       └── theme.rs
└── examples/
    ├── basic.rs
    ├── custom_schema.rs
    └── pagination.rs
```
