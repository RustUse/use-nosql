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
    /// A reusable partition key.
    PartitionKey
}
string_newtype! {
    /// A shard key.
    ShardKey
}
string_newtype! {
    /// A routing key.
    RoutingKey
}
string_newtype! {
    /// A sort key.
    SortKey
}

/// A composite partitioning key.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CompositeKey {
    parts: Vec<PartitionKey>,
}

impl CompositeKey {
    /// Creates an empty composite key.
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    /// Creates a composite key from parts.
    pub fn from_parts(parts: Vec<PartitionKey>) -> Self {
        Self { parts }
    }

    /// Adds a partition key part.
    pub fn with_part(mut self, part: PartitionKey) -> Self {
        self.parts.push(part);
        self
    }

    /// Returns all parts.
    pub fn parts(&self) -> &[PartitionKey] {
        &self.parts
    }
}

/// Partition strategy labels.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PartitionStrategy {
    Hash,
    Range,
    List,
    Composite,
    Manual,
    #[default]
    Unknown,
}

impl PartitionStrategy {
    /// Returns a stable lowercase label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hash => "hash",
            Self::Range => "range",
            Self::List => "list",
            Self::Composite => "composite",
            Self::Manual => "manual",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for PartitionStrategy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{CompositeKey, PartitionKey, PartitionStrategy, RoutingKey, ShardKey, SortKey};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn constructs_partition_keys() {
        assert_eq!(PartitionKey::new("tenant_1").to_string(), "tenant_1");
        assert_eq!(ShardKey::new("shard-a").as_ref(), "shard-a");
        assert_eq!(RoutingKey::new("route-1").as_str(), "route-1");
        assert_eq!(SortKey::new("created_at").to_string(), "created_at");
    }

    #[test]
    fn builds_composite_keys_and_formats_strategies() {
        let key = CompositeKey::new()
            .with_part(PartitionKey::new("tenant_1"))
            .with_part(PartitionKey::new("customer_1"));

        assert_eq!(key.parts().len(), 2);
        assert_eq!(PartitionStrategy::Composite.to_string(), "composite");
    }

    #[test]
    fn hashes_equal_partition_keys() {
        let mut left = DefaultHasher::new();
        let mut right = DefaultHasher::new();
        PartitionKey::new("same").hash(&mut left);
        PartitionKey::new("same").hash(&mut right);
        assert_eq!(left.finish(), right.finish());
    }
}
