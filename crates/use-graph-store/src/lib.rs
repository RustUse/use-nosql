#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

macro_rules! string_newtype {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates a new string-backed primitive.
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            /// Returns the stored string value.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

string_newtype! {
    /// A property-graph vertex identifier.
    VertexId
}
string_newtype! {
    /// A property-graph edge identifier.
    EdgeId
}
string_newtype! {
    /// A property-graph vertex label.
    VertexLabel
}
string_newtype! {
    /// A property-graph edge label.
    EdgeLabel
}
string_newtype! {
    /// A property key.
    PropertyKey
}
string_newtype! {
    /// A property value payload.
    PropertyValue
}
string_newtype! {
    /// A traversal label used by graph-store queries or projections.
    TraversalLabel
}

/// A graph property key/value pair.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GraphProperty {
    key: PropertyKey,
    value: PropertyValue,
}

impl GraphProperty {
    /// Creates a graph property.
    pub fn new(key: PropertyKey, value: PropertyValue) -> Self {
        Self { key, value }
    }

    /// Returns the property key.
    pub const fn key(&self) -> &PropertyKey {
        &self.key
    }

    /// Returns the property value.
    pub const fn value(&self) -> &PropertyValue {
        &self.value
    }
}

/// A property-graph vertex model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphVertex {
    id: VertexId,
    label: VertexLabel,
    properties: Vec<GraphProperty>,
}

impl GraphVertex {
    /// Creates a graph vertex.
    pub fn new(id: VertexId, label: VertexLabel) -> Self {
        Self {
            id,
            label,
            properties: Vec::new(),
        }
    }

    /// Adds a property.
    pub fn with_property(mut self, property: GraphProperty) -> Self {
        self.properties.push(property);
        self
    }

    /// Returns the vertex identifier.
    pub const fn id(&self) -> &VertexId {
        &self.id
    }

    /// Returns the vertex label.
    pub const fn label(&self) -> &VertexLabel {
        &self.label
    }

    /// Returns vertex properties.
    pub fn properties(&self) -> &[GraphProperty] {
        &self.properties
    }
}

/// A property-graph edge model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphEdge {
    id: EdgeId,
    source: VertexId,
    target: VertexId,
    label: EdgeLabel,
    properties: Vec<GraphProperty>,
}

impl GraphEdge {
    /// Creates a graph edge between two vertices.
    pub fn new(id: EdgeId, source: VertexId, target: VertexId, label: EdgeLabel) -> Self {
        Self {
            id,
            source,
            target,
            label,
            properties: Vec::new(),
        }
    }

    /// Adds a property.
    pub fn with_property(mut self, property: GraphProperty) -> Self {
        self.properties.push(property);
        self
    }

    /// Returns the edge identifier.
    pub const fn id(&self) -> &EdgeId {
        &self.id
    }

    /// Returns the source vertex identifier.
    pub const fn source(&self) -> &VertexId {
        &self.source
    }

    /// Returns the target vertex identifier.
    pub const fn target(&self) -> &VertexId {
        &self.target
    }

    /// Returns the edge label.
    pub const fn label(&self) -> &EdgeLabel {
        &self.label
    }

    /// Returns edge properties.
    pub fn properties(&self) -> &[GraphProperty] {
        &self.properties
    }
}

#[cfg(test)]
mod tests {
    use super::{
        EdgeId, EdgeLabel, GraphEdge, GraphProperty, GraphVertex, PropertyKey, PropertyValue,
        TraversalLabel, VertexId, VertexLabel,
    };

    #[test]
    fn constructs_graph_store_labels() {
        assert_eq!(VertexId::new("v1").to_string(), "v1");
        assert_eq!(TraversalLabel::new("friends").as_ref(), "friends");
    }

    #[test]
    fn builds_vertices_and_edges() {
        let property = GraphProperty::new(PropertyKey::new("name"), PropertyValue::new("Ada"));
        let vertex = GraphVertex::new(VertexId::new("person_1"), VertexLabel::new("Person"))
            .with_property(property.clone());
        let edge = GraphEdge::new(
            EdgeId::new("edge_1"),
            VertexId::new("person_1"),
            VertexId::new("person_2"),
            EdgeLabel::new("knows"),
        )
        .with_property(property);

        assert_eq!(vertex.properties().len(), 1);
        assert_eq!(edge.source().as_str(), "person_1");
        assert_eq!(edge.properties().len(), 1);
    }
}
