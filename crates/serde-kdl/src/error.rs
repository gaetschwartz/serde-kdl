#![allow(unused_assignments)]
use miette::Diagnostic;
use std::fmt;
use thiserror::Error;

/// Errors that can occur during KDL serialization or deserialization.
#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    /// A custom error from serde during deserialization.
    #[error("Deserialization error: {0}")]
    #[diagnostic(code(serde_kdl::de::custom))]
    De(String),

    /// A custom error from serde during serialization.
    #[error("Serialization error: {0}")]
    #[diagnostic(code(serde_kdl::ser::custom))]
    Ser(String),

    /// Error parsing KDL input.
    #[error("KDL parse error: {0}")]
    #[diagnostic(code(serde_kdl::parse))]
    KdlParse(#[from] kdl::KdlError),

    /// Expected a node with a specific name but found a different one.
    #[error("expected node named '{expected}', found '{found}'")]
    #[diagnostic(
        code(serde_kdl::unexpected_node_name),
        help("check that your KDL structure matches the expected schema")
    )]
    UnexpectedNodeName { expected: String, found: String },

    /// A required field is missing from a node.
    #[error("missing required field '{field}' in node '{node}'")]
    #[diagnostic(
        code(serde_kdl::missing_field),
        help("add the '{field}' field to the '{node}' node")
    )]
    MissingField { field: String, node: String },

    /// A field has an unexpected value type.
    #[error("unexpected value type for field '{field}': expected {expected}, found {found}")]
    #[diagnostic(
        code(serde_kdl::unexpected_type),
        help("change the value type to {expected}")
    )]
    UnexpectedValueType {
        field: String,
        expected: String,
        found: String,
    },

    /// The KDL document is missing a root node.
    #[error("missing root node")]
    #[diagnostic(
        code(serde_kdl::missing_root),
        help("ensure your KDL document has at least one node")
    )]
    MissingRootNode,

    /// The KDL document has more than one root node when only one was expected.
    #[error("expected exactly one root node, found {count}")]
    #[diagnostic(
        code(serde_kdl::multiple_roots),
        help("wrap multiple nodes in a parent node or deserialize to a sequence type")
    )]
    MultipleRootNodes { count: usize },

    /// The Rust type is not supported for KDL serialization.
    #[error("unsupported type: {0}")]
    #[diagnostic(
        code(serde_kdl::unsupported_type),
        help("see the serde-kdl documentation for supported types")
    )]
    UnsupportedType(String),

    /// An invalid sequence index was encountered.
    #[error("invalid sequence index: {0}")]
    #[diagnostic(code(serde_kdl::invalid_index))]
    InvalidSequenceIndex(usize),

    #[error("Map keys must serialize to strings in KDL. Found: {0:?}")]
    InvalidMapKey(Box<kdl::KdlNode>),

    #[error("missing key when serializing map")]
    KeyMissingInMapSerialization,
    /// Invalid hex string when deserializing bytes.
    #[cfg(feature = "bytes")]
    #[error("invalid hex string: {0}")]
    #[diagnostic(
        code(serde_kdl::invalid_hex),
        help("ensure the string contains only valid hexadecimal characters")
    )]
    InvalidHexString(String),

    /// No current node available for value deserialization.
    #[error("no current node for value")]
    #[diagnostic(code(serde_kdl::no_current_node))]
    NoCurrentNode,

    /// No current value available for field deserialization.
    #[error("no current value for field")]
    #[diagnostic(code(serde_kdl::no_current_value))]
    NoCurrentValue,
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Ser(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::De(msg.to_string())
    }
}

/// A specialized Result type for serde-kdl operations.
pub type Result<T> = std::result::Result<T, Error>;
