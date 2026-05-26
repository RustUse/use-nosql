# use-document-store

Document-store modeling primitives for `RustUse`.

## Experimental

`use-document-store` is experimental while `use-nosql` remains below `0.3.0`.

## Example

```rust
use use_document_store::{CollectionName, DocumentId, PatchOperation, PatchSet};

let collection = CollectionName::new("customers");
let document_id = DocumentId::new("customer_123");
let patch = PatchSet::new(vec![PatchOperation::set("profile.display_name", "Ada")]);

assert_eq!(collection.as_str(), "customers");
assert_eq!(document_id.to_string(), "customer_123");
assert_eq!(patch.operations()[0].path(), "profile.display_name");
```

## Scope

- Collection, document, field, revision, version, and metadata labels.
- Vendor-neutral patch operation containers.
- String-backed values without a required JSON dependency.

## Non-goals

- Document database clients.
- ORM behavior.
- Query execution or persistence.
- Vendor-specific patch syntax.

## License

Licensed under either Apache-2.0 or MIT.
