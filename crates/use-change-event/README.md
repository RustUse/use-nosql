# use-change-event

Change-event and stream-event primitives for `RustUse`.

## Experimental

`use-change-event` is experimental while `use-nosql` remains below `0.3.0`.

## Example

```rust
use use_change_event::{ChangeEvent, ChangeEventId, ChangeEventKind, ChangeSequence};

let event = ChangeEvent::new(
    ChangeEventId::new("evt_1"),
    ChangeEventKind::Update,
    "customer_123",
)
.with_sequence(ChangeSequence::new(42));

assert_eq!(event.kind(), ChangeEventKind::Update);
assert_eq!(event.sequence(), Some(ChangeSequence::new(42)));
```

## Scope

- Change event identifiers, kinds, cursors, resume tokens, sequences, and changed document references.
- Stream-event modeling without driver behavior.

## Non-goals

- Change stream clients.
- Network calls.
- Vendor-specific resume semantics.

## License

Licensed under either Apache-2.0 or MIT.
