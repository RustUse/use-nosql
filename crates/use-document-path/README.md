# use-document-path

Document path primitives for `RustUse` NoSQL-style data modeling.

## Experimental

`use-document-path` is experimental while `use-nosql` remains below `0.3.0`.

## Example

```rust
use use_document_path::{DocumentPath, PathParseError};

let path = DocumentPath::try_new("profile.display_name")?;
let segments = path.segments()?;

assert_eq!(path.as_str(), "profile.display_name");
assert_eq!(segments.len(), 2);
# Ok::<(), PathParseError>(())
```

## Scope

- Dot-path strings such as `profile.display_name`.
- Path segment and field selector labels.
- Conservative path parsing errors.

## Non-goals

- JSON Pointer implementation.
- JSON document parsing.
- Database-specific path syntax.

## License

Licensed under either Apache-2.0 or MIT.
