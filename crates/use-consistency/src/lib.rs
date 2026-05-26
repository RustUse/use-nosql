#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

/// Consistency level labels for read/write modeling.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsistencyLevel {
    One,
    Two,
    Three,
    Quorum,
    All,
    LocalQuorum,
    EventuallyConsistent,
    StronglyConsistent,
    #[default]
    Unknown,
}

impl ConsistencyLevel {
    /// Returns a stable lowercase label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::One => "one",
            Self::Two => "two",
            Self::Three => "three",
            Self::Quorum => "quorum",
            Self::All => "all",
            Self::LocalQuorum => "local-quorum",
            Self::EventuallyConsistent => "eventually-consistent",
            Self::StronglyConsistent => "strongly-consistent",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for ConsistencyLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A read concern label.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReadConcern(ConsistencyLevel);

impl ReadConcern {
    /// Creates a read concern.
    pub const fn new(level: ConsistencyLevel) -> Self {
        Self(level)
    }

    /// Returns the consistency level.
    pub const fn level(self) -> ConsistencyLevel {
        self.0
    }
}

impl fmt::Display for ReadConcern {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// A write concern label.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WriteConcern(ConsistencyLevel);

impl WriteConcern {
    /// Creates a write concern.
    pub const fn new(level: ConsistencyLevel) -> Self {
        Self(level)
    }

    /// Returns the consistency level.
    pub const fn level(self) -> ConsistencyLevel {
        self.0
    }
}

impl fmt::Display for WriteConcern {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Durability labels for write persistence modeling.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DurabilityLevel {
    Memory,
    Disk,
    Replicated,
    Majority,
    #[default]
    Unknown,
}

impl DurabilityLevel {
    /// Returns a stable lowercase label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Memory => "memory",
            Self::Disk => "disk",
            Self::Replicated => "replicated",
            Self::Majority => "majority",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for DurabilityLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A replication factor count.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReplicationFactor(u16);

impl ReplicationFactor {
    /// Creates a replication factor.
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    /// Returns the replication factor value.
    pub const fn value(self) -> u16 {
        self.0
    }
}

impl fmt::Display for ReplicationFactor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A quorum count.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Quorum(u16);

impl Quorum {
    /// Creates a quorum count.
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    /// Returns the quorum value.
    pub const fn value(self) -> u16 {
        self.0
    }
}

impl fmt::Display for Quorum {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ConsistencyLevel, DurabilityLevel, Quorum, ReadConcern, ReplicationFactor, WriteConcern,
    };

    #[test]
    fn formats_consistency_levels() {
        assert_eq!(ConsistencyLevel::One.to_string(), "one");
        assert_eq!(ConsistencyLevel::Quorum.to_string(), "quorum");
        assert_eq!(ConsistencyLevel::LocalQuorum.to_string(), "local-quorum");
        assert_eq!(
            ConsistencyLevel::EventuallyConsistent.to_string(),
            "eventually-consistent"
        );
        assert_eq!(
            ConsistencyLevel::StronglyConsistent.to_string(),
            "strongly-consistent"
        );
        assert_eq!(ConsistencyLevel::Unknown.to_string(), "unknown");
    }

    #[test]
    fn builds_concern_and_replication_labels() {
        let read = ReadConcern::new(ConsistencyLevel::Quorum);
        let write = WriteConcern::new(ConsistencyLevel::All);
        let replication = ReplicationFactor::new(3);
        let quorum = Quorum::new(2);

        assert_eq!(read.to_string(), "quorum");
        assert_eq!(write.level(), ConsistencyLevel::All);
        assert_eq!(replication.value(), 3);
        assert_eq!(quorum.to_string(), "2");
        assert_eq!(DurabilityLevel::Replicated.to_string(), "replicated");
    }
}
