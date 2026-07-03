# AT Crab

Rust AT Protocol libraries. Loosely inspired by [Mary](https://github.com/mary-ext)'s [atcute](https://github.com/mary-ext/atcute).

> Do not use this in production. See https://atproto.com/sdks for alternatives.

## Adding as a dependency

```toml
[dependencies]
atcrab = { git = "https://github.com/<user>/atcrab" }
```

## Usage

### Fetch with built-in types

Built-in lexicon types implement `Collection` so the NSID is inferred:

```rust
use atcrab::lexicons::Document;
use atcrab::Repo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repo::new("metru.dev").await?;
    let docs = repo.fetch_collection::<Document>().await?;
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
    let posts = repo.fetch_collection::<Post>().await?;
    println!("{} posts", posts.records.len());
    Ok(())
}
```

### Low-level API

If you don't want to implement `Collection`, pass the NSID directly:

```rust
// first page
let posts = repo.fetch::<Post>("app.bsky.feed.post").await?;
// next page from cursor
let posts = repo.fetch_cursor::<Post>("app.bsky.feed.post", cursor).await?;
// all pages
let all   = repo.fetch_all::<Post>("app.bsky.feed.post").await?;
```

## Examples

```sh
cargo run --example basic           # fetch documents with built-in types
cargo run --example custom_schema   # fetch posts with a user-defined type
cargo run --example pagination      # cursor-based pagination
cargo run --example render          # pretty-print documents
```
