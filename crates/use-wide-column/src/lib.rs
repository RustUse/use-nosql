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
    /// A generic wide-column keyspace name.
    KeyspaceName
}
string_newtype! {
    /// A generic column-family name.
    ColumnFamilyName
}
string_newtype! {
    /// A generic wide-column table name.
    WideTableName
}
string_newtype! {
    /// A wide-column partition key.
    PartitionKey
}
string_newtype! {
    /// A clustering key within a partition.
    ClusteringKey
}
string_newtype! {
    /// A wide-column column name.
    ColumnName
}
string_newtype! {
    /// A wide-column value payload.
    ColumnValue
}

/// A column-family descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnFamily {
    name: ColumnFamilyName,
    columns: Vec<ColumnName>,
}

impl ColumnFamily {
    /// Creates a column family with no registered columns.
    pub fn new(name: ColumnFamilyName) -> Self {
        Self {
            name,
            columns: Vec::new(),
        }
    }

    /// Adds a column name.
    pub fn with_column(mut self, column: ColumnName) -> Self {
        self.columns.push(column);
        self
    }

    /// Returns the family name.
    pub const fn name(&self) -> &ColumnFamilyName {
        &self.name
    }

    /// Returns the registered columns.
    pub fn columns(&self) -> &[ColumnName] {
        &self.columns
    }
}

/// A generic wide-column row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WideColumnRow {
    partition_key: PartitionKey,
    clustering_key: Option<ClusteringKey>,
    columns: Vec<(ColumnName, ColumnValue)>,
}

impl WideColumnRow {
    /// Creates a row for a partition key.
    pub fn new(partition_key: PartitionKey) -> Self {
        Self {
            partition_key,
            clustering_key: None,
            columns: Vec::new(),
        }
    }

    /// Sets the clustering key.
    pub fn with_clustering_key(mut self, clustering_key: ClusteringKey) -> Self {
        self.clustering_key = Some(clustering_key);
        self
    }

    /// Adds a column value.
    pub fn with_column(mut self, name: ColumnName, value: ColumnValue) -> Self {
        self.columns.push((name, value));
        self
    }

    /// Returns the partition key.
    pub const fn partition_key(&self) -> &PartitionKey {
        &self.partition_key
    }

    /// Returns the clustering key, if present.
    pub const fn clustering_key(&self) -> Option<&ClusteringKey> {
        self.clustering_key.as_ref()
    }

    /// Returns row columns.
    pub fn columns(&self) -> &[(ColumnName, ColumnValue)] {
        &self.columns
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ClusteringKey, ColumnFamily, ColumnFamilyName, ColumnName, ColumnValue, KeyspaceName,
        PartitionKey, WideColumnRow, WideTableName,
    };

    #[test]
    fn constructs_wide_column_labels() {
        assert_eq!(KeyspaceName::new("app").to_string(), "app");
        assert_eq!(WideTableName::new("events").as_ref(), "events");
        assert_eq!(ColumnFamilyName::new("profile").as_str(), "profile");
    }

    #[test]
    fn builds_families_and_rows() {
        let family = ColumnFamily::new(ColumnFamilyName::new("profile"))
            .with_column(ColumnName::new("display_name"));
        let row = WideColumnRow::new(PartitionKey::new("customer_123"))
            .with_clustering_key(ClusteringKey::new("2026-05"))
            .with_column(ColumnName::new("display_name"), ColumnValue::new("Joshua"));

        assert_eq!(family.columns().len(), 1);
        assert_eq!(row.partition_key().as_str(), "customer_123");
        assert_eq!(row.columns().len(), 1);
    }
}
