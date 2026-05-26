#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

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
    /// A validated or raw document path such as `profile.display_name`.
    DocumentPath
}
string_newtype! {
    /// One segment inside a dot-separated document path.
    PathSegment
}
string_newtype! {
    /// A field selector label for extracting document fields.
    FieldSelector
}

impl DocumentPath {
    /// Creates a document path after validating dot-path structure.
    pub fn try_new(value: impl AsRef<str>) -> Result<Self, PathParseError> {
        validate_path(value.as_ref())?;
        Ok(Self::new(value.as_ref().trim()))
    }

    /// Parses the path into individual segments.
    pub fn segments(&self) -> Result<Vec<PathSegment>, PathParseError> {
        parse_segments(self.as_str())
    }
}

impl FromStr for DocumentPath {
    type Err = PathParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::try_new(input)
    }
}

/// Error returned when a document dot-path is invalid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PathParseError {
    /// The path had no non-whitespace content.
    EmptyPath,
    /// The path contained an empty segment at the given zero-based index.
    EmptySegment { index: usize },
}

impl fmt::Display for PathParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPath => formatter.write_str("document path must not be empty"),
            Self::EmptySegment { index } => {
                write!(
                    formatter,
                    "document path segment at index {index} must not be empty"
                )
            },
        }
    }
}

impl Error for PathParseError {}

fn validate_path(input: &str) -> Result<(), PathParseError> {
    parse_segments(input).map(|_| ())
}

fn parse_segments(input: &str) -> Result<Vec<PathSegment>, PathParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(PathParseError::EmptyPath);
    }

    let mut segments = Vec::new();
    for (index, segment) in trimmed.split('.').enumerate() {
        if segment.is_empty() {
            return Err(PathParseError::EmptySegment { index });
        }
        segments.push(PathSegment::new(segment));
    }

    Ok(segments)
}

#[cfg(test)]
mod tests {
    use super::{DocumentPath, FieldSelector, PathParseError, PathSegment};

    #[test]
    fn constructs_and_displays_paths() {
        let path = DocumentPath::new("profile.display_name");
        assert_eq!(path.as_str(), "profile.display_name");
        assert_eq!(path.to_string(), "profile.display_name");
        assert_eq!(path.as_ref(), "profile.display_name");
        assert_eq!(DocumentPath::from("profile"), DocumentPath::new("profile"));
    }

    #[test]
    fn parses_dot_path_segments() -> Result<(), PathParseError> {
        let path = DocumentPath::try_new("settings.notifications.email")?;
        let segments = path.segments()?;

        assert_eq!(
            segments,
            vec![
                PathSegment::new("settings"),
                PathSegment::new("notifications"),
                PathSegment::new("email"),
            ]
        );
        assert_eq!(
            FieldSelector::new("profile.display_name").to_string(),
            "profile.display_name"
        );

        Ok(())
    }

    #[test]
    fn rejects_invalid_paths() {
        assert_eq!(DocumentPath::try_new(""), Err(PathParseError::EmptyPath));
        assert_eq!(
            DocumentPath::try_new("profile..name"),
            Err(PathParseError::EmptySegment { index: 1 })
        );
    }
}
