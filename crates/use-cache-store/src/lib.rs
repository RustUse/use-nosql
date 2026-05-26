#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::error::Error;
use std::time::{Duration, SystemTime};

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
    /// A cache key.
    CacheKey
}
string_newtype! {
    /// A cache namespace.
    CacheNamespace
}
string_newtype! {
    /// A cache value payload.
    CacheValue
}

impl CacheKey {
    /// Creates a cache key builder.
    pub fn builder() -> CacheKeyBuilder {
        CacheKeyBuilder::new()
    }
}

/// Builder for composed cache keys.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CacheKeyBuilder {
    namespace: Option<CacheNamespace>,
    segments: Vec<String>,
}

impl CacheKeyBuilder {
    /// Creates an empty cache key builder.
    pub const fn new() -> Self {
        Self {
            namespace: None,
            segments: Vec::new(),
        }
    }

    /// Sets the namespace prefix.
    pub fn namespace(mut self, namespace: CacheNamespace) -> Self {
        self.namespace = Some(namespace);
        self
    }

    /// Adds a non-empty segment.
    pub fn segment(mut self, segment: impl AsRef<str>) -> Self {
        let value = segment.as_ref();
        if !value.is_empty() {
            self.segments.push(value.to_owned());
        }
        self
    }

    /// Builds the cache key using `:` separators.
    pub fn build(self) -> CacheKey {
        let mut parts = Vec::new();
        if let Some(namespace) = self.namespace {
            parts.push(namespace.to_string());
        }
        parts.extend(self.segments);
        CacheKey::new(parts.join(":"))
    }
}

/// Error returned when a TTL value is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvalidTtlError {
    /// A TTL must be greater than zero.
    ZeroDuration,
}

impl fmt::Display for InvalidTtlError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroDuration => formatter.write_str("ttl duration must be greater than zero"),
        }
    }
}

impl Error for InvalidTtlError {}

/// A positive time-to-live duration.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Ttl(Duration);

impl Ttl {
    /// Creates a positive TTL.
    pub fn new(duration: Duration) -> Result<Self, InvalidTtlError> {
        if duration.is_zero() {
            Err(InvalidTtlError::ZeroDuration)
        } else {
            Ok(Self(duration))
        }
    }

    /// Creates a TTL from seconds.
    pub fn seconds(seconds: u64) -> Result<Self, InvalidTtlError> {
        Self::new(Duration::from_secs(seconds))
    }

    /// Returns the stored duration.
    pub const fn duration(self) -> Duration {
        self.0
    }
}

/// Cache expiration modeling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Expiration {
    Never,
    At(SystemTime),
    After(Ttl),
}

/// Common cache eviction labels.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EvictionPolicy {
    Lru,
    Lfu,
    Fifo,
    Ttl,
    Manual,
    #[default]
    Unknown,
}

impl EvictionPolicy {
    /// Returns a stable lowercase label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lru => "lru",
            Self::Lfu => "lfu",
            Self::Fifo => "fifo",
            Self::Ttl => "ttl",
            Self::Manual => "manual",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for EvictionPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Cache entry status labels.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CacheStatus {
    Hit,
    Miss,
    Stale,
    Expired,
    #[default]
    Unknown,
}

impl CacheStatus {
    /// Returns a stable lowercase label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hit => "hit",
            Self::Miss => "miss",
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for CacheStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A modeled cache entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CacheEntry {
    key: CacheKey,
    value: CacheValue,
    ttl: Option<Ttl>,
    status: CacheStatus,
}

impl CacheEntry {
    /// Creates a cache entry.
    pub const fn new(key: CacheKey, value: CacheValue) -> Self {
        Self {
            key,
            value,
            ttl: None,
            status: CacheStatus::Unknown,
        }
    }

    /// Sets the TTL.
    pub const fn with_ttl(mut self, ttl: Ttl) -> Self {
        self.ttl = Some(ttl);
        self
    }

    /// Sets the cache status.
    pub const fn with_status(mut self, status: CacheStatus) -> Self {
        self.status = status;
        self
    }

    /// Returns the key.
    pub const fn key(&self) -> &CacheKey {
        &self.key
    }

    /// Returns the value.
    pub const fn value(&self) -> &CacheValue {
        &self.value
    }

    /// Returns the TTL.
    pub const fn ttl(&self) -> Option<Ttl> {
        self.ttl
    }

    /// Returns the status.
    pub const fn status(&self) -> CacheStatus {
        self.status
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CacheEntry, CacheKey, CacheNamespace, CacheStatus, CacheValue, EvictionPolicy,
        InvalidTtlError, Ttl,
    };

    #[test]
    fn composes_cache_keys() {
        let key = CacheKey::builder()
            .namespace(CacheNamespace::new("reviews"))
            .segment("location")
            .segment("fort-wayne")
            .segment("summary")
            .build();

        assert_eq!(key.to_string(), "reviews:location:fort-wayne:summary");
    }

    #[test]
    fn validates_ttl_and_builds_entries() -> Result<(), InvalidTtlError> {
        let ttl = Ttl::seconds(60)?;
        let entry = CacheEntry::new(CacheKey::new("reviews:summary"), CacheValue::new("cached"))
            .with_ttl(ttl)
            .with_status(CacheStatus::Hit);

        assert_eq!(entry.ttl(), Some(ttl));
        assert_eq!(entry.status(), CacheStatus::Hit);
        assert_eq!(Ttl::seconds(0), Err(InvalidTtlError::ZeroDuration));
        assert_eq!(EvictionPolicy::Lru.to_string(), "lru");

        Ok(())
    }
}
