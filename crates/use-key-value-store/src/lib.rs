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
    /// A key-value store key.
    Key
}
string_newtype! {
    /// A key-value store value payload.
    Value
}
string_newtype! {
    /// A reusable key prefix.
    KeyPrefix
}
string_newtype! {
    /// A key namespace.
    KeyNamespace
}
string_newtype! {
    /// A bucket or logical storage name.
    BucketName
}
string_newtype! {
    /// A simple key pattern label.
    KeyPattern
}

impl Key {
    /// Builds a key by joining non-empty segments with `:`.
    pub fn from_segments(segments: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        let joined = segments
            .into_iter()
            .map(|segment| segment.as_ref().to_owned())
            .filter(|segment| !segment.is_empty())
            .collect::<Vec<_>>()
            .join(":");
        Self::new(joined)
    }

    /// Builds a key under a prefix.
    pub fn with_prefix(prefix: &KeyPrefix, segment: impl AsRef<str>) -> Self {
        Self::from_segments([prefix.as_str(), segment.as_ref()])
    }
}

/// A key-value entry.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct KeyValueEntry {
    key: Key,
    value: Value,
}

impl KeyValueEntry {
    /// Creates a key-value entry.
    pub fn new(key: Key, value: Value) -> Self {
        Self { key, value }
    }

    /// Returns the entry key.
    pub const fn key(&self) -> &Key {
        &self.key
    }

    /// Returns the entry value.
    pub const fn value(&self) -> &Value {
        &self.value
    }
}

/// A half-open or closed key range description.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct KeyRange {
    start: Option<Key>,
    end: Option<Key>,
}

impl KeyRange {
    /// Creates a key range from optional bounds.
    pub const fn new(start: Option<Key>, end: Option<Key>) -> Self {
        Self { start, end }
    }

    /// Creates a range starting at `start`.
    pub fn starting_at(start: Key) -> Self {
        Self::new(Some(start), None)
    }

    /// Creates a range ending at `end`.
    pub fn ending_at(end: Key) -> Self {
        Self::new(None, Some(end))
    }

    /// Returns the start bound.
    pub const fn start(&self) -> Option<&Key> {
        self.start.as_ref()
    }

    /// Returns the end bound.
    pub const fn end(&self) -> Option<&Key> {
        self.end.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{BucketName, Key, KeyPrefix, KeyRange, KeyValueEntry, Value};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn constructs_and_displays_keys() {
        let key = Key::from_segments(["tenant", "customer", "123"]);
        assert_eq!(key.to_string(), "tenant:customer:123");
        assert_eq!(key.as_ref(), "tenant:customer:123");
        assert_eq!(BucketName::new("cache").to_string(), "cache");
    }

    #[test]
    fn composes_keys_with_prefixes() {
        let key = Key::with_prefix(&KeyPrefix::new("tenant:acme"), "profile");
        let entry = KeyValueEntry::new(key.clone(), Value::new("payload"));
        let range = KeyRange::starting_at(key.clone());

        assert_eq!(key.as_str(), "tenant:acme:profile");
        assert_eq!(entry.key(), &key);
        assert_eq!(range.start(), Some(&key));
    }

    #[test]
    fn hashes_equal_keys() {
        let mut left = DefaultHasher::new();
        let mut right = DefaultHasher::new();
        Key::new("same").hash(&mut left);
        Key::new("same").hash(&mut right);
        assert_eq!(left.finish(), right.finish());
    }
}
