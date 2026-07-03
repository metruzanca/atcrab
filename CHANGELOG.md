# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This library started as a personal project and will remain v0.x until the API is stable, test coverage is high, and it's fit for general use. AI agents assisted with implementation, but the API design prioritizes idiomatic Rust and ergonomics for human developers.

## [Unreleased]

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
