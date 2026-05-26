# use-graph-store

Property-graph store modeling primitives for `RustUse`.

## Experimental

`use-graph-store` is experimental while `use-nosql` remains below `0.3.0`.

## Example

```rust
use use_graph_store::{GraphProperty, GraphVertex, PropertyKey, PropertyValue, VertexId, VertexLabel};

let vertex = GraphVertex::new(VertexId::new("person_1"), VertexLabel::new("Person"))
    .with_property(GraphProperty::new(PropertyKey::new("name"), PropertyValue::new("Ada")));

assert_eq!(vertex.label().as_str(), "Person");
assert_eq!(vertex.properties().len(), 1);
```

## Scope

- Property-graph store identifiers, labels, properties, vertices, and edges.
- Data modeling for graph-store records.

## Non-goals

- Graph-theory algorithms.
- Replacement for the `use-graph` RustUse set.
- Graph database clients or traversals.

## License

Licensed under either Apache-2.0 or MIT.
