# Releasing

This repository uses a focused-first release flow rather than a single-crate manual publish pattern.

## First publish wave

Publish all focused crates first:

```txt
use-document-store
use-key-value-store
use-wide-column
use-graph-store
use-cache-store
use-search-index
use-vector-store
use-timeseries-store
use-document-path
use-partition-key
use-consistency
use-change-event
```

Wait for crates.io index propagation, then publish `use-nosql`.

## Validation

```sh
cargo fmt --all -- --check
cargo check --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo check --workspace --all-features --examples
cargo doc --workspace --all-features --no-deps
cargo deny check
cargo audit
```
