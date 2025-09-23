//! Validation functions for KDL strings and values
//!
//! This module contains validation logic for UTF-8 strings, disallowed code points,
//! identifier string validation, and type annotation validation according to the KDL specification.

use crate::ast::{
    is_reserved_type, KdlValue, RESERVED_FLOAT_TYPES, RESERVED_INTEGER_TYPES, RESERVED_STRING_TYPES,
};
use syn::Result;

/// Validates type annotation against value according to Section 3.8
#[allow(dead_code)]
pub(crate) fn validate_type_annotation(type_annotation: &str, value: &KdlValue) -> Result<()> {
    match value {
        KdlValue::Integer(_) => {
            // For integers, only integer type annotations or custom (non-reserved) annotations are allowed
            if RESERVED_INTEGER_TYPES.contains(&type_annotation) {
                Ok(())
            } else if !is_reserved_type(type_annotation) {
                // Custom (non-reserved) type annotations are allowed
                Ok(())
            } else {
                // This is a reserved type but not for integers
                Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "Type annotation '{}' is not valid for integer values",
                        type_annotation
                    ),
                ))
            }
        }
        KdlValue::Float(_) => {
            // For floats, only float type annotations or custom (non-reserved) annotations are allowed
            if RESERVED_FLOAT_TYPES.contains(&type_annotation) {
                Ok(())
            } else if !is_reserved_type(type_annotation) {
                // Custom (non-reserved) type annotations are allowed
                Ok(())
            } else {
                // This is a reserved type but not for floats
                Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "Type annotation '{}' is not valid for float values",
                        type_annotation
                    ),
                ))
            }
        }
        KdlValue::String(_) => {
            // For strings, only string type annotations or custom (non-reserved) annotations are allowed
            if RESERVED_STRING_TYPES.contains(&type_annotation) {
                Ok(())
            } else if !is_reserved_type(type_annotation) {
                // Custom (non-reserved) type annotations are allowed
                Ok(())
            } else {
                // This is a reserved type but not for strings
                Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "Type annotation '{}' is not valid for string values",
                        type_annotation
                    ),
                ))
            }
        }
        KdlValue::Boolean(_) | KdlValue::Null => {
            // Boolean and null values don't have reserved type annotations,
            // but custom annotations are allowed
            if !is_reserved_type(type_annotation) {
                Ok(())
            } else {
                Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "Type annotation '{}' is reserved and not valid for this value type",
                        type_annotation
                    ),
                ))
            }
        }
        KdlValue::TypeAnnotated { value, .. } => {
            // For nested type annotations, validate the inner value
            validate_type_annotation(type_annotation, value)
        }
    }
}

// =============================================================================
// Identifier String Validation (Section 3.10)
// =============================================================================

/// Validates an identifier string with context about whether keywords should be rejected
pub(crate) fn validate_identifier_string_with_context(
    identifier: &str,
    span: proc_macro2::Span,
    reject_keywords: bool,
) -> Result<()> {
    if identifier.is_empty() {
        return Err(syn::Error::new(span, "Identifier cannot be empty"));
    }

    let mut chars = identifier.chars();
    let first_char = chars.next().unwrap(); // Safe because we checked for empty above

    // Check initial character restrictions (Section 3.10.1)
    if !is_valid_initial_character(first_char, &mut chars.clone()) {
        return Err(syn::Error::new(
            span,
            format!("Invalid initial character '{}' in identifier", first_char),
        ));
    }

    // Check remaining characters (Section 3.10.2)
    for ch in chars {
        if !is_valid_identifier_character(ch) {
            return Err(syn::Error::new(
                span,
                format!("Invalid character '{}' in identifier", ch),
            ));
        }
    }

    // Check for disallowed patterns
    if looks_like_number(identifier) {
        return Err(syn::Error::new(
            span,
            format!(
                "Identifier '{}' looks like a number and is not allowed",
                identifier
            ),
        ));
    }

    if reject_keywords && is_keyword(identifier) {
        return Err(syn::Error::new(
            span,
            format!(
                "Identifier '{}' is a reserved keyword without '#' prefix",
                identifier
            ),
        ));
    }

    Ok(())
}

/// Checks if a character is valid as the initial character of an identifier (Section 3.10.1)
pub(crate) fn is_valid_initial_character(ch: char, remaining_chars: &mut std::str::Chars) -> bool {
    // Cannot start with decimal digit
    if ch.is_ascii_digit() {
        return false;
    }

    // Cannot start with non-identifier characters
    if !is_valid_identifier_character(ch) {
        return false;
    }

    // Special rules for +, -, and .
    match ch {
        '+' | '-' => {
            // Can only be initial if second character is not a digit
            if let Some(second_char) = remaining_chars.clone().next() {
                if second_char.is_ascii_digit() {
                    return false;
                }
                // If second character is '.', third character must not be a digit
                if second_char == '.' {
                    if let Some(third_char) = remaining_chars.clone().nth(1) {
                        if third_char.is_ascii_digit() {
                            return false;
                        }
                    }
                }
            }
            true
        }
        '.' => {
            // Can only be initial if second character is not a digit
            if let Some(second_char) = remaining_chars.clone().next() {
                if second_char.is_ascii_digit() {
                    return false;
                }
            }
            true
        }
        _ => true,
    }
}

/// Checks if a character is valid anywhere in an identifier (Section 3.10.2)
pub(crate) fn is_valid_identifier_character(ch: char) -> bool {
    // Cannot use specific punctuation characters
    if matches!(
        ch,
        '(' | ')' | '{' | '}' | '[' | ']' | '/' | '\\' | '"' | '#' | ';' | '='
    ) {
        return false;
    }

    // Cannot use whitespace (Section 3.17)
    if is_whitespace(ch) {
        return false;
    }

    // Cannot use newlines (Section 3.18)
    if is_newline(ch) {
        return false;
    }

    // Cannot use disallowed literal code points (Section 3.19)
    if is_disallowed_code_point(ch) {
        return false;
    }

    true
}

/// Checks if an identifier looks like a number and should be rejected
pub(crate) fn looks_like_number(identifier: &str) -> bool {
    // Check for "almost a number" pattern: decimal point without leading digit (like ".1")
    if identifier.starts_with('.') && identifier.len() > 1 {
        if let Some(second_char) = identifier.chars().nth(1) {
            if second_char.is_ascii_digit() {
                return true;
            }
        }
    }

    // Check for identifiers that appear to start with a number
    // Examples: "1.0v2", "-1em"
    let mut chars = identifier.chars();
    let first_char = match chars.next() {
        Some(ch) => ch,
        None => return false,
    };

    // Handle signs
    let effective_first_char = if matches!(first_char, '+' | '-') {
        match chars.next() {
            Some(ch) => ch,
            None => return false, // Just a sign, not a number
        }
    } else {
        first_char
    };

    // If it starts with a digit, it looks like a number
    if effective_first_char.is_ascii_digit() {
        return true;
    }

    // If it starts with a dot after a sign, it might look like a number
    if effective_first_char == '.' && first_char != effective_first_char {
        // This is "+." or "-." - check if followed by digit
        if let Some(third_char) = chars.next() {
            if third_char.is_ascii_digit() {
                return true;
            }
        }
    }

    false
}

/// Checks if an identifier is a reserved keyword without the '#' prefix
pub(crate) fn is_keyword(identifier: &str) -> bool {
    matches!(
        identifier,
        "inf" | "-inf" | "nan" | "true" | "false" | "null"
    )
}

// =============================================================================
// Helper functions for whitespace, newlines, and disallowed code points
// =============================================================================

/// Checks if a character is whitespace according to Section 3.17
pub(crate) fn is_whitespace(ch: char) -> bool {
    match ch as u32 {
        0x0009 |  // Character Tabulation
        0x0020 |  // Space
        0x00A0 |  // No-Break Space
        0x1680 |  // Ogham Space Mark
        0x2000 |  // En Quad
        0x2001 |  // Em Quad
        0x2002 |  // En Space
        0x2003 |  // Em Space
        0x2004 |  // Three-Per-Em Space
        0x2005 |  // Four-Per-Em Space
        0x2006 |  // Six-Per-Em Space
        0x2007 |  // Figure Space
        0x2008 |  // Punctuation Space
        0x2009 |  // Thin Space
        0x200A |  // Hair Space
        0x202F |  // Narrow No-Break Space
        0x205F |  // Medium Mathematical Space
        0x3000    // Ideographic Space
        => true,
        _ => false,
    }
}

/// Checks if a character is a newline according to Section 3.18
pub(crate) fn is_newline(ch: char) -> bool {
    match ch as u32 {
        0x000A |  // LF - Line Feed
        0x000B |  // VT - Vertical Tab
        0x000C |  // FF - Form Feed
        0x000D |  // CR - Carriage Return
        0x0085 |  // NEL - Next Line
        0x2028 |  // LS - Line Separator
        0x2029    // PS - Paragraph Separator
        => true,
        _ => false,
    }
}

/// Checks if a character is a disallowed literal code point according to Section 3.19
pub(crate) fn is_disallowed_code_point(ch: char) -> bool {
    let code_point = ch as u32;

    // U+0000-0008 control characters
    (code_point <= 0x0008) ||
    // U+000E-001F control characters
    (0x000E..=0x001F).contains(&code_point) ||
    // U+007F Delete control character
    (code_point == 0x007F) ||
    // U+D800-DFFF Non-Unicode Scalar Values
    (0xD800..=0xDFFF).contains(&code_point) ||
    // U+200E-200F direction control
    (0x200E..=0x200F).contains(&code_point) ||
    // U+202A-202E direction control
    (0x202A..=0x202E).contains(&code_point) ||
    // U+2066-2069 direction control
    (0x2066..=0x2069).contains(&code_point) ||
    // U+FEFF BOM (except at document start, but we don't handle that exception here)
    (code_point == 0xFEFF)
}
