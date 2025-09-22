//! Serde serialization and deserialization support for KDL (Kindly Document Language).
//!
//! This crate provides serde integration for KDL documents, allowing you to serialize
//! and deserialize Rust types to/from KDL format without intermediate representations.
//!
//! # Examples
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use serde_kdl::{to_string, from_str};
//!
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! struct Config {
//!     name: String,
//!     port: u16,
//!     debug: bool,
//! }
//!
//! let config = Config {
//!     name: "my-app".to_string(),
//!     port: 8080,
//!     debug: true,
//! };
//!
//! // Serialize to KDL string
//! let kdl_string = to_string(&config).unwrap();
//!
//! // Deserialize from KDL string
//! let deserialized: Config = from_str(&kdl_string).unwrap();
//!
//! assert_eq!(config, deserialized);
//! ```

mod error;
mod ser;
mod de;

#[cfg(feature = "bytes")]
mod hex;

#[cfg(feature = "bytes")]
pub mod hex_serde;

pub use error::{Error, Result};
pub use ser::Serializer;
pub use de::Deserializer;

/// Serialize a value to a KDL string.
///
/// This function serializes the given value to a KDL document and returns it as a string.
///
/// # Examples
///
/// ```
/// use serde_kdl::to_string;
///
/// let value = 42;
/// let kdl_string = to_string(&value).unwrap();
/// assert_eq!(kdl_string, "root 42\n");
/// ```
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: serde::Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.into_document().to_string())
}

/// Serialize a value to a KDL document.
///
/// This function serializes the given value directly to a `kdl::KdlDocument`.
///
/// # Examples
///
/// ```
/// use serde_kdl::to_document;
///
/// let value = 42;
/// let document = to_document(&value).unwrap();
/// ```
pub fn to_document<T>(value: &T) -> Result<kdl::KdlDocument>
where
    T: serde::Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.into_document())
}

/// Deserialize a value from a KDL string.
///
/// This function parses the KDL string and deserializes it to the target type.
///
/// # Examples
///
/// ```
/// use serde_kdl::from_str;
///
/// let kdl_string = "root 42";
/// let value: i32 = from_str(kdl_string).unwrap();
/// assert_eq!(value, 42);
/// ```
pub fn from_str<T>(s: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let document: kdl::KdlDocument = s.parse()?;
    from_document(&document)
}

/// Deserialize a value from a KDL document.
///
/// This function deserializes the given `kdl::KdlDocument` to the target type.
///
/// # Examples
///
/// ```
/// use serde_kdl::from_document;
/// use kdl::KdlDocument;
///
/// let document: KdlDocument = "root 42".parse().unwrap();
/// let value: i32 = from_document(&document).unwrap();
/// assert_eq!(value, 42);
/// ```
pub fn from_document<T>(document: &kdl::KdlDocument) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let mut deserializer = Deserializer::new(document);
    T::deserialize(&mut deserializer)
}