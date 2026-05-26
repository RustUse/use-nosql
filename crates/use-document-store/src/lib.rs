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
    /// A document collection name.
    CollectionName
}
string_newtype! {
    /// A document identifier.
    DocumentId
}
string_newtype! {
    /// A document field label.
    DocumentField
}

/// A monotonically increasing document revision label.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DocumentRevision(u64);

impl DocumentRevision {
    /// Creates a document revision value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the revision value.
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for DocumentRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A document schema or application version label.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DocumentVersion(u64);

impl DocumentVersion {
    /// Creates a document version value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the version value.
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for DocumentVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Vendor-neutral metadata for a modeled document.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DocumentMetadata {
    collection: Option<CollectionName>,
    revision: Option<DocumentRevision>,
    version: Option<DocumentVersion>,
}

impl DocumentMetadata {
    /// Creates empty document metadata.
    pub const fn new() -> Self {
        Self {
            collection: None,
            revision: None,
            version: None,
        }
    }

    /// Sets the collection name.
    pub fn with_collection(mut self, collection: CollectionName) -> Self {
        self.collection = Some(collection);
        self
    }

    /// Sets the document revision.
    pub const fn with_revision(mut self, revision: DocumentRevision) -> Self {
        self.revision = Some(revision);
        self
    }

    /// Sets the document version.
    pub const fn with_version(mut self, version: DocumentVersion) -> Self {
        self.version = Some(version);
        self
    }

    /// Returns the collection name, if present.
    pub const fn collection(&self) -> Option<&CollectionName> {
        self.collection.as_ref()
    }

    /// Returns the revision, if present.
    pub const fn revision(&self) -> Option<DocumentRevision> {
        self.revision
    }

    /// Returns the version, if present.
    pub const fn version(&self) -> Option<DocumentVersion> {
        self.version
    }
}

/// A single document patch operation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PatchOperation {
    Set { path: String, value: String },
    Unset { path: String },
    Remove { path: String },
    Increment { path: String, amount: i64 },
    Append { path: String, value: String },
    Prepend { path: String, value: String },
    Merge { path: String, value: String },
    Replace { path: String, value: String },
}

impl PatchOperation {
    /// Creates a set operation.
    pub fn set(path: impl AsRef<str>, value: impl Into<String>) -> Self {
        Self::Set {
            path: path.as_ref().to_owned(),
            value: value.into(),
        }
    }

    /// Creates an unset operation.
    pub fn unset(path: impl AsRef<str>) -> Self {
        Self::Unset {
            path: path.as_ref().to_owned(),
        }
    }

    /// Creates a remove operation.
    pub fn remove(path: impl AsRef<str>) -> Self {
        Self::Remove {
            path: path.as_ref().to_owned(),
        }
    }

    /// Creates an increment operation.
    pub fn increment(path: impl AsRef<str>, amount: i64) -> Self {
        Self::Increment {
            path: path.as_ref().to_owned(),
            amount,
        }
    }

    /// Creates an append operation.
    pub fn append(path: impl AsRef<str>, value: impl Into<String>) -> Self {
        Self::Append {
            path: path.as_ref().to_owned(),
            value: value.into(),
        }
    }

    /// Creates a prepend operation.
    pub fn prepend(path: impl AsRef<str>, value: impl Into<String>) -> Self {
        Self::Prepend {
            path: path.as_ref().to_owned(),
            value: value.into(),
        }
    }

    /// Creates a merge operation.
    pub fn merge(path: impl AsRef<str>, value: impl Into<String>) -> Self {
        Self::Merge {
            path: path.as_ref().to_owned(),
            value: value.into(),
        }
    }

    /// Creates a replace operation.
    pub fn replace(path: impl AsRef<str>, value: impl Into<String>) -> Self {
        Self::Replace {
            path: path.as_ref().to_owned(),
            value: value.into(),
        }
    }

    /// Returns the operation path.
    pub fn path(&self) -> &str {
        match self {
            Self::Set { path, .. }
            | Self::Unset { path }
            | Self::Remove { path }
            | Self::Increment { path, .. }
            | Self::Append { path, .. }
            | Self::Prepend { path, .. }
            | Self::Merge { path, .. }
            | Self::Replace { path, .. } => path,
        }
    }
}

/// A set of document patch operations.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PatchSet {
    operations: Vec<PatchOperation>,
}

impl PatchSet {
    /// Creates a patch set from operations.
    pub fn new(operations: Vec<PatchOperation>) -> Self {
        Self { operations }
    }

    /// Returns the patch operations.
    pub fn operations(&self) -> &[PatchOperation] {
        &self.operations
    }

    /// Adds one operation.
    pub fn with_operation(mut self, operation: PatchOperation) -> Self {
        self.operations.push(operation);
        self
    }
}

/// Patch operations addressed to one document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentPatch {
    document_id: DocumentId,
    patch_set: PatchSet,
}

impl DocumentPatch {
    /// Creates a document patch.
    pub fn new(document_id: DocumentId, patch_set: PatchSet) -> Self {
        Self {
            document_id,
            patch_set,
        }
    }

    /// Returns the patched document identifier.
    pub const fn document_id(&self) -> &DocumentId {
        &self.document_id
    }

    /// Returns the patch set.
    pub const fn patch_set(&self) -> &PatchSet {
        &self.patch_set
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CollectionName, DocumentId, DocumentMetadata, DocumentPatch, DocumentRevision,
        DocumentVersion, PatchOperation, PatchSet,
    };
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn constructs_document_newtypes() {
        let collection = CollectionName::new("customers");
        let document_id = DocumentId::new("customer_123");
        assert_eq!(collection.to_string(), "customers");
        assert_eq!(document_id.as_ref(), "customer_123");
    }

    #[test]
    fn hashes_equal_document_ids() {
        let mut left = DefaultHasher::new();
        let mut right = DefaultHasher::new();
        DocumentId::new("same").hash(&mut left);
        DocumentId::new("same").hash(&mut right);
        assert_eq!(left.finish(), right.finish());
    }

    #[test]
    fn builds_metadata_and_patch_operations() {
        let metadata = DocumentMetadata::new()
            .with_collection(CollectionName::new("customers"))
            .with_revision(DocumentRevision::new(7))
            .with_version(DocumentVersion::new(2));
        let patch = PatchSet::new(vec![
            PatchOperation::set("profile.display_name", "Joshua Whalen"),
            PatchOperation::increment("stats.reviews", 1),
        ]);
        let document_patch = DocumentPatch::new(DocumentId::new("customer_123"), patch);

        assert_eq!(metadata.collection().unwrap().as_str(), "customers");
        assert_eq!(metadata.revision(), Some(DocumentRevision::new(7)));
        assert_eq!(
            document_patch.patch_set().operations()[0].path(),
            "profile.display_name"
        );
    }
}
