# use-wide-column

Wide-column store modeling primitives for `RustUse`.

## Experimental

`use-wide-column` is experimental while `use-nosql` remains below `0.3.0`.

## Example

```rust
use use_wide_column::{ColumnName, ColumnValue, PartitionKey, WideColumnRow};

let row = WideColumnRow::new(PartitionKey::new("customer_123"))
    .with_column(ColumnName::new("display_name"), ColumnValue::new("Ada"));

assert_eq!(row.partition_key().as_str(), "customer_123");
assert_eq!(row.columns().len(), 1);
```

## Scope

- Generic keyspace, column-family, table, partition, clustering, column, and row labels.
- Vendor-neutral row and column-family containers.

## Non-goals

- Cassandra or Scylla clients.
- Query execution.
- Vendor-specific consistency or schema syntax.

## License

Licensed under either Apache-2.0 or MIT.
