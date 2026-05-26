# use-search-index

Search-index modeling primitives for `RustUse`.

## Experimental

`use-search-index` is experimental while `use-nosql` remains below `0.3.0`.

## Example

```rust
use use_search_index::{SearchDocumentId, SearchField, SearchIndexDocument, SearchQueryShape};

let document = SearchIndexDocument::new(SearchDocumentId::new("review_1"))
    .with_field(SearchField::new("title"), "Great service");

assert_eq!(document.fields().len(), 1);
assert_eq!(SearchQueryShape::Term.to_string(), "term");
```

## Scope

- Index, document, field, term, analyzer, query-shape, sort, and filter labels.
- Small search-index document containers.

## Non-goals

- Search engine implementation.
- Elasticsearch, OpenSearch, or vendor clients.
- Query execution.

## License

Licensed under either Apache-2.0 or MIT.
