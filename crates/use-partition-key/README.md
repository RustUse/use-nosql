# use-partition-key

Partition and routing key primitives for `RustUse`.

## Experimental

`use-partition-key` is experimental while `use-nosql` remains below `0.3.0`.

## Example

```rust
use use_partition_key::{CompositeKey, PartitionKey, PartitionStrategy};

let key = CompositeKey::new()
    .with_part(PartitionKey::new("tenant_1"))
    .with_part(PartitionKey::new("customer_1"));

assert_eq!(key.parts().len(), 2);
assert_eq!(PartitionStrategy::Hash.to_string(), "hash");
```

## Scope

- Partition, shard, routing, sort, and composite key labels.
- Partition strategy descriptors.

## Non-goals

- Hashing implementations.
- Cluster topology management.
- Vendor-specific routing algorithms.

## License

Licensed under either Apache-2.0 or MIT.
