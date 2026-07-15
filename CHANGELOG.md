# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This library started as a personal project and will remain v0.x until the API is stable, test coverage is high, and it's fit for general use. AI agents assisted with implementation, but the API design prioritizes idiomatic Rust and ergonomics for human developers.

## [Unreleased]

## [0.2.0] - 2026-07-15

### Added

- Leaflet lexicon support (`lexicons::Leaflet` module) with fully typed record types:
  - `Leaflet::Document`, `Leaflet::Publication`, `Leaflet::PublicationPage`
  - `Leaflet::Comment`, `Leaflet::Subscription`, `Leaflet::Recommend`
  - `Leaflet::PollDefinition`, `Leaflet::PollVote`
  - Leaflet theme types: `Leaflet::BackgroundImage`, `Leaflet::Wordmark`
  - Leaflet content types: `Leaflet::Content`, `Leaflet::Page` (LinearDocument/Canvas), `Leaflet::Block` (22 typed variants)
  - Rich text: `Leaflet::Facet`, `Leaflet::FacetFeature` (11 feature variants), `Leaflet::ByteSlice`
- `standard::Content` tagged enum on `standard::Document.content` — `Content::Leaflet(LeafletContent)` when published via Leaflet.
- Examples: `leaflet_overview`, `leaflet_render` for listing and rendering Leaflet-published blog content.

### Changed

- **Breaking:** site.standard types moved from `lexicons::*` to `lexicons::standard::*`.
  - `lexicons::Document` → `lexicons::standard::Document`
  - `lexicons::Publication` → `lexicons::standard::Publication`
  - `lexicons::Subscription` → `lexicons::standard::Subscription`
  - `lexicons::Recommend` → `lexicons::standard::Recommend`
  - `lexicons::{BasicTheme, Rgb, Rgba}` → `lexicons::standard::{BasicTheme, Rgb, Rgba}`
- Leaflet type names shortened by dropping the `Leaflet` prefix (e.g., `LeafletDocument` → `Leaflet::Document`).
- `render` example updated to use typed `Leaflet::Block` / `Leaflet::Facet` API.

## [0.1.0] - 2026-07-03

### Added

- Handle → DID resolution via HTTP `.well-known/atproto-did` with DNS `_atproto` TXT fallback.
- DID → PDS resolution supporting `did:plc` and `did:web` methods.
- `Repo` struct for fetching AT Protocol collections from a user's PDS.
- `Collection` trait with NSID constants for type-safe lexicon access.
- Cursor-based pagination via `fetch_cursor` and `fetch_all`.
- Built-in lexicon types:
  - `site.standard.document` (Document)
  - `site.standard.publication` (Publication)
  - `site.standard.graph.subscription` (Subscription)
  - `site.standard.graph.recommend` (Recommend)
  - `site.standard.theme.basic` (BasicTheme, Rgb, Rgba)
  - Shared types: Blob, BlobLink, StrongRef, SelfLabel
- Examples: basic fetch, custom schemas (Bluesky posts), pagination, document rendering, blog publication filtering.
