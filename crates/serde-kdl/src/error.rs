use std::fmt;
use thiserror::Error;

/// Errors that can occur during KDL serialization or deserialization.
#[derive(Error, Debug)]
pub enum Error {
    #[error("serde error: {0}")]
    Serde(String),

    #[error("KDL parse error: {0}")]
    KdlParse(#[from] kdl::KdlError),

    #[error("expected node named '{expected}', found '{found}'")]
    UnexpectedNodeName { expected: String, found: String },

    #[error("missing required field '{field}' in node '{node}'")]
    MissingField { field: String, node: String },

    #[error("unexpected value type for field '{field}': expected {expected}, found {found}")]
    UnexpectedValueType {
        field: String,
        expected: String,
        found: String,
    },

    #[error("missing root node")]
    MissingRootNode,

    #[error("expected exactly one root node, found {count}")]
    MultipleRootNodes { count: usize },

    #[error("unsupported type: {0}")]
    UnsupportedType(String),

    #[error("invalid sequence index: {0}")]
    InvalidSequenceIndex(usize),
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Serde(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Serde(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
