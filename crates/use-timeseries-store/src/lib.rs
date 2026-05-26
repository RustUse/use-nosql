#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
    /// A time-series identifier.
    SeriesId
}
string_newtype! {
    /// A metric name.
    MetricName
}

/// A timestamp backed by `std::time::SystemTime`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Timestamp(SystemTime);

impl Timestamp {
    /// Creates a timestamp.
    pub const fn new(value: SystemTime) -> Self {
        Self(value)
    }

    /// Creates a timestamp for the current system time.
    pub fn now() -> Self {
        Self(SystemTime::now())
    }

    /// Returns the stored system time.
    pub const fn system_time(self) -> SystemTime {
        self.0
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.duration_since(UNIX_EPOCH) {
            Ok(duration) => write!(formatter, "{}", duration.as_secs()),
            Err(error) => write!(formatter, "-{}", error.duration().as_secs()),
        }
    }
}

/// A time-series numeric value.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct TimeSeriesValue(f64);

impl TimeSeriesValue {
    /// Creates a time-series value.
    pub const fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the stored value.
    pub const fn value(self) -> f64 {
        self.0
    }
}

impl fmt::Display for TimeSeriesValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A single time-series point.
#[derive(Clone, Debug, PartialEq)]
pub struct TimeSeriesPoint {
    series_id: SeriesId,
    metric_name: MetricName,
    timestamp: Timestamp,
    value: TimeSeriesValue,
}

impl TimeSeriesPoint {
    /// Creates a time-series point.
    pub fn new(
        series_id: SeriesId,
        metric_name: MetricName,
        timestamp: Timestamp,
        value: TimeSeriesValue,
    ) -> Self {
        Self {
            series_id,
            metric_name,
            timestamp,
            value,
        }
    }

    /// Returns the series identifier.
    pub const fn series_id(&self) -> &SeriesId {
        &self.series_id
    }

    /// Returns the metric name.
    pub const fn metric_name(&self) -> &MetricName {
        &self.metric_name
    }

    /// Returns the timestamp.
    pub const fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// Returns the value.
    pub const fn value(&self) -> TimeSeriesValue {
        self.value
    }
}

/// A retention duration.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RetentionWindow(Duration);

impl RetentionWindow {
    /// Creates a retention window.
    pub const fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Returns the duration.
    pub const fn duration(self) -> Duration {
        self.0
    }
}

/// A sampling interval duration.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SamplingInterval(Duration);

impl SamplingInterval {
    /// Creates a sampling interval.
    pub const fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Returns the duration.
    pub const fn duration(self) -> Duration {
        self.0
    }
}

/// An aggregation window duration.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AggregationWindow(Duration);

impl AggregationWindow {
    /// Creates an aggregation window.
    pub const fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Returns the duration.
    pub const fn duration(self) -> Duration {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AggregationWindow, MetricName, RetentionWindow, SamplingInterval, SeriesId,
        TimeSeriesPoint, TimeSeriesValue, Timestamp,
    };
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn constructs_time_series_labels() {
        assert_eq!(SeriesId::new("cpu:host-1").to_string(), "cpu:host-1");
        assert_eq!(MetricName::new("cpu.usage").as_ref(), "cpu.usage");
    }

    #[test]
    fn builds_points_and_windows() {
        let point = TimeSeriesPoint::new(
            SeriesId::new("host_1"),
            MetricName::new("cpu.usage"),
            Timestamp::new(UNIX_EPOCH + Duration::from_secs(10)),
            TimeSeriesValue::new(0.75),
        );
        let retention = RetentionWindow::new(Duration::from_hours(24));
        let sampling = SamplingInterval::new(Duration::from_mins(1));
        let aggregation = AggregationWindow::new(Duration::from_mins(5));

        assert_eq!(point.timestamp().to_string(), "10");
        assert_eq!(point.value().to_string(), "0.75");
        assert_eq!(retention.duration().as_secs(), 86_400);
        assert_eq!(sampling.duration().as_secs(), 60);
        assert_eq!(aggregation.duration().as_secs(), 300);
    }
}
