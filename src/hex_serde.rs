//! Custom serde module for serializing bytes as hex strings.
//!
//! **Note**: When the `bytes` feature is enabled (default), `Vec<u8>` and `&[u8]`
//! are automatically serialized as hex strings without requiring this module.
//! This module is kept for explicit control and backwards compatibility.
//!
//! # Default Behavior (with `bytes` feature)
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use serde_kdl::{from_str, to_string};
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq)]
//! struct Data {
//!     name: String,
//!     bytes: Vec<u8>,  // Automatically serialized as hex!
//! }
//!
//! let data = Data {
//!     name: "example".to_string(),
//!     bytes: vec![0xde, 0xad, 0xbe, 0xef],
//! };
//!
//! let kdl_string = to_string(&data).unwrap();
//! let parsed: Data = from_str(&kdl_string).unwrap();
//! assert_eq!(data, parsed);
//! ```
//!
//! # Explicit Control
//!
//! Use this module with the `#[serde(with = "serde_kdl::hex_serde")]` attribute
//! for explicit control or when you need backwards compatibility:
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use serde_kdl::{from_str, to_string};
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq)]
//! struct Data {
//!     name: String,
//!     #[serde(with = "serde_kdl::hex_serde")]
//!     bytes: Vec<u8>,
//! }
//!
//! let data = Data {
//!     name: "example".to_string(),
//!     bytes: vec![0xde, 0xad, 0xbe, 0xef],
//! };
//!
//! let kdl_string = to_string(&data).unwrap();
//! let parsed: Data = from_str(&kdl_string).unwrap();
//! assert_eq!(data, parsed);
//! ```

use crate::hex::{decode_hex, encode_hex};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Serialize bytes as a hex string.
pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex_string = encode_hex(bytes);
    hex_string.serialize(serializer)
}

/// Deserialize a hex string as bytes.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let hex_string = String::deserialize(deserializer)?;
    decode_hex(&hex_string).map_err(serde::de::Error::custom)
}

/// Serialize an option of bytes as an optional hex string.
pub mod option {
    use super::*;

    pub fn serialize<S>(bytes: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match bytes {
            Some(bytes) => {
                let hex_string = encode_hex(bytes);
                serializer.serialize_some(&hex_string)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt_hex_string: Option<String> = Option::deserialize(deserializer)?;
        match opt_hex_string {
            Some(hex_string) => {
                let bytes = decode_hex(&hex_string).map_err(serde::de::Error::custom)?;
                Ok(Some(bytes))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use crate::{from_str, to_string};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestStruct {
        name: String,
        #[serde(with = "crate::hex_serde")]
        data: Vec<u8>,
    }

    #[test]
    fn test_hex_serde_module() {
        let test_data = TestStruct {
            name: "test".to_string(),
            data: vec![0xde, 0xad, 0xbe, 0xef],
        };

        let kdl_string = to_string(&test_data).unwrap();
        let parsed: TestStruct = from_str(&kdl_string).unwrap();
        assert_eq!(test_data, parsed);
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct OptionalBytes {
        name: String,
        #[serde(with = "crate::hex_serde::option")]
        data: Option<Vec<u8>>,
    }

    #[test]
    fn test_optional_hex_serde_with_data() {
        let with_data = OptionalBytes {
            name: "with".to_string(),
            data: Some(vec![0x01, 0x02, 0x03]),
        };

        let kdl_string = to_string(&with_data).unwrap();
        let parsed: OptionalBytes = from_str(&kdl_string).unwrap();
        assert_eq!(with_data, parsed);
    }
}