#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Thin facade for primitive NoSQL data modeling crates.

#[cfg(feature = "cache-store")]
pub use use_cache_store as cache;
#[cfg(feature = "change-event")]
pub use use_change_event as change_event;
#[cfg(feature = "consistency")]
pub use use_consistency as consistency;
#[cfg(feature = "document-path")]
pub use use_document_path as document_path;
#[cfg(feature = "document-store")]
pub use use_document_store as document_store;
#[cfg(feature = "graph-store")]
pub use use_graph_store as graph_store;
#[cfg(feature = "key-value-store")]
pub use use_key_value_store as key_value;
#[cfg(feature = "partition-key")]
pub use use_partition_key as partition_key;
#[cfg(feature = "search-index")]
pub use use_search_index as search_index;
#[cfg(feature = "timeseries-store")]
pub use use_timeseries_store as timeseries;
#[cfg(feature = "vector-store")]
pub use use_vector_store as vector;
#[cfg(feature = "wide-column")]
pub use use_wide_column as wide_column;

#[cfg(feature = "cache-store")]
pub use use_cache_store::{
    CacheEntry, CacheKey, CacheKeyBuilder, CacheNamespace, CacheStatus, CacheValue, EvictionPolicy,
    Expiration, InvalidTtlError, Ttl,
};
#[cfg(feature = "change-event")]
pub use use_change_event::{
    ChangeCursor, ChangeEvent, ChangeEventId, ChangeEventKind, ChangeSequence, ChangedDocument,
    ResumeToken,
};
#[cfg(feature = "consistency")]
pub use use_consistency::{
    ConsistencyLevel, DurabilityLevel, Quorum, ReadConcern, ReplicationFactor, WriteConcern,
};
#[cfg(feature = "document-path")]
pub use use_document_path::{DocumentPath, FieldSelector, PathParseError, PathSegment};
#[cfg(feature = "document-store")]
pub use use_document_store::{
    CollectionName, DocumentField, DocumentId, DocumentMetadata, DocumentPatch, DocumentRevision,
    DocumentVersion, PatchOperation, PatchSet,
};
#[cfg(feature = "graph-store")]
pub use use_graph_store::{
    EdgeId, EdgeLabel, GraphEdge, GraphProperty, GraphVertex, PropertyKey, PropertyValue,
    TraversalLabel, VertexId, VertexLabel,
};
#[cfg(feature = "key-value-store")]
pub use use_key_value_store::{
    BucketName, Key, KeyNamespace, KeyPattern, KeyPrefix, KeyRange, KeyValueEntry, Value,
};
#[cfg(feature = "partition-key")]
pub use use_partition_key::{
    CompositeKey, PartitionKey, PartitionStrategy, RoutingKey, ShardKey, SortKey,
};
#[cfg(feature = "search-index")]
pub use use_search_index::{
    IndexName, SearchAnalyzer, SearchDocumentId, SearchField, SearchFilter, SearchIndexDocument,
    SearchQueryShape, SearchSort, SearchTerm,
};
#[cfg(feature = "timeseries-store")]
pub use use_timeseries_store::{
    AggregationWindow, MetricName, RetentionWindow, SamplingInterval, SeriesId, TimeSeriesPoint,
    TimeSeriesValue, Timestamp,
};
#[cfg(feature = "vector-store")]
pub use use_vector_store::{
    Embedding, InvalidDimensionError, SimilarityMetric, VectorCollectionName, VectorDimension,
    VectorId, VectorMetadata, VectorRecord,
};
#[cfg(feature = "wide-column")]
pub use use_wide_column::{
    ClusteringKey, ColumnFamily, ColumnFamilyName, ColumnName, ColumnValue, KeyspaceName,
    PartitionKey as WideColumnPartitionKey, WideColumnRow, WideTableName,
};
