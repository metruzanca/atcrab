# Examples

Example output for each example. Run them yourself with `cargo run --example <name>`.

## basic

Fetches standard.site documents with built-in `standard_site::Document` type.

```
cargo run --example basic
```

<details>
<summary>Output</summary>

```
DID:  did:plc:msockseqtdlbyntubnhntlmo
PDS:  https://velvetfoot.us-east.host.bsky.network

Found 3 document(s):
  [Just a dummy post to understand images in leaflet] https://pdsls.dev/at://did:plc:msockseqtdlbyntubnhntlmo/site.standard.document/3mqp6niuplk2t
  [Repeat yourself, dammit!] https://pdsls.dev/at://did:plc:msockseqtdlbyntubnhntlmo/site.standard.document/3mpvuj4ijzc2s
  [Stop choosing Convenience] https://pdsls.dev/at://did:plc:msockseqtdlbyntubnhntlmo/site.standard.document/3mpur4lejvs27
```

</details>

## custom_schema

Fetches Bluesky posts using a user-defined `Post` type implementing `Collection`.

```
cargo run --example custom_schema
```

<details>
<summary>Output (truncated — 50 posts shown)</summary>

```
Found 50 post(s):
  2026-07-15T17:01:55.355Z  Signed up. 🫡🇮🇹
  2026-07-15T17:01:12.882Z  Err I meant to say follow people that talk about things you care about.
  2026-07-15T17:00:48.698Z  Follow people you care about and mute words you don't want to hear talked about.

Use some custom peeds. There's friends of friends likes which is pretty neat.

My discover isn't bad either.

My entire feed is mostly just software development.
  2026-07-14T20:13:31.857Z  Nice.
  2026-07-13T07:29:56.326Z  I'm happy using lower cost models having previously used opus. What's your workload like?
  2026-07-13T07:27:02.490Z  What were the reasons you migrated to that pds?
  2026-07-13T07:26:01.672Z  Started using deekseek v4 flash exclusively. Its surprisingly good for how cheap and fast it is.

I'm building small features at a time and iterating. No need for an expensive model. Big feature? Break it up.
  2026-07-13T04:32:03.887Z  Friends of mine are making a voice call ai automation business...
  2026-07-13T03:49:58.279Z  My simple dioxus app just turned non-trivial...
  ...
```

</details>

## pagination

Demonstrates cursor-based pagination with `fetch`, `fetch_cursor`, and `fetch_all`.

```
cargo run --example pagination
```

<details>
<summary>Output (truncated)</summary>

```
fetch_collection()  → 50 records on first page
                        2026-07-15T17:01:55.355Z  Signed up. 🫡🇮🇹
                        2026-07-15T17:01:12.882Z  Err I meant to say follow people...
                        2026-07-15T17:00:48.698Z  Follow people you care about...
                        ...
                        cursor available: 3mptfduovyc2i
fetch_collection_cursor() → 50 records on second page
fetch_all_collection() → 116 total records across all pages
```

</details>

## render

Pretty-prints standard.site documents using typed `leaflet_pub::Block` and `leaflet_pub::Facet` enums with hyperlink rendering (OSC-8 escape sequences for clickable URLs in supporting terminals).

```
cargo run --example render
```

<details>
<summary>Output</summary>

```
──────────────────────────────────────────────────────────────────────
# Just a dummy post to understand images in leaflet
2026-07-15

  [image: 600x600]


uri: https://pdsls.dev/at://did:plc:msockseqtdlbyntubnhntlmo/site.standard.document/3mqp6niuplk2t

──────────────────────────────────────────────────────────────────────
# Repeat yourself, dammit!
2024-04-05
tags: #programming #best-practice

> Dry is dead and we Killed it.


  ### Dry is Dead and we Killed it.
  Stop using DRY or making everything into a component/function...
  ...
  > Optimize for change first.
  ────────────────────────────────────────

  ### What other people say
  Other people have said this before and likely did a better job.
    - AHA Programming by Kent C. Dodds
    - The Wrong Abstraction by Sandi Metz
    - ...


uri: https://pdsls.dev/at://did:plc:msockseqtdlbyntubnhntlmo/site.standard.document/3mpvuj4ijzc2s

──────────────────────────────────────────────────────────────────────
# Stop choosing Convenience
2026-07-05
tags: #electron #rust #agents

> We've optimized for developer convenience for years...

  For years, I've been frustrated by the number of desktop apps...
  ...
──────────────────────────────────────────────────────────────────────
3 document(s)
```

</details>

## blog

Fetches publications and their documents, joining by the `site` field.

```
cargo run --example blog
```

<details>
<summary>Output</summary>

```
[Sam's Blog] https://pdsls.dev/at://did:plc:msockseqtdlbyntubnhntlmo/site.standard.publication/3mpr6p5us2227
- Just a dummy post to understand images in leaflet https://pdsls.dev/at://did:plc:msockseqtdlbyntubnhntlmo/site.standard.document/3mqp6niuplk2t
- Repeat yourself, dammit! https://pdsls.dev/at://did:plc:msockseqtdlbyntubnhntlmo/site.standard.document/3mpvuj4ijzc2s
- Stop choosing Convenience https://pdsls.dev/at://did:plc:msockseqtdlbyntubnhntlmo/site.standard.document/3mpur4lejvs27
```

</details>

## leaflet_overview

Lists all publications from metru.dev with a plaintext preview (first text block) for each post, using the typed `leaflet_pub::Block` enum.

```
cargo run --example leaflet_overview
```

<details>
<summary>Output</summary>

```
[Metru's Notes] at://did:plc:msockseqtdlbyntubnhntlmo/site.standard.publication/3mqp5ut7dhs23
  (no posts)

[Sam's Blog] at://did:plc:msockseqtdlbyntubnhntlmo/site.standard.publication/3mpr6p5us2227
  - Just a dummy post to understand images in leaflet — 
  - Repeat yourself, dammit! — 
  - Stop choosing Convenience — For years, I've been frustrated by the number of desktop apps companies ship ...
```

</details>

## leaflet_render

Renders a specific publication's posts as plaintext markdown, handling headers, text, blockquotes, images, horizontal rules, lists, and code blocks via the `leaflet_pub::Block` enum.

```
cargo run --example leaflet_render
```

<details>
<summary>Output</summary>

```
# Sam's Blog

## Just a dummy post to understand images in leaflet
_2026-07-15_

[image: 600x600]

---

## Repeat yourself, dammit!
_2024-04-05_

> Dry is dead and we Killed it.

## Dry is Dead and we Killed it.

Stop using DRY or making everything into a component/function...

---

## What other people say

  - AHA Programming by Kent C. Dodds
  - The Wrong Abstraction by Sandi Metz
  - Please do repeat yourself (DRY is dead) by Rafal Stozek
  - Do Repeat Yourself by Ted Spence
  - Do Repeat yourself by Lucian Radu Teodorescu

---

## Stop choosing Convenience
_2026-07-05_

> We've optimized for developer convenience for years...

For years, I've been frustrated by the number of desktop apps...
...
```

</details>

## create_record

Creates, updates, and deletes a record in the user's PDS (requires `ATP_PASSWORD` env var).

```
cargo run --example create_record
```

<details>
<summary>Output</summary>

```
Created: at://did:plc:msockseqtdlbyntubnhntlmo/net.example.test/3mpxxxxxxxxxx
Updated: at://did:plc:msockseqtdlbyntubnhntlmo/net.example.test/3mpxxxxxxxxxx
Deleted: 3mpxxxxxxxxxx
```

</details>

## upload_blob

Uploads a blob to the user's PDS (requires `ATP_PASSWORD` env var).

```
cargo run --example upload_blob
```

<details>
<summary>Output</summary>

```
Uploaded blob: bafkreifz...
  mimeType: text/plain
  size: 11
```

</details>
