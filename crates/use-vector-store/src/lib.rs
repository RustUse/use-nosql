#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
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
    /// A vector record identifier.
    VectorId
}
string_newtype! {
    /// A vector collection name.
    VectorCollectionName
}

/// A vector dimension count.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VectorDimension(usize);

impl VectorDimension {
    /// Creates a vector dimension count.
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    /// Returns the dimension count.
    pub const fn value(self) -> usize {
        self.0
    }
}

impl fmt::Display for VectorDimension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A vector embedding payload.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Embedding(Vec<f32>);

impl Embedding {
    /// Creates an embedding from components.
    pub fn new(values: Vec<f32>) -> Self {
        Self(values)
    }

    /// Returns embedding components.
    pub fn values(&self) -> &[f32] {
        &self.0
    }

    /// Returns the embedding dimension.
    pub fn dimension(&self) -> VectorDimension {
        VectorDimension::new(self.0.len())
    }

    /// Returns whether the embedding has no components.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Error returned when a vector dimension does not match its embedding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidDimensionError {
    expected: VectorDimension,
    actual: VectorDimension,
}

impl InvalidDimensionError {
    /// Creates a dimension mismatch error.
    pub const fn new(expected: VectorDimension, actual: VectorDimension) -> Self {
        Self { expected, actual }
    }

    /// Returns the expected dimension.
    pub const fn expected(self) -> VectorDimension {
        self.expected
    }

    /// Returns the actual dimension.
    pub const fn actual(self) -> VectorDimension {
        self.actual
    }
}

impl fmt::Display for InvalidDimensionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "vector dimension mismatch: expected {}, got {}",
            self.expected, self.actual
        )
    }
}

impl Error for InvalidDimensionError {}

/// Similarity metric labels used by vector stores.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SimilarityMetric {
    Cosine,
    DotProduct,
    Euclidean,
    Manhattan,
    Hamming,
    #[default]
    Unknown,
}

impl SimilarityMetric {
    /// Returns a stable lowercase label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cosine => "cosine",
            Self::DotProduct => "dot-product",
            Self::Euclidean => "euclidean",
            Self::Manhattan => "manhattan",
            Self::Hamming => "hamming",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for SimilarityMetric {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// String metadata attached to a vector record.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VectorMetadata {
    entries: Vec<(String, String)>,
}

impl VectorMetadata {
    /// Creates empty metadata.
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Adds a metadata entry.
    pub fn with_entry(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.entries.push((key.into(), value.into()));
        self
    }

    /// Returns metadata entries.
    pub fn entries(&self) -> &[(String, String)] {
        &self.entries
    }
}

/// A vector store record.
#[derive(Clone, Debug, PartialEq)]
pub struct VectorRecord {
    id: VectorId,
    embedding: Embedding,
    dimension: Option<VectorDimension>,
    similarity_metric: Option<SimilarityMetric>,
    metadata: VectorMetadata,
}

impl VectorRecord {
    /// Creates a vector record.
    pub fn new(id: VectorId, embedding: Embedding) -> Self {
        Self {
            id,
            embedding,
            dimension: None,
            similarity_metric: None,
            metadata: VectorMetadata::new(),
        }
    }

    /// Sets and validates the expected dimension.
    pub fn with_dimension(
        mut self,
        dimension: VectorDimension,
    ) -> Result<Self, InvalidDimensionError> {
        let actual = self.embedding.dimension();
        if dimension != actual {
            return Err(InvalidDimensionError::new(dimension, actual));
        }
        self.dimension = Some(dimension);
        Ok(self)
    }

    /// Sets the similarity metric.
    pub const fn with_similarity_metric(mut self, similarity_metric: SimilarityMetric) -> Self {
        self.similarity_metric = Some(similarity_metric);
        self
    }

    /// Sets vector metadata.
    pub fn with_metadata(mut self, metadata: VectorMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Returns the record identifier.
    pub const fn id(&self) -> &VectorId {
        &self.id
    }

    /// Returns the embedding.
    pub const fn embedding(&self) -> &Embedding {
        &self.embedding
    }

    /// Returns the validated dimension, if present.
    pub const fn dimension(&self) -> Option<VectorDimension> {
        self.dimension
    }

    /// Returns the similarity metric, if present.
    pub const fn similarity_metric(&self) -> Option<SimilarityMetric> {
        self.similarity_metric
    }

    /// Returns vector metadata.
    pub const fn metadata(&self) -> &VectorMetadata {
        &self.metadata
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Embedding, InvalidDimensionError, SimilarityMetric, VectorCollectionName, VectorDimension,
        VectorId, VectorMetadata, VectorRecord,
    };

    #[test]
    fn constructs_vector_labels_and_embedding() {
        let id = VectorId::new("review_embedding");
        let collection = VectorCollectionName::new("reviews");
        let embedding = Embedding::new(vec![0.1, 0.2, 0.3]);

        assert_eq!(id.to_string(), "review_embedding");
        assert_eq!(collection.as_ref(), "reviews");
        assert_eq!(embedding.dimension(), VectorDimension::new(3));
    }

    #[test]
    fn validates_vector_dimensions() -> Result<(), InvalidDimensionError> {
        let metadata = VectorMetadata::new().with_entry("source", "review");
        let record = VectorRecord::new(VectorId::new("review_1"), Embedding::new(vec![1.0, 0.0]))
            .with_dimension(VectorDimension::new(2))?
            .with_similarity_metric(SimilarityMetric::Cosine)
            .with_metadata(metadata);

        assert_eq!(record.dimension(), Some(VectorDimension::new(2)));
        assert_eq!(record.similarity_metric(), Some(SimilarityMetric::Cosine));
        assert_eq!(record.metadata().entries().len(), 1);
        assert_eq!(SimilarityMetric::DotProduct.to_string(), "dot-product");

        Ok(())
    }

    #[test]
    fn rejects_dimension_mismatches() {
        let result = VectorRecord::new(VectorId::new("review_1"), Embedding::new(vec![1.0, 0.0]))
            .with_dimension(VectorDimension::new(3));

        assert_eq!(
            result,
            Err(InvalidDimensionError::new(
                VectorDimension::new(3),
                VectorDimension::new(2)
            ))
        );
    }
}
