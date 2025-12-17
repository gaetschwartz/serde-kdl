mod de;
mod error;
mod ser;

#[cfg(feature = "bytes")]
mod hex;

#[cfg(feature = "bytes")]
pub mod hex_serde;

pub use de::Deserializer;
pub use error::{Error, Result};
pub use ser::Serializer;

/// Default node name used when serializing primitive values.
pub const DEFAULT_NODE_NAME: &str = "-";

// Re-export kdl types for macro usage
#[doc(hidden)]
pub mod private {
    pub use kdl;
}

// Re-export the procedural macro
#[cfg(feature = "macros")]
pub use serde_kdl_macro::kdl;

/// Serialize a value to a KDL string.
///
/// This function serializes the given value to a KDL document and returns it as a string.
/// The resulting KDL can be parsed by any KDL parser and deserialized back using [`from_str`].
///
/// # Examples
///
/// Basic value serialization:
/// ```
/// use serde_kdl::to_string;
///
/// let value = 42;
/// let kdl_string = to_string(&value).unwrap();
/// assert_eq!(kdl_string, "- 42\n");
/// ```
///
/// Struct serialization:
/// ```
/// use serde::{Serialize};
/// use serde_kdl::to_string;
///
/// #[derive(Serialize)]
/// struct Config {
///     name: String,
///     port: u16,
///     enabled: bool,
/// }
///
/// let config = Config {
///     name: "my-service".to_string(),
///     port: 8080,
///     enabled: true,
/// };
///
/// let kdl_string = to_string(&config).unwrap();
/// assert_eq!(kdl_string, "\
/// name my-service
/// port 8080
/// enabled #true
/// ");
/// ```
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: serde::Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.into_document().to_string())
}

/// Serialize a value to a pretty-printed KDL string.
///
/// This function serializes the given value to a KDL document, applies automatic
/// formatting for readability, and returns it as a string. The output includes
/// proper indentation for nested structures.
///
/// # Examples
///
/// ```
/// use serde::Serialize;
/// use serde_kdl::to_pretty_string;
///
/// #[derive(Serialize)]
/// struct Server {
///     host: String,
///     port: u16,
/// }
///
/// #[derive(Serialize)]
/// struct Config {
///     server: Server,
///     debug: bool,
/// }
///
/// let config = Config {
///     server: Server {
///         host: "localhost".to_string(),
///         port: 8080,
///     },
///     debug: true,
/// };
///
/// let pretty = to_pretty_string(&config).unwrap();
/// assert_eq!(pretty, "\
/// server host=localhost port=8080
/// debug #true
/// ");
/// ```
///
/// Compare with [`to_string`] which produces compact output without formatting.
pub fn to_pretty_string<T>(value: &T) -> Result<String>
where
    T: serde::Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    let mut document = serializer.into_document();
    document.autoformat();
    Ok(document.to_string())
}

/// Serialize a value to a KDL document.
///
/// This function serializes the given value directly to a `kdl::KdlDocument`.
/// The resulting document can be manipulated, combined with other documents,
/// or used with [`from_document`] for deserialization.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use serde_kdl::to_document;
///
/// let value = 42;
/// let document = to_document(&value).unwrap();
/// assert_eq!(document.nodes().len(), 1);
/// ```
///
/// Working with the document directly:
/// ```
/// use serde_kdl::{to_document, kdl};
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Database {
///     host: String,
///     port: u16,
/// }
///
/// let db = Database {
///     host: "localhost".to_string(),
///     port: 5432,
/// };
///
/// // Serialize to document
/// let mut doc = to_document(&db).unwrap();
///
/// // Combine with macro-generated document
/// let extra_config = kdl! {
///     cache ttl=300 max_entries=1000
/// };
///
/// // Add nodes from the macro document
/// for node in extra_config.nodes() {
///     doc.nodes_mut().push(node.clone());
/// }
///
/// // Now doc contains both serialized struct and macro content
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
/// The string can come from any source, including output from [`to_string`] or
/// KDL documents created with the [`kdl!`] macro.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use serde_kdl::from_str;
///
/// let kdl_string = "- 42";
/// let value: i32 = from_str(kdl_string).unwrap();
/// assert_eq!(value, 42);
/// ```
///
/// Deserializing structs:
/// ```
/// use serde::Deserialize;
/// use serde_kdl::from_str;
///
/// #[derive(Deserialize, Debug, PartialEq)]
/// struct Config {
///     name: String,
///     port: u16,
///     enabled: bool,
/// }
///
/// let kdl_string = r#"Config name="web-server" port=8080 enabled=#true"#;
/// let config: Config = from_str(kdl_string).unwrap();
///
/// assert_eq!(config.name, "web-server");
/// assert_eq!(config.port, 8080);
/// assert_eq!(config.enabled, true);
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
/// This is especially useful when working with documents created by the [`kdl!`]
/// macro or when you need to manipulate documents before deserialization.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use serde_kdl::from_document;
/// use kdl::KdlDocument;
///
/// let document: KdlDocument = "- 42".parse().unwrap();
/// let value: i32 = from_document(&document).unwrap();
/// assert_eq!(value, 42);
/// ```
///
/// Working with the [`kdl!`] macro:
/// ```
/// use serde_kdl::{kdl, from_document};
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Debug, PartialEq)]
/// struct AppSettings {
///     name: String,
///     debug: bool,
///     workers: u32,
/// }
///
/// // Create document with macro
/// let doc = kdl! {
///     AppSettings name="my-app" debug=true workers=4
/// };
///
/// // Deserialize directly from the document
/// let settings: AppSettings = from_document(&doc).unwrap();
/// assert_eq!(settings.name, "my-app");
/// assert_eq!(settings.debug, true);
/// assert_eq!(settings.workers, 4);
/// ```
///
/// Combining with document manipulation:
/// ```
/// use serde_kdl::{kdl, from_document, to_document};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct Config {
///     name: String,
///     version: String,
/// }
///
/// // Start with macro-generated base
/// let mut doc = kdl! {
///     Config name="base-app" version="1.0.0"
/// };
///
/// // You can deserialize from the complete document
/// let config: Config = from_document(&doc).unwrap();
/// assert_eq!(config.name, "base-app");
/// assert_eq!(config.version, "1.0.0");
/// ```
pub fn from_document<T>(document: &kdl::KdlDocument) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let mut deserializer = Deserializer::new(document);
    T::deserialize(&mut deserializer)
}
