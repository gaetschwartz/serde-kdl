//! String parsing and escape sequence processing
//!
//! This module handles parsing of KDL strings, including standard escape sequences
//! and Unicode escapes. Line continuation (escaped whitespace) is not supported.

use syn::Result;

/// Processes escape sequences in quoted strings
/// Supports standard escapes (\n, \r, \t, \\, \", \b, \f, \s) and Unicode escapes (\u{...})
/// Line continuation (backslash followed by whitespace) is not supported
pub(crate) fn process_string_escapes(value: &str, span: proc_macro2::Span) -> Result<String> {
    let mut result = String::new();
    let mut chars = value.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next_ch) = chars.peek() {
                match next_ch {
                    // Standard escape sequences
                    'n' => {
                        chars.next(); // consume 'n'
                        result.push('\u{000A}'); // Line Feed
                    }
                    'r' => {
                        chars.next(); // consume 'r'
                        result.push('\u{000D}'); // Carriage Return
                    }
                    't' => {
                        chars.next(); // consume 't'
                        result.push('\u{0009}'); // Character Tabulation (Tab)
                    }
                    '\\' => {
                        chars.next(); // consume '\'
                        result.push('\u{005C}'); // Reverse Solidus (Backslash)
                    }
                    '"' => {
                        chars.next(); // consume '"'
                        result.push('\u{0022}'); // Quotation Mark (Double Quote)
                    }
                    'b' => {
                        chars.next(); // consume 'b'
                        result.push('\u{0008}'); // Backspace
                    }
                    'f' => {
                        chars.next(); // consume 'f'
                        result.push('\u{000C}'); // Form Feed
                    }
                    's' => {
                        chars.next(); // consume 's'
                        result.push('\u{0020}'); // Space
                    }
                    // Unicode escape sequence
                    'u' => {
                        chars.next(); // consume 'u'

                        // Expect opening brace
                        if chars.next() != Some('{') {
                            return Err(syn::Error::new(
                                span,
                                "Invalid Unicode escape: expected '{' after '\\u'",
                            ));
                        }

                        // Collect hex digits until closing brace
                        let mut hex_digits = String::new();
                        let mut found_closing_brace = false;

                        for hex_ch in chars.by_ref() {
                            if hex_ch == '}' {
                                found_closing_brace = true;
                                break;
                            }
                            if hex_ch.is_ascii_hexdigit() {
                                hex_digits.push(hex_ch);
                            } else {
                                return Err(syn::Error::new(
                                    span,
                                    format!(
                                        "Invalid character '{}' in Unicode escape sequence",
                                        hex_ch
                                    ),
                                ));
                            }
                        }

                        if !found_closing_brace {
                            return Err(syn::Error::new(
                                span,
                                "Unterminated Unicode escape sequence",
                            ));
                        }

                        if hex_digits.is_empty() {
                            return Err(syn::Error::new(span, "Empty Unicode escape sequence"));
                        }

                        if hex_digits.len() > 6 {
                            return Err(syn::Error::new(
                                span,
                                format!(
                                    "Unicode escape sequence too long: {} hex digits (max 6)",
                                    hex_digits.len()
                                ),
                            ));
                        }

                        // Parse hex number
                        match u32::from_str_radix(&hex_digits, 16) {
                            Ok(code_point) => match char::from_u32(code_point) {
                                Some(unicode_char) => result.push(unicode_char),
                                None => {
                                    return Err(syn::Error::new(
                                        span,
                                        format!("Invalid Unicode code point: U+{:X}", code_point),
                                    ))
                                }
                            },
                            Err(_) => {
                                return Err(syn::Error::new(
                                    span,
                                    format!("Invalid hex number in Unicode escape: {}", hex_digits),
                                ))
                            }
                        }
                    }
                    // Invalid escape sequences
                    _ => {
                        return Err(syn::Error::new(
                            span,
                            format!("Invalid escape sequence: \\{}", next_ch),
                        ));
                    }
                }
            } else {
                // Backslash at end of string - invalid
                return Err(syn::Error::new(
                    span,
                    "Invalid escape sequence: backslash at end of string",
                ));
            }
        } else {
            result.push(ch);
        }
    }

    Ok(result)
}
