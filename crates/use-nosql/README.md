# use-nosql

Feature-gated facade crate for RustUse NoSQL-style data modeling primitives.

## Experimental

`use-nosql` is experimental while the workspace remains below `0.3.0`.

## Example

```rust
# #[cfg(feature = "full")]
# {
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
# }
```

```rust
# #[cfg(feature = "full")]
# {
use use_nosql::{CacheKey, CacheNamespace};

let key = CacheKey::builder()
    .namespace(CacheNamespace::new("reviews"))
    .segment("google-business-profile")
    .segment("location")
    .segment("fort-wayne")
    .segment("summary")
    .build();

assert_eq!(key.to_string(), "reviews:google-business-profile:location:fort-wayne:summary");
# }
```

```rust
# #[cfg(feature = "full")]
# {
use use_nosql::{Embedding, SimilarityMetric, VectorDimension, VectorId, VectorRecord};

let record = VectorRecord::new(
    VectorId::new("review_789_embedding"),
    Embedding::new(vec![0.012, -0.032, 0.481]),
)
.with_dimension(VectorDimension::new(3))
.unwrap()
.with_similarity_metric(SimilarityMetric::Cosine);

assert_eq!(record.dimension(), Some(VectorDimension::new(3)));
# }
```

## Scope

- Vendor-neutral document, key-value, wide-column, graph-store, cache, search, vector, time-series, partition, consistency, and change-event primitives.
- Feature-gated re-exports for focused child crates.
- Small data containers, labels, and validation helpers.

## Non-goals

- Database drivers or clients.
- ORM, migration, query execution, or connection-pool abstractions.
- Vendor SDK wrappers.
- Network calls or async runtime dependencies.

## License

Licensed under either Apache-2.0 or MIT.
