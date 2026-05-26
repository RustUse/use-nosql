# use-cache-store

Cache store modeling primitives for `RustUse`.

## Experimental

`use-cache-store` is experimental while `use-nosql` remains below `0.3.0`.

## Example

```rust
use use_cache_store::{CacheEntry, CacheKey, CacheNamespace, CacheValue, Ttl};

let key = CacheKey::builder()
    .namespace(CacheNamespace::new("reviews"))
    .segment("summary")
    .build();
let entry = CacheEntry::new(key, CacheValue::new("cached")).with_ttl(Ttl::seconds(60)?);

assert_eq!(entry.ttl().map(Ttl::duration).unwrap().as_secs(), 60);
# Ok::<(), use_cache_store::InvalidTtlError>(())
```

## Scope

- Cache key, namespace, value, entry, TTL, expiration, status, and eviction labels.
- Simple key composition primitives.

## Non-goals

- Redis or Memcached clients.
- Network calls.
- Cache invalidation engines.

## License

Licensed under either Apache-2.0 or MIT.
