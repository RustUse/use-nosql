# use-timeseries-store

Time-series store modeling primitives for `RustUse`.

## Experimental

`use-timeseries-store` is experimental while `use-nosql` remains below `0.3.0`.

## Example

```rust
use std::time::{Duration, UNIX_EPOCH};
use use_timeseries_store::{MetricName, SeriesId, TimeSeriesPoint, TimeSeriesValue, Timestamp};

let point = TimeSeriesPoint::new(
    SeriesId::new("host_1"),
    MetricName::new("cpu.usage"),
    Timestamp::new(UNIX_EPOCH + Duration::from_secs(10)),
    TimeSeriesValue::new(0.75),
);

assert_eq!(point.value().value(), 0.75);
```

## Scope

- Series, metric, timestamp, point, value, retention, sampling, and aggregation primitives.
- `std::time`-based time modeling.

## Non-goals

- Time-series database clients.
- Query execution.
- Chronology or calendar libraries.

## License

Licensed under either Apache-2.0 or MIT.
