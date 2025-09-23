//! Hex encoding and decoding utilities for the bytes feature.

use crate::error::{Error, Result};

/// Encode bytes as a lowercase hex string.
pub fn encode_hex(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        result.push_str(&format!("{byte:02x}"));
    }
    result
}

/// Decode a hex string into bytes.
/// Returns an error if the string contains invalid hex characters or has odd length.
pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>> {
    if !hex_str.len().is_multiple_of(2) {
        return Err(Error::InvalidHexString(
            "hex string must have even length".to_string(),
        ));
    }

    let mut bytes = Vec::with_capacity(hex_str.len() / 2);
    let chars: Vec<char> = hex_str.chars().collect();

    for chunk in chars.chunks(2) {
        let high = hex_char_to_value(chunk[0])?;
        let low = hex_char_to_value(chunk[1])?;
        bytes.push((high << 4) | low);
    }

    Ok(bytes)
}

/// Convert a hex character to its numeric value.
fn hex_char_to_value(c: char) -> Result<u8> {
    match c {
        '0'..='9' => Ok(c as u8 - b'0'),
        'a'..='f' => Ok(c as u8 - b'a' + 10),
        'A'..='F' => Ok(c as u8 - b'A' + 10),
        _ => Err(Error::InvalidHexString(format!(
            "invalid hex character: '{c}'"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_hex() {
        assert_eq!(encode_hex(&[]), "");
        assert_eq!(encode_hex(&[0x48, 0x65, 0x6c, 0x6c, 0x6f]), "48656c6c6f");
        assert_eq!(encode_hex(&[0x00, 0xff, 0xaa, 0x55]), "00ffaa55");
    }

    #[test]
    fn test_decode_hex() {
        assert_eq!(decode_hex("").unwrap(), vec![]);
        assert_eq!(
            decode_hex("48656c6c6f").unwrap(),
            vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]
        );
        assert_eq!(
            decode_hex("00ffaa55").unwrap(),
            vec![0x00, 0xff, 0xaa, 0x55]
        );
        assert_eq!(
            decode_hex("00FFAA55").unwrap(),
            vec![0x00, 0xff, 0xaa, 0x55]
        ); // uppercase
    }

    #[test]
    fn test_decode_hex_errors() {
        assert!(decode_hex("1").is_err()); // odd length
        assert!(decode_hex("1g").is_err()); // invalid character
        assert!(decode_hex("zz").is_err()); // invalid characters
    }
}
