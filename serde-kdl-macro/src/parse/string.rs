//! String parsing and Unicode escape processing
//!
//! This module handles parsing of KDL strings, including Unicode escape sequences
//! and validation according to Section 3.9 of the KDL specification.

use syn::Result;

/// Processes Unicode escape sequences (\u{...}) in quoted strings
pub(crate) fn process_unicode_escapes(value: &str, span: proc_macro2::Span) -> Result<String> {
    let mut result = String::new();
    let mut chars = value.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next_ch) = chars.peek() {
                if next_ch == 'u' {
                    chars.next(); // consume 'u'

                    // Expect opening brace
                    if chars.next() != Some('{') {
                        return Err(syn::Error::new(
                            span,
                            "Invalid Unicode escape: expected '{' after '\\u'"
                        ));
                    }

                    // Collect hex digits until closing brace
                    let mut hex_digits = String::new();
                    let mut found_closing_brace = false;

                    while let Some(hex_ch) = chars.next() {
                        if hex_ch == '}' {
                            found_closing_brace = true;
                            break;
                        }
                        if hex_ch.is_ascii_hexdigit() {
                            hex_digits.push(hex_ch);
                        } else {
                            return Err(syn::Error::new(
                                span,
                                format!("Invalid character '{}' in Unicode escape sequence", hex_ch)
                            ));
                        }
                    }

                    if !found_closing_brace {
                        return Err(syn::Error::new(
                            span,
                            "Unterminated Unicode escape sequence"
                        ));
                    }

                    if hex_digits.is_empty() {
                        return Err(syn::Error::new(
                            span,
                            "Empty Unicode escape sequence"
                        ));
                    }

                    // Parse hex number
                    match u32::from_str_radix(&hex_digits, 16) {
                        Ok(code_point) => {
                            match char::from_u32(code_point) {
                                Some(unicode_char) => result.push(unicode_char),
                                None => return Err(syn::Error::new(
                                    span,
                                    format!("Invalid Unicode code point: U+{:X}", code_point)
                                )),
                            }
                        }
                        Err(_) => return Err(syn::Error::new(
                            span,
                            format!("Invalid hex number in Unicode escape: {}", hex_digits)
                        )),
                    }
                } else {
                    // Handle other escape sequences (not implemented yet)
                    result.push(ch);
                    result.push(next_ch);
                    chars.next(); // consume the next character
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }

    Ok(result)
}
