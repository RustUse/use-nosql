# use-key-value-store

Key-value store modeling primitives for `RustUse`.

## Experimental

`use-key-value-store` is experimental while `use-nosql` remains below `0.3.0`.

## Example

```rust
use use_key_value_store::{Key, KeyPrefix, KeyValueEntry, Value};

let key = Key::with_prefix(&KeyPrefix::new("tenant:acme"), "profile");
let entry = KeyValueEntry::new(key, Value::new("cached payload"));

assert_eq!(entry.key().as_str(), "tenant:acme:profile");
```

## Scope

- Key, value, prefix, namespace, bucket, pattern, and range labels.
- Simple segment composition with `:` separators.
- Small entry and range containers.

## Non-goals

- Redis clients or protocol support.
- Network calls.
- Cache/database persistence.

## License

Licensed under either Apache-2.0 or MIT.
