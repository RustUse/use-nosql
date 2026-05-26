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
    /// A search index name.
    IndexName
}
string_newtype! {
    /// A search index document identifier.
    SearchDocumentId
}
string_newtype! {
    /// A searchable field name.
    SearchField
}
string_newtype! {
    /// A search term.
    SearchTerm
}
string_newtype! {
    /// A search analyzer label.
    SearchAnalyzer
}
string_newtype! {
    /// A search filter label or expression shape.
    SearchFilter
}

/// A modeled search-index document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchIndexDocument {
    id: SearchDocumentId,
    fields: Vec<(SearchField, String)>,
}

impl SearchIndexDocument {
    /// Creates an empty search-index document.
    pub fn new(id: SearchDocumentId) -> Self {
        Self {
            id,
            fields: Vec::new(),
        }
    }

    /// Adds a field value.
    pub fn with_field(mut self, field: SearchField, value: impl Into<String>) -> Self {
        self.fields.push((field, value.into()));
        self
    }

    /// Returns the document identifier.
    pub const fn id(&self) -> &SearchDocumentId {
        &self.id
    }

    /// Returns field values.
    pub fn fields(&self) -> &[(SearchField, String)] {
        &self.fields
    }
}

/// A conservative search query shape label.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SearchQueryShape {
    Term,
    Phrase,
    Boolean,
    Vector,
    Hybrid,
    #[default]
    Unknown,
}

impl SearchQueryShape {
    /// Returns a stable lowercase label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Term => "term",
            Self::Phrase => "phrase",
            Self::Boolean => "boolean",
            Self::Vector => "vector",
            Self::Hybrid => "hybrid",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for SearchQueryShape {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A sort instruction for search results.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchSort {
    field: SearchField,
    descending: bool,
}

impl SearchSort {
    /// Creates an ascending sort.
    pub const fn ascending(field: SearchField) -> Self {
        Self {
            field,
            descending: false,
        }
    }

    /// Creates a descending sort.
    pub const fn descending(field: SearchField) -> Self {
        Self {
            field,
            descending: true,
        }
    }

    /// Returns the sort field.
    pub const fn field(&self) -> &SearchField {
        &self.field
    }

    /// Returns whether the sort is descending.
    pub const fn is_descending(&self) -> bool {
        self.descending
    }
}

#[cfg(test)]
mod tests {
    use super::{
        IndexName, SearchAnalyzer, SearchDocumentId, SearchField, SearchFilter,
        SearchIndexDocument, SearchQueryShape, SearchSort, SearchTerm,
    };

    #[test]
    fn constructs_search_labels() {
        assert_eq!(IndexName::new("reviews").to_string(), "reviews");
        assert_eq!(SearchTerm::new("plumber").as_ref(), "plumber");
        assert_eq!(SearchAnalyzer::new("standard").as_str(), "standard");
        assert_eq!(SearchFilter::new("rating >= 4").to_string(), "rating >= 4");
    }

    #[test]
    fn builds_documents_and_sorts() {
        let document = SearchIndexDocument::new(SearchDocumentId::new("review_1"))
            .with_field(SearchField::new("title"), "Great service");
        let sort = SearchSort::descending(SearchField::new("rating"));

        assert_eq!(document.fields().len(), 1);
        assert!(sort.is_descending());
        assert_eq!(SearchQueryShape::Hybrid.to_string(), "hybrid");
    }
}
