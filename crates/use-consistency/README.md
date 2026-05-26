# use-consistency

Consistency and durability label primitives for `RustUse`.

## Experimental

`use-consistency` is experimental while `use-nosql` remains below `0.3.0`.

## Example

```rust
use use_consistency::{ConsistencyLevel, Quorum, ReadConcern, ReplicationFactor};

let read = ReadConcern::new(ConsistencyLevel::LocalQuorum);
let replication = ReplicationFactor::new(3);
let quorum = Quorum::new(2);

assert_eq!(read.to_string(), "local-quorum");
assert_eq!(replication.value(), 3);
assert_eq!(quorum.value(), 2);
```

## Scope

- Consistency, read concern, write concern, durability, replication factor, and quorum labels.
- Descriptive, vendor-neutral semantics.

## Non-goals

- Vendor-specific consistency guarantees.
- Cluster management.
- Read or write execution.

## License

Licensed under either Apache-2.0 or MIT.
