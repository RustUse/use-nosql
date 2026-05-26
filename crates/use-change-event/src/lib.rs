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
    /// A change event identifier.
    ChangeEventId
}
string_newtype! {
    /// A change stream cursor.
    ChangeCursor
}
string_newtype! {
    /// A change stream resume token.
    ResumeToken
}
string_newtype! {
    /// A changed document reference.
    ChangedDocument
}

/// A change sequence number.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ChangeSequence(u64);

impl ChangeSequence {
    /// Creates a change sequence value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the sequence value.
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ChangeSequence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Change event kind labels.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ChangeEventKind {
    Insert,
    Update,
    Delete,
    Replace,
    Upsert,
    Expire,
    #[default]
    Unknown,
}

impl ChangeEventKind {
    /// Returns a stable lowercase label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Insert => "insert",
            Self::Update => "update",
            Self::Delete => "delete",
            Self::Replace => "replace",
            Self::Upsert => "upsert",
            Self::Expire => "expire",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for ChangeEventKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A vendor-neutral change event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeEvent {
    id: ChangeEventId,
    kind: ChangeEventKind,
    document: ChangedDocument,
    cursor: Option<ChangeCursor>,
    resume_token: Option<ResumeToken>,
    sequence: Option<ChangeSequence>,
}

impl ChangeEvent {
    /// Creates a change event.
    pub fn new(id: ChangeEventId, kind: ChangeEventKind, document: impl Into<String>) -> Self {
        Self {
            id,
            kind,
            document: ChangedDocument::new(document),
            cursor: None,
            resume_token: None,
            sequence: None,
        }
    }

    /// Sets the cursor.
    pub fn with_cursor(mut self, cursor: ChangeCursor) -> Self {
        self.cursor = Some(cursor);
        self
    }

    /// Sets the resume token.
    pub fn with_resume_token(mut self, resume_token: ResumeToken) -> Self {
        self.resume_token = Some(resume_token);
        self
    }

    /// Sets the sequence.
    pub const fn with_sequence(mut self, sequence: ChangeSequence) -> Self {
        self.sequence = Some(sequence);
        self
    }

    /// Returns the event identifier.
    pub const fn id(&self) -> &ChangeEventId {
        &self.id
    }

    /// Returns the event kind.
    pub const fn kind(&self) -> ChangeEventKind {
        self.kind
    }

    /// Returns the changed document reference.
    pub const fn document(&self) -> &ChangedDocument {
        &self.document
    }

    /// Returns the cursor, if present.
    pub const fn cursor(&self) -> Option<&ChangeCursor> {
        self.cursor.as_ref()
    }

    /// Returns the resume token, if present.
    pub const fn resume_token(&self) -> Option<&ResumeToken> {
        self.resume_token.as_ref()
    }

    /// Returns the sequence, if present.
    pub const fn sequence(&self) -> Option<ChangeSequence> {
        self.sequence
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ChangeCursor, ChangeEvent, ChangeEventId, ChangeEventKind, ChangeSequence, ResumeToken,
    };

    #[test]
    fn formats_change_event_kinds() {
        assert_eq!(ChangeEventKind::Insert.to_string(), "insert");
        assert_eq!(ChangeEventKind::Update.to_string(), "update");
        assert_eq!(ChangeEventKind::Delete.to_string(), "delete");
        assert_eq!(ChangeEventKind::Unknown.to_string(), "unknown");
    }

    #[test]
    fn builds_change_events() {
        let event = ChangeEvent::new(
            ChangeEventId::new("evt_1"),
            ChangeEventKind::Replace,
            "customer_123",
        )
        .with_cursor(ChangeCursor::new("cursor_1"))
        .with_resume_token(ResumeToken::new("token_1"))
        .with_sequence(ChangeSequence::new(42));

        assert_eq!(event.id().as_str(), "evt_1");
        assert_eq!(event.kind(), ChangeEventKind::Replace);
        assert_eq!(event.document().as_ref(), "customer_123");
        assert_eq!(event.sequence(), Some(ChangeSequence::new(42)));
    }
}
