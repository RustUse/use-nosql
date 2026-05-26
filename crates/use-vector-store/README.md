# use-vector-store

Vector-store and embedding modeling primitives for `RustUse`.

## Experimental

`use-vector-store` is experimental while `use-nosql` remains below `0.3.0`.

## Example

```rust
use use_vector_store::{Embedding, SimilarityMetric, VectorDimension, VectorId, VectorRecord};

let record = VectorRecord::new(VectorId::new("review_1"), Embedding::new(vec![0.1, 0.2]))
    .with_dimension(VectorDimension::new(2))?
    .with_similarity_metric(SimilarityMetric::Cosine);

assert_eq!(record.dimension(), Some(VectorDimension::new(2)));
# Ok::<(), use_vector_store::InvalidDimensionError>(())
```

## Scope

- Vector identifiers, embeddings, dimensions, similarity metrics, metadata, and records.
- Dimension consistency validation.

## Non-goals

- Qdrant, Pinecone, Weaviate, or other vector database clients.
- Approximate nearest-neighbor indexes.
- Embedding generation.

## License

Licensed under either Apache-2.0 or MIT.
