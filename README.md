# RustUse/use-nosql

`use-nosql` is a RustUse facade workspace for small, focused Rust 2024 primitive crates around NoSQL-style data modeling.

RustUse is a collection of composable sets of primitive Rust utility crates for fellow crustaceans.

`use-nosql` covers document stores, key-value stores, wide-column stores, graph stores, cache stores, search indexes, vector stores, time-series stores, partitioning, consistency labels, and change events.

## Experimental

`use-nosql` is experimental while the workspace remains below `0.3.0`. Expect small API refinements during the first release wave.

## Non-goals

`use-nosql` is not:

- a database driver
- a database client
- an ORM
- a connection pool
- a query execution engine
- a migration tool
- an async networking abstraction
- a vendor SDK wrapper

The crates avoid network calls, database connections, query execution, driver behavior, ORM behavior, migrations, and vendor-specific adapters.

## Relationships

- `use-nosql` models generic NoSQL-style data modeling primitives.
- `use-sql` models generic SQL language, value, query, and schema primitives.
- `use-postgres` models PostgreSQL-specific metadata primitives.
- `use-data` models generic data format helpers.
- `use-graph` remains the home for graph-theory algorithms and general graph primitives.

## Workspace crates

| Crate                  | Path                           | Purpose                                                                                 |
| ---------------------- | ------------------------------ | --------------------------------------------------------------------------------------- |
| `use-nosql`            | `crates/use-nosql/`            | Feature-gated facade over the focused NoSQL primitive crates                            |
| `use-document-store`   | `crates/use-document-store/`   | Document-store identifiers, metadata, and patch operation primitives                    |
| `use-key-value-store`  | `crates/use-key-value-store/`  | Key, namespace, bucket, entry, pattern, and range primitives                            |
| `use-wide-column`      | `crates/use-wide-column/`      | Wide-column keyspace, family, row, column, and clustering primitives                    |
| `use-graph-store`      | `crates/use-graph-store/`      | Property-graph store vertex, edge, label, and property primitives                       |
| `use-cache-store`      | `crates/use-cache-store/`      | Cache key, namespace, entry, TTL, expiration, status, and eviction primitives           |
| `use-search-index`     | `crates/use-search-index/`     | Search index document, field, term, analyzer, query-shape, sort, and filter primitives  |
| `use-vector-store`     | `crates/use-vector-store/`     | Vector ID, embedding, dimension, metric, metadata, and record primitives                |
| `use-timeseries-store` | `crates/use-timeseries-store/` | Series, metric, timestamp, point, retention, sampling, and aggregation primitives       |
| `use-document-path`    | `crates/use-document-path/`    | Dot-path, segment, selector, and parse-error primitives                                 |
| `use-partition-key`    | `crates/use-partition-key/`    | Partition, shard, routing, sort, composite key, and strategy primitives                 |
| `use-consistency`      | `crates/use-consistency/`      | Consistency, read concern, write concern, durability, replication, and quorum labels    |
| `use-change-event`     | `crates/use-change-event/`     | Change event, event ID, cursor, resume token, sequence, and changed-document primitives |

## Installation

Use the workspace directly or depend on a Git revision until the first crates.io release is published.

```toml
[dependencies]
use-nosql = { git = "https://github.com/RustUse/use-nosql", rev = "<commit>" }
```

After publication, choose the narrowest focused crate that matches your use case or use the facade when one dependency is more convenient.

```toml
[dependencies]
use-nosql = "0.1.0"
```

## Basic usage

```rust
use use_nosql::{CollectionName, DocumentId, DocumentPath, PatchOperation, PatchSet};

let collection = CollectionName::new("customers");
let document_id = DocumentId::new("customer_123");
let patch = PatchSet::new(vec![PatchOperation::set(
    DocumentPath::new("profile.display_name"),
    "Joshua Whalen",
)]);

assert_eq!(collection.as_str(), "customers");
assert_eq!(document_id.as_str(), "customer_123");
assert_eq!(patch.operations().len(), 1);
```

```rust
use use_nosql::{CacheKey, CacheNamespace};

let key = CacheKey::builder()
    .namespace(CacheNamespace::new("reviews"))
    .segment("google-business-profile")
    .segment("location")
    .segment("fort-wayne")
    .segment("summary")
    .build();

assert_eq!(key.to_string(), "reviews:google-business-profile:location:fort-wayne:summary");
```

```rust
use use_nosql::{Embedding, SimilarityMetric, VectorDimension, VectorId, VectorRecord};

let record = VectorRecord::new(
    VectorId::new("review_789_embedding"),
    Embedding::new(vec![0.012, -0.032, 0.481]),
)
.with_dimension(VectorDimension::new(3))
.unwrap()
.with_similarity_metric(SimilarityMetric::Cosine);

assert_eq!(record.dimension(), Some(VectorDimension::new(3)));
```

## Development

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo check --workspace --all-features --examples
cargo doc --workspace --all-features --no-deps
```

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
