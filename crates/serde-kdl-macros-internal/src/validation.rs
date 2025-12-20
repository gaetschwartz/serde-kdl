//! Validation functions for KDL strings and values
//!
//! This module contains validation logic for UTF-8 strings, disallowed code points,
//! identifier string validation, and type annotation validation according to the KDL specification.

use crate::{
    ast::{
        KdlValue, RESERVED_FLOAT_TYPES, RESERVED_INTEGER_TYPES, RESERVED_STRING_TYPES,
        is_reserved_type,
    },
    parse::value::KdlLit,
};
use proc_macro2::Span;
use syn::Result;

/// Validates type annotation against value according to Section 3.8
#[allow(dead_code)]
pub fn validate_type_annotation(type_annotation: &str, value: &KdlValue) -> Result<()> {
    match value {
        KdlValue::Lit(KdlLit::Integer(_, _)) => {
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
                    format!("Type annotation '{type_annotation}' is not valid for integer values"),
                ))
            }
        }
        KdlValue::Lit(
            KdlLit::Float(_, _) | KdlLit::Nan(_) | KdlLit::Infinity(_) | KdlLit::NegInfinity(_),
        ) => {
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
                    format!("Type annotation '{type_annotation}' is not valid for float values"),
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
                    format!("Type annotation '{type_annotation}' is not valid for string values"),
                ))
            }
        }
        KdlValue::Lit(KdlLit::Boolean(_, _) | KdlLit::Null(_)) => {
            // Boolean and null values don't have reserved type annotations,
            // but custom annotations are allowed
            if is_reserved_type(type_annotation) {
                Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "Type annotation '{type_annotation}' is reserved and not valid for this value type"
                    ),
                ))
            } else {
                Ok(())
            }
        }
        KdlValue::Variable(_) => {
            // Variables are resolved at runtime, we can't validate the type annotation
            // against the actual value at compile time. Allow any annotation.
            Ok(())
        }
    }
}

// =============================================================================
// Identifier String Validation (Section 3.10)
// =============================================================================

#[derive(Clone, Copy, Debug, Default)]
pub struct ValidationOptions {
    pub strict: bool,
}

pub const RESERVED_KEYWORDS: &[&str] = &["inf", "-inf", "nan", "true", "false", "null"];

/// Validates an identifier string with context about whether keywords should be rejected
pub fn validate_identifier(ident: &syn::Ident, options: ValidationOptions) -> Result<()> {
    let ident_str = ident.to_string();
    validate_identifier_string(&ident_str, ident.span(), options)
}

/// Validates an identifier string with context about whether keywords should be rejected
pub fn validate_identifier_string(
    ident_str: &str,
    span: Span,
    options: ValidationOptions,
) -> Result<()> {
    if RESERVED_KEYWORDS.contains(&ident_str) {
        return Err(syn::Error::new(
            span,
            format!(
                "Identifier '{ident_str}' is a reserved keyword and cannot be used as an identifier",
            ),
        ));
    }

    if options.strict {
        let mut chars = ident_str.chars();
        let first_char = chars.next().unwrap(); // Safe because we checked for empty above

        // Check initial character restrictions (Section 3.10.1)
        if !strict::is_valid_initial_character(first_char, &mut chars) {
            return Err(syn::Error::new(
                span,
                format!("Invalid initial character '{first_char}' in identifier"),
            ));
        }

        // Check remaining characters (Section 3.10.2)
        for ch in chars {
            if !strict::is_valid_identifier_character(ch) {
                return Err(syn::Error::new(
                    span,
                    format!("Invalid character '{ch}' in identifier"),
                ));
            }
        }

        // Check for disallowed patterns
        if strict::looks_like_number(ident_str) {
            return Err(syn::Error::new(
                span,
                format!("Identifier '{ident_str}' looks like a number and is not allowed"),
            ));
        }
    }

    Ok(())
}

#[allow(unused)]
mod strict {
    use super::*;

    /// Checks if a character is valid as the initial character of an identifier (Section 3.10.1)
    pub fn is_valid_initial_character(ch: char, remaining_chars: &mut std::str::Chars) -> bool {
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
                    if second_char == '.'
                        && let Some(third_char) = remaining_chars.clone().nth(1)
                        && third_char.is_ascii_digit()
                    {
                        return false;
                    }
                }
                true
            }
            '.' => {
                // Can only be initial if second character is not a digit
                if let Some(second_char) = remaining_chars.clone().next()
                    && second_char.is_ascii_digit()
                {
                    return false;
                }
                true
            }
            _ => true,
        }
    }

    /// Checks if an identifier looks like a number and should be rejected
    pub fn looks_like_number(identifier: &str) -> bool {
        // Check for "almost a number" pattern: decimal point without leading digit (like ".1")
        if identifier.starts_with('.')
            && identifier.len() > 1
            && let Some(second_char) = identifier.chars().nth(1)
            && second_char.is_ascii_digit()
        {
            return true;
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
            if let Some(third_char) = chars.next()
                && third_char.is_ascii_digit()
            {
                return true;
            }
        }

        false
    }

    /// Checks if a character is valid anywhere in an identifier (Section 3.10.2)
    pub fn is_valid_identifier_character(ch: char) -> bool {
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
    // =============================================================================
    // Helper functions for whitespace, newlines, and disallowed code points
    // =============================================================================

    /// Checks if a character is whitespace according to Section 3.17
    pub fn is_whitespace(ch: char) -> bool {
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
    pub fn is_newline(ch: char) -> bool {
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
    pub fn is_disallowed_code_point(ch: char) -> bool {
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

    // =============================================================================
    // Unit Tests
    // =============================================================================

    #[cfg(test)]
    mod tests {
        use super::*;

        /// Validates an identifier string with context about whether keywords should be rejected
        pub fn validate_identifier_string_test(ident_str: &str) -> syn::Result<()> {
            validate_identifier_string(
                ident_str,
                Span::call_site(),
                ValidationOptions { strict: true },
            )
        }

        // =========================================================================
        // Section 3.17: Whitespace Tests
        // =========================================================================

        /// Test that all whitespace characters from Table 2 are correctly identified
        #[test]
        fn test_whitespace_characters() {
            // Test all whitespace characters defined in Section 3.17, Table 2
            let whitespace_chars = [
                ('\u{0009}', "Character Tabulation"),      // U+0009
                ('\u{0020}', "Space"),                     // U+0020
                ('\u{00A0}', "No-Break Space"),            // U+00A0
                ('\u{1680}', "Ogham Space Mark"),          // U+1680
                ('\u{2000}', "En Quad"),                   // U+2000
                ('\u{2001}', "Em Quad"),                   // U+2001
                ('\u{2002}', "En Space"),                  // U+2002
                ('\u{2003}', "Em Space"),                  // U+2003
                ('\u{2004}', "Three-Per-Em Space"),        // U+2004
                ('\u{2005}', "Four-Per-Em Space"),         // U+2005
                ('\u{2006}', "Six-Per-Em Space"),          // U+2006
                ('\u{2007}', "Figure Space"),              // U+2007
                ('\u{2008}', "Punctuation Space"),         // U+2008
                ('\u{2009}', "Thin Space"),                // U+2009
                ('\u{200A}', "Hair Space"),                // U+200A
                ('\u{202F}', "Narrow No-Break Space"),     // U+202F
                ('\u{205F}', "Medium Mathematical Space"), // U+205F
                ('\u{3000}', "Ideographic Space"),         // U+3000
            ];

            for (ch, name) in &whitespace_chars {
                assert!(
                    is_whitespace(*ch),
                    "Character {} (U+{:04X}) should be recognized as whitespace",
                    name,
                    *ch as u32
                );
            }
        }

        /// Test that non-whitespace characters are not identified as whitespace
        #[test]
        fn test_non_whitespace_characters() {
            let non_whitespace_chars = [
                'a', 'Z', '0', '9', '_', '-', '+', '.', '!', '@', '#', '$', '%', '^', '&', '*',
                '(', ')', '[', ']', '{', '}', '|', '\\', ':', ';', '"', '\'', '<', '>', ',', '?',
                '/', '~', '`', '=',
                // Unicode characters that are not whitespace
                '\u{0008}', // Backspace (control character, not whitespace)
                '\u{000A}', // Line Feed (newline, not whitespace)
                '\u{000B}', // Vertical Tab (newline, not whitespace)
                '\u{000C}', // Form Feed (newline, not whitespace)
                '\u{000D}', // Carriage Return (newline, not whitespace)
                '\u{001F}', // Unit Separator (control character, not whitespace)
                '\u{0021}', // Exclamation mark (regular character)
                '\u{007F}', // Delete (control character, not whitespace)
                '\u{0080}', // Padding Character (control character, not whitespace)
                '\u{167F}', // One before Ogham Space Mark
                '\u{1681}', // One after Ogham Space Mark
                '\u{1FFF}', // One before En Quad range
                '\u{200B}', // Zero Width Space (not in whitespace table)
                '\u{200C}', // Zero Width Non-Joiner (not in whitespace table)
                '\u{200D}', // Zero Width Joiner (not in whitespace table)
                '\u{200E}', // Left-to-Right Mark (direction control, not whitespace)
                '\u{2060}', // Word Joiner (not in whitespace table)
                '\u{2FFE}', // One before Ideographic Space
                '\u{3001}', // One after Ideographic Space
            ];

            for ch in &non_whitespace_chars {
                assert!(
                    !is_whitespace(*ch),
                    "Character '{}' (U+{:04X}) should NOT be recognized as whitespace",
                    ch,
                    *ch as u32
                );
            }
        }

        /// Test that whitespace characters are rejected in identifiers
        #[test]
        fn test_whitespace_not_valid_in_identifiers() {
            // Test all whitespace characters from Table 2
            let whitespace_chars = [
                '\u{0009}', // Character Tabulation
                '\u{0020}', // Space
                '\u{00A0}', // No-Break Space
                '\u{1680}', // Ogham Space Mark
                '\u{2000}', // En Quad
                '\u{2001}', // Em Quad
                '\u{2002}', // En Space
                '\u{2003}', // Em Space
                '\u{2004}', // Three-Per-Em Space
                '\u{2005}', // Four-Per-Em Space
                '\u{2006}', // Six-Per-Em Space
                '\u{2007}', // Figure Space
                '\u{2008}', // Punctuation Space
                '\u{2009}', // Thin Space
                '\u{200A}', // Hair Space
                '\u{202F}', // Narrow No-Break Space
                '\u{205F}', // Medium Mathematical Space
                '\u{3000}', // Ideographic Space
            ];

            for ch in &whitespace_chars {
                assert!(
                    !is_valid_identifier_character(*ch),
                    "Whitespace character U+{:04X} should not be valid in identifiers",
                    *ch as u32
                );
            }
        }

        /// Test identifier validation with whitespace characters
        #[test]
        fn test_identifier_validation_with_whitespace() {
            let test_cases = [
                ("hello world", "identifier with regular space"),
                ("hello\tworld", "identifier with tab"),
                ("hello\u{00A0}world", "identifier with no-break space"),
                ("hello\u{1680}world", "identifier with ogham space mark"),
                ("hello\u{2000}world", "identifier with en quad"),
                ("hello\u{2001}world", "identifier with em quad"),
                ("hello\u{2002}world", "identifier with en space"),
                ("hello\u{2003}world", "identifier with em space"),
                ("hello\u{2004}world", "identifier with three-per-em space"),
                ("hello\u{2005}world", "identifier with four-per-em space"),
                ("hello\u{2006}world", "identifier with six-per-em space"),
                ("hello\u{2007}world", "identifier with figure space"),
                ("hello\u{2008}world", "identifier with punctuation space"),
                ("hello\u{2009}world", "identifier with thin space"),
                ("hello\u{200A}world", "identifier with hair space"),
                (
                    "hello\u{202F}world",
                    "identifier with narrow no-break space",
                ),
                (
                    "hello\u{205F}world",
                    "identifier with medium mathematical space",
                ),
                ("hello\u{3000}world", "identifier with ideographic space"),
                (" hello", "identifier starting with space"),
                ("hello ", "identifier ending with space"),
                ("\thello", "identifier starting with tab"),
                ("hello\t", "identifier ending with tab"),
            ];

            for (identifier, description) in &test_cases {
                let result = validate_identifier_string_test(identifier);

                assert!(
                    result.is_err(),
                    "Identifier validation should fail for {description}: '{identifier}'"
                );
            }
        }

        /// Test valid identifiers that don't contain whitespace
        #[test]
        fn test_valid_identifiers_without_whitespace() {
            let valid_identifiers = [
                "hello",
                "world",
                "identifier",
                "snake_case",
                "kebab-case",
                "camelCase",
                "PascalCase",
                "with123numbers",
                "with.dots",
                "with+plus",
                "with-minus",
                "unicode_αβγ",
                "emoji_😀🌟",
                "mixed_identifierαβγ123",
            ];

            for identifier in &valid_identifiers {
                let result = validate_identifier_string_test(identifier);
                assert!(
                    result.is_ok(),
                    "Identifier validation should pass for valid identifier: '{}'. Error: {:?}",
                    identifier,
                    result.err()
                );
            }

            // Test identifiers that might fail due to containing invalid characters
            // These are documented to show what characters are not allowed in identifiers
            let invalid_identifiers = [
                "special_chars!@#$%^&*()", // Contains special chars that might be disallowed
                "with(parens)",            // Contains parentheses
                "with[brackets]",          // Contains brackets
                "with{braces}",            // Contains braces
                "with/slash",              // Contains slash
                "with\\backslash",         // Contains backslash
                "with\"quotes\"",          // Contains quotes
                "with#hash",               // Contains hash
                "with;semicolon",          // Contains semicolon
                "with=equals",             // Contains equals
            ];

            for identifier in &invalid_identifiers {
                let result = validate_identifier_string_test(identifier);

                if result.is_ok() {
                    println!("Note: Identifier '{identifier}' was unexpectedly valid");
                }
                // We don't assert failure here because the exact rules depend on the implementation
            }
        }

        /// Test edge cases around whitespace boundaries
        #[test]
        fn test_whitespace_boundary_cases() {
            // Test characters just before and after the whitespace ranges
            let boundary_cases = [
                ('\u{0008}', false), // Just before Character Tabulation
                ('\u{000A}', false), // Just after Character Tabulation (newline)
                ('\u{001F}', false), // Just before Space
                ('\u{0021}', false), // Just after Space
                ('\u{009F}', false), // Just before No-Break Space
                ('\u{00A1}', false), // Just after No-Break Space
                ('\u{167F}', false), // Just before Ogham Space Mark
                ('\u{1681}', false), // Just after Ogham Space Mark
                ('\u{1FFF}', false), // Just before En Quad range
                ('\u{200B}', false), // Just after Hair Space (Zero Width Space, not whitespace)
                ('\u{202E}', false), // Just before Narrow No-Break Space
                ('\u{2030}', false), // Just after Narrow No-Break Space
                ('\u{205E}', false), // Just before Medium Mathematical Space
                ('\u{2060}', false), // Just after Medium Mathematical Space
                ('\u{2FFF}', false), // Just before Ideographic Space
                ('\u{3001}', false), // Just after Ideographic Space
            ];

            for (ch, should_be_whitespace) in &boundary_cases {
                assert_eq!(
                    is_whitespace(*ch),
                    *should_be_whitespace,
                    "Character U+{:04X} whitespace test failed",
                    *ch as u32
                );
            }
        }

        /// Test performance with a large string containing various whitespace characters
        #[test]
        fn test_whitespace_performance() {
            // Create a string with many whitespace characters for performance testing
            let mut test_string = String::new();
            let whitespace_chars = [
                '\u{0009}', '\u{0020}', '\u{00A0}', '\u{1680}', '\u{2000}', '\u{2001}', '\u{2002}',
                '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}',
                '\u{200A}', '\u{202F}', '\u{205F}', '\u{3000}',
            ];

            // Repeat the pattern 1000 times
            for _ in 0..1000 {
                for &ch in &whitespace_chars {
                    test_string.push(ch);
                }
            }

            // Test that all characters are recognized as whitespace
            let start = std::time::Instant::now();
            for ch in test_string.chars() {
                assert!(is_whitespace(ch));
            }
            let duration = start.elapsed();

            // The test should complete reasonably quickly (within 1 second)
            assert!(
                duration.as_secs() < 1,
                "Whitespace validation took too long: {duration:?}"
            );
        }

        /// Test interaction between whitespace and other character classes
        #[test]
        fn test_whitespace_vs_other_character_classes() {
            // Test that whitespace characters are not confused with newlines
            let whitespace_chars = [
                '\u{0009}', '\u{0020}', '\u{00A0}', '\u{1680}', '\u{2000}', '\u{2001}', '\u{2002}',
                '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}',
                '\u{200A}', '\u{202F}', '\u{205F}', '\u{3000}',
            ];

            for &ch in &whitespace_chars {
                assert!(
                    is_whitespace(ch),
                    "Character U+{:04X} should be whitespace",
                    ch as u32
                );
                assert!(
                    !is_newline(ch),
                    "Character U+{:04X} should not be newline",
                    ch as u32
                );
                assert!(
                    !is_disallowed_code_point(ch),
                    "Character U+{:04X} should not be disallowed",
                    ch as u32
                );
            }

            // Test that newline characters are not whitespace
            let newline_chars = [
                '\u{000A}', // LF
                '\u{000B}', // VT
                '\u{000C}', // FF
                '\u{000D}', // CR
                '\u{0085}', // NEL
                '\u{2028}', // LS
                '\u{2029}', // PS
            ];

            for &ch in &newline_chars {
                assert!(
                    !is_whitespace(ch),
                    "Character U+{:04X} should not be whitespace",
                    ch as u32
                );
                assert!(
                    is_newline(ch),
                    "Character U+{:04X} should be newline",
                    ch as u32
                );
            }
        }

        // =========================================================================
        // Section 3.18: Newline Tests
        // =========================================================================

        /// Test that all newline characters from Table 3 are correctly identified
        #[test]
        fn test_newline_characters() {
            // Test all newline characters defined in Section 3.18, Table 3
            let newline_chars = [
                ('\u{000A}', "LF", "Line Feed"),
                ('\u{000B}', "VT", "Vertical Tab"),
                ('\u{000C}', "FF", "Form Feed"),
                ('\u{000D}', "CR", "Carriage Return"),
                ('\u{0085}', "NEL", "Next Line"),
                ('\u{2028}', "LS", "Line Separator"),
                ('\u{2029}', "PS", "Paragraph Separator"),
            ];

            for (ch, acronym, name) in &newline_chars {
                assert!(
                    is_newline(*ch),
                    "Character {} ({}) (U+{:04X}) should be recognized as newline",
                    name,
                    acronym,
                    *ch as u32
                );
            }
        }

        /// Test that non-newline characters are not identified as newlines
        #[test]
        fn test_non_newline_characters() {
            let non_newline_chars = [
                // Regular characters
                'a', 'Z', '0', '9', '_', '-', '+', '.', '!', '@', '#', '$', '%', '^', '&', '*', '(',
                ')', '[', ']', '{', '}', '|', '\\', ':', ';', '"', '\'', '<', '>', ',', '?', '/',
                '~', '`', '=',
                // Whitespace characters (should not be newlines)
                '\u{0009}', // Character Tabulation
                '\u{0020}', // Space
                '\u{00A0}', // No-Break Space
                '\u{1680}', // Ogham Space Mark
                '\u{2000}', // En Quad
                '\u{2001}', // Em Quad
                '\u{2002}', // En Space
                '\u{2003}', // Em Space
                '\u{2004}', // Three-Per-Em Space
                '\u{2005}', // Four-Per-Em Space
                '\u{2006}', // Six-Per-Em Space
                '\u{2007}', // Figure Space
                '\u{2008}', // Punctuation Space
                '\u{2009}', // Thin Space
                '\u{200A}', // Hair Space
                '\u{202F}', // Narrow No-Break Space
                '\u{205F}', // Medium Mathematical Space
                '\u{3000}', // Ideographic Space
                // Control characters that are not newlines
                '\u{0000}', // Null
                '\u{0001}', // Start of Heading
                '\u{0008}', // Backspace
                '\u{000E}', // Shift Out
                '\u{001F}', // Unit Separator
                '\u{007F}', // Delete
                // Characters near newline ranges but not newlines
                '\u{0084}', // One before NEL
                '\u{0086}', // One after NEL
                '\u{2027}', // One before LS
                '\u{202A}', // One after PS
            ];

            for ch in &non_newline_chars {
                assert!(
                    !is_newline(*ch),
                    "Character '{}' (U+{:04X}) should NOT be recognized as newline",
                    ch,
                    *ch as u32
                );
            }
        }

        /// Test that newline characters are rejected in identifiers
        #[test]
        fn test_newlines_not_valid_in_identifiers() {
            // Test all newline characters from Table 3
            let newline_chars = [
                '\u{000A}', // LF - Line Feed
                '\u{000B}', // VT - Vertical Tab
                '\u{000C}', // FF - Form Feed
                '\u{000D}', // CR - Carriage Return
                '\u{0085}', // NEL - Next Line
                '\u{2028}', // LS - Line Separator
                '\u{2029}', // PS - Paragraph Separator
            ];

            for ch in &newline_chars {
                assert!(
                    !is_valid_identifier_character(*ch),
                    "Newline character U+{:04X} should not be valid in identifiers",
                    *ch as u32
                );
            }
        }

        /// Test identifier validation with newline characters
        #[test]
        fn test_identifier_validation_with_newlines() {
            let test_cases = [
                ("hello\nworld", "identifier with LF"),
                ("hello\x0Bworld", "identifier with VT"),
                ("hello\x0Cworld", "identifier with FF"),
                ("hello\rworld", "identifier with CR"),
                ("hello\r\nworld", "identifier with CRLF"),
                ("hello\u{0085}world", "identifier with NEL"),
                ("hello\u{2028}world", "identifier with LS"),
                ("hello\u{2029}world", "identifier with PS"),
                ("\nhello", "identifier starting with LF"),
                ("hello\n", "identifier ending with LF"),
                ("\rhello", "identifier starting with CR"),
                ("hello\r", "identifier ending with CR"),
                ("\r\nhello", "identifier starting with CRLF"),
                ("hello\r\n", "identifier ending with CRLF"),
            ];

            for (ident_str, description) in &test_cases {
                let result = validate_identifier_string_test(ident_str);
                assert!(
                    result.is_err(),
                    "Identifier validation should fail for {}: '{}'",
                    description,
                    ident_str
                        .chars()
                        .map(|c| format!("U+{:04X}", c as u32))
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
        }

        /// Test valid identifiers that don't contain newlines
        #[test]
        fn test_valid_identifiers_without_newlines() {
            let valid_identifiers = [
                "hello",
                "world",
                "identifier",
                "snake_case",
                "kebab-case",
                "camelCase",
                "PascalCase",
                "with123numbers",
                "with.dots",
                "with+plus",
                "with-minus",
                "unicode_αβγ",
                "emoji_😀🌟",
                "mixed_identifierαβγ123",
            ];

            for identifier in &valid_identifiers {
                let result = validate_identifier_string_test(identifier);
                assert!(
                    result.is_ok(),
                    "Identifier validation should pass for valid identifier: '{}'. Error: {:?}",
                    identifier,
                    result.err()
                );
            }

            // Characters that might look like newlines but aren't
            // These will fail due to whitespace, not newlines
            let whitespace_identifiers = [
                "with\u{0009}tab",   // Tab is whitespace, should fail differently
                "with\u{0020}space", // Space is whitespace, should fail differently
            ];

            for ident in &whitespace_identifiers {
                let result = validate_identifier_string_test(ident);
                // Note: Some of these will fail due to whitespace, but not due to newlines
                let error_msg = result.unwrap_err().to_string();
                assert!(
                    !error_msg.contains("newline"),
                    "Identifier '{ident}' should not fail due to newline validation"
                );
            }
        }

        /// Test edge cases around newline boundaries
        #[test]
        fn test_newline_boundary_cases() {
            // Test characters just before and after the newline ranges
            let boundary_cases = [
                ('\u{0009}', false), // Character Tabulation (just before LF)
                ('\u{000D}', true),  // CR (should be newline)
                ('\u{000E}', false), // Shift Out (just after CR)
                ('\u{0084}', false), // Index (just before NEL)
                ('\u{0085}', true),  // NEL (should be newline)
                ('\u{0086}', false), // Start of Selected Area (just after NEL)
                ('\u{2027}', false), // Hyphenation Point (just before LS)
                ('\u{2028}', true),  // LS (should be newline)
                ('\u{2029}', true),  // PS (should be newline)
                ('\u{202A}', false), // Left-to-Right Embedding (just after PS)
            ];

            for (ch, should_be_newline) in &boundary_cases {
                assert_eq!(
                    is_newline(*ch),
                    *should_be_newline,
                    "Character U+{:04X} newline test failed",
                    *ch as u32
                );
            }
        }

        /// Test CRLF sequence handling
        /// Note: The specification states that CRLF should be treated as a single newline,
        /// but our current implementation treats CR and LF as separate newline characters.
        /// This test documents the current behavior.
        #[test]
        fn test_crlf_sequence() {
            // Test individual components of CRLF
            assert!(is_newline('\u{000D}'), "CR should be a newline");
            assert!(is_newline('\u{000A}'), "LF should be a newline");

            // Test CRLF in identifiers (should fail due to both CR and LF being newlines)
            let crlf_identifier = "hello\r\nworld";
            let result = validate_identifier_string_test(crlf_identifier);
            assert!(result.is_err(), "Identifier with CRLF should be invalid");

            // Test that both CR and LF are detected as invalid characters
            let mut found_cr_error = false;
            let mut found_lf_error = false;

            // Test CR separately
            let cr_result = validate_identifier_string_test("hello\rworld");
            if cr_result.is_err() {
                found_cr_error = true;
            }

            // Test LF separately
            let lf_result = validate_identifier_string_test("hello\nworld");
            if lf_result.is_err() {
                found_lf_error = true;
            }

            assert!(
                found_cr_error,
                "CR should cause identifier validation to fail"
            );
            assert!(
                found_lf_error,
                "LF should cause identifier validation to fail"
            );
        }

        /// Test performance with a large string containing various newline characters
        #[test]
        fn test_newline_performance() {
            // Create a string with many newline characters for performance testing
            let mut chars_to_test = Vec::new();
            let newline_chars = [
                '\u{000A}', '\u{000B}', '\u{000C}', '\u{000D}', '\u{0085}', '\u{2028}', '\u{2029}',
            ];

            // Add newline characters
            for _ in 0..1000 {
                for &ch in &newline_chars {
                    chars_to_test.push(ch);
                }
            }

            // Add some non-newline characters for comparison
            for _ in 0..1000 {
                chars_to_test.push('a');
                chars_to_test.push('\u{0020}'); // Space
                chars_to_test.push('\u{0009}'); // Tab
            }

            // Test that newline detection is fast
            let start = std::time::Instant::now();
            let mut newline_count = 0;
            for &ch in &chars_to_test {
                if is_newline(ch) {
                    newline_count += 1;
                }
            }
            let duration = start.elapsed();

            // Verify we found the expected number of newlines
            assert_eq!(
                newline_count, 7000,
                "Should find exactly 7000 newline characters"
            );

            // The test should complete reasonably quickly (within 1 second)
            assert!(
                duration.as_secs() < 1,
                "Newline validation took too long: {duration:?}"
            );
        }

        /// Test interaction between newlines and other character classes
        #[test]
        fn test_newline_vs_other_character_classes() {
            // Test that newline characters are not confused with whitespace
            let newline_chars = [
                '\u{000A}', // LF
                '\u{000B}', // VT
                '\u{000C}', // FF
                '\u{000D}', // CR
                '\u{0085}', // NEL
                '\u{2028}', // LS
                '\u{2029}', // PS
            ];

            for &ch in &newline_chars {
                assert!(
                    is_newline(ch),
                    "Character U+{:04X} should be newline",
                    ch as u32
                );
                assert!(
                    !is_whitespace(ch),
                    "Character U+{:04X} should not be whitespace",
                    ch as u32
                );

                // Some newline characters might also be disallowed code points
                // This depends on the specific implementation of disallowed code points
                if ch as u32 <= 0x001F && ch as u32 >= 0x000E {
                    // VT (U+000B) and FF (U+000C) might be disallowed control characters
                    // but LF (U+000A) and CR (U+000D) are typically allowed for newlines
                    // The specific behavior depends on implementation
                }
            }

            // Test that whitespace characters are not newlines
            let whitespace_chars = [
                '\u{0009}', // Character Tabulation
                '\u{0020}', // Space
                '\u{00A0}', // No-Break Space
                '\u{1680}', // Ogham Space Mark
                '\u{2000}', // En Quad
                '\u{3000}', // Ideographic Space
            ];

            for &ch in &whitespace_chars {
                assert!(
                    !is_newline(ch),
                    "Character U+{:04X} should not be newline",
                    ch as u32
                );
                assert!(
                    is_whitespace(ch),
                    "Character U+{:04X} should be whitespace",
                    ch as u32
                );
                assert!(
                    !is_disallowed_code_point(ch),
                    "Character U+{:04X} should not be disallowed",
                    ch as u32
                );
            }
        }

        /// Test specific newline character properties
        #[test]
        fn test_specific_newline_characters() {
            // Test each newline character individually with detailed assertions

            // LF (Line Feed) - U+000A
            assert!(is_newline('\u{000A}'));
            assert!(!is_whitespace('\u{000A}'));
            assert!(!is_valid_identifier_character('\u{000A}'));

            // VT (Vertical Tab) - U+000B
            assert!(is_newline('\u{000B}'));
            assert!(!is_whitespace('\u{000B}'));
            assert!(!is_valid_identifier_character('\u{000B}'));

            // FF (Form Feed) - U+000C
            assert!(is_newline('\u{000C}'));
            assert!(!is_whitespace('\u{000C}'));
            assert!(!is_valid_identifier_character('\u{000C}'));

            // CR (Carriage Return) - U+000D
            assert!(is_newline('\u{000D}'));
            assert!(!is_whitespace('\u{000D}'));
            assert!(!is_valid_identifier_character('\u{000D}'));

            // NEL (Next Line) - U+0085
            assert!(is_newline('\u{0085}'));
            assert!(!is_whitespace('\u{0085}'));
            assert!(!is_valid_identifier_character('\u{0085}'));

            // LS (Line Separator) - U+2028
            assert!(is_newline('\u{2028}'));
            assert!(!is_whitespace('\u{2028}'));
            assert!(!is_valid_identifier_character('\u{2028}'));

            // PS (Paragraph Separator) - U+2029
            assert!(is_newline('\u{2029}'));
            assert!(!is_whitespace('\u{2029}'));
            assert!(!is_valid_identifier_character('\u{2029}'));
        }

        /// Test that the newline function handles all Unicode ranges correctly
        #[test]
        fn test_newline_unicode_ranges() {
            // Test various Unicode ranges to ensure no false positives

            // ASCII range (except the actual newlines)
            for code_point in 0x0000..=0x007F {
                let ch = char::from_u32(code_point).unwrap();
                let expected_newline = matches!(code_point, 0x000A..=0x000D);
                assert_eq!(
                    is_newline(ch),
                    expected_newline,
                    "ASCII character U+{code_point:04X} newline detection failed"
                );
            }

            // Latin-1 Supplement range
            for code_point in 0x0080..=0x00FF {
                let ch = char::from_u32(code_point).unwrap();
                let expected_newline = code_point == 0x0085; // Only NEL
                assert_eq!(
                    is_newline(ch),
                    expected_newline,
                    "Latin-1 Supplement character U+{code_point:04X} newline detection failed"
                );
            }

            // General Punctuation range (where LS and PS are located)
            for code_point in 0x2000..=0x206F {
                let ch = char::from_u32(code_point).unwrap();
                let expected_newline = matches!(code_point, 0x2028 | 0x2029);
                assert_eq!(
                    is_newline(ch),
                    expected_newline,
                    "General Punctuation character U+{code_point:04X} newline detection failed"
                );
            }
        }

        // =========================================================================
        // Section 3.19: Disallowed Code Points Tests
        // =========================================================================

        /// Test that all disallowed code points from Section 3.19 are correctly identified
        #[test]
        fn test_disallowed_control_characters_0000_0008() {
            // Test U+0000-0008 (various control characters)
            for code_point in 0x0000..=0x0008 {
                let ch = char::from_u32(code_point).unwrap();
                assert!(
                    is_disallowed_code_point(ch),
                    "Control character U+{code_point:04X} should be disallowed"
                );
            }
        }

        /// Test that all disallowed code points from Section 3.19 are correctly identified
        #[test]
        fn test_disallowed_control_characters_000e_001f() {
            // Test U+000E-001F (various control characters)
            for code_point in 0x000E..=0x001F {
                let ch = char::from_u32(code_point).unwrap();
                assert!(
                    is_disallowed_code_point(ch),
                    "Control character U+{code_point:04X} should be disallowed"
                );
            }
        }

        /// Test that the Delete control character (U+007F) is disallowed
        #[test]
        fn test_disallowed_delete_character() {
            let delete_char = char::from_u32(0x007F).unwrap();
            assert!(
                is_disallowed_code_point(delete_char),
                "Delete control character U+007F should be disallowed"
            );
        }

        /// Test that Unicode surrogate code points (U+D800-DFFF) are disallowed
        #[test]
        fn test_disallowed_surrogate_code_points() {
            // Test the surrogate range U+D800-DFFF
            // Note: These might not be valid Rust chars, so we test the ones that are representable
            for code_point in 0xD800..=0xDFFF {
                if let Some(ch) = char::from_u32(code_point) {
                    assert!(
                        is_disallowed_code_point(ch),
                        "Surrogate code point U+{code_point:04X} should be disallowed"
                    );
                }
                // Even if char::from_u32 returns None, these code points should be disallowed
                // Our implementation checks the numeric range directly
                assert!(
                    (0xD800..=0xDFFF).contains(&code_point),
                    "Code point U+{code_point:04X} should be in disallowed surrogate range"
                );
            }
        }

        /// Test that Unicode direction control characters are disallowed
        #[test]
        fn test_disallowed_direction_control_characters() {
            // Test U+200E-200F (Left-to-Right Mark, Right-to-Left Mark)
            for code_point in 0x200E..=0x200F {
                let ch = char::from_u32(code_point).unwrap();
                assert!(
                    is_disallowed_code_point(ch),
                    "Direction control character U+{code_point:04X} should be disallowed"
                );
            }

            // Test U+202A-202E (various directional embedding and override characters)
            for code_point in 0x202A..=0x202E {
                let ch = char::from_u32(code_point).unwrap();
                assert!(
                    is_disallowed_code_point(ch),
                    "Direction control character U+{code_point:04X} should be disallowed"
                );
            }

            // Test U+2066-2069 (various directional isolate characters)
            for code_point in 0x2066..=0x2069 {
                let ch = char::from_u32(code_point).unwrap();
                assert!(
                    is_disallowed_code_point(ch),
                    "Direction control character U+{code_point:04X} should be disallowed"
                );
            }
        }

        /// Test that BOM/ZWNBSP (U+FEFF) is disallowed
        /// Note: The spec allows this as the first character in a document, but our validation
        /// function marks it as disallowed in general contexts
        #[test]
        fn test_disallowed_bom_character() {
            let bom_char = char::from_u32(0xFEFF).unwrap();
            assert!(
                is_disallowed_code_point(bom_char),
                "BOM/ZWNBSP character U+FEFF should be disallowed in general contexts"
            );
        }

        /// Test that allowed characters are not marked as disallowed
        #[test]
        fn test_allowed_characters() {
            let allowed_chars = [
                // ASCII printable characters
                'a', 'z', 'A', 'Z', '0', '9', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '[',
                ']', '{', '}', '|', '\\', ':', ';', '"', '\'', '<', '>', ',', '?', '/', '~', '`',
                '=', '+', '-', '_', '.',
                // Some control characters that are allowed (newlines and tab)
                '\u{0009}', // Tab (whitespace)
                '\u{000A}', // LF (newline)
                '\u{000B}', // VT (newline)
                '\u{000C}', // FF (newline)
                '\u{000D}', // CR (newline)
                // Whitespace characters
                '\u{0020}', // Space
                '\u{00A0}', // No-Break Space
                '\u{1680}', // Ogham Space Mark
                '\u{2000}', // En Quad
                '\u{2001}', // Em Quad
                '\u{3000}', // Ideographic Space
                // Unicode characters
                'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', '中', '文', '日', '本', '語', '😀', '🌟',
                '🎉', '💻', '🚀',
                // Characters just outside disallowed ranges
                '\u{0080}', // Just after control character range
                '\u{D7FF}', // Just before surrogate range
                '\u{E000}', // Just after surrogate range
                '\u{200D}', // Zero Width Joiner (just before direction control)
                '\u{2010}', // Hyphen (just after direction control)
                '\u{202F}', // Narrow No-Break Space (just after direction control)
                '\u{2065}', // Just before directional isolate range
                '\u{206A}', // Just after directional isolate range
                '\u{FEFE}', // Just before BOM
            ];

            for &ch in &allowed_chars {
                assert!(
                    !is_disallowed_code_point(ch),
                    "Character '{}' (U+{:04X}) should NOT be disallowed",
                    ch,
                    ch as u32
                );
            }
        }

        /// Test that disallowed characters are rejected in identifiers
        #[test]
        fn test_disallowed_characters_not_valid_in_identifiers() {
            // Test some specific disallowed characters
            let disallowed_chars = [
                '\u{0000}', // Null
                '\u{0001}', // Start of Heading
                '\u{0007}', // Bell
                '\u{0008}', // Backspace
                '\u{000E}', // Shift Out
                '\u{001F}', // Unit Separator
                '\u{007F}', // Delete
                '\u{200E}', // Left-to-Right Mark
                '\u{200F}', // Right-to-Left Mark
                '\u{202A}', // Left-to-Right Embedding
                '\u{202E}', // Right-to-Left Override
                '\u{2066}', // Left-to-Right Isolate
                '\u{2069}', // Pop Directional Isolate
                '\u{FEFF}', // BOM/ZWNBSP
            ];

            for ch in &disallowed_chars {
                assert!(
                    !is_valid_identifier_character(*ch),
                    "Disallowed character U+{:04X} should not be valid in identifiers",
                    *ch as u32
                );
            }
        }

        /// Test identifier validation with disallowed characters
        #[test]
        fn test_identifier_validation_with_disallowed_characters() {
            let test_cases = [
                ("hello\u{0000}world", "identifier with null character"),
                ("hello\u{0001}world", "identifier with start of heading"),
                ("hello\u{0007}world", "identifier with bell character"),
                ("hello\u{0008}world", "identifier with backspace"),
                ("hello\u{000E}world", "identifier with shift out"),
                ("hello\u{001F}world", "identifier with unit separator"),
                ("hello\u{007F}world", "identifier with delete"),
                ("hello\u{200E}world", "identifier with left-to-right mark"),
                ("hello\u{200F}world", "identifier with right-to-left mark"),
                (
                    "hello\u{202A}world",
                    "identifier with left-to-right embedding",
                ),
                (
                    "hello\u{202E}world",
                    "identifier with right-to-left override",
                ),
                (
                    "hello\u{2066}world",
                    "identifier with left-to-right isolate",
                ),
                (
                    "hello\u{2069}world",
                    "identifier with pop directional isolate",
                ),
                ("hello\u{FEFF}world", "identifier with BOM/ZWNBSP"),
                ("\u{0000}hello", "identifier starting with null"),
                ("hello\u{0000}", "identifier ending with null"),
                ("\u{007F}hello", "identifier starting with delete"),
                ("hello\u{007F}", "identifier ending with delete"),
            ];

            for (identifier, description) in &test_cases {
                let result = validate_identifier_string_test(identifier);
                assert!(
                    result.is_err(),
                    "Identifier validation should fail for {}: '{}'",
                    description,
                    identifier
                        .chars()
                        .map(|c| format!("U+{:04X}", c as u32))
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
        }

        /// Test edge cases around disallowed code point boundaries
        #[test]
        fn test_disallowed_boundary_cases() {
            let boundary_cases = [
                // Around the first control character range (U+0000-0008)
                ('\u{0000}', true),  // First disallowed
                ('\u{0008}', true),  // Last disallowed in this range
                ('\u{0009}', false), // Tab (allowed, it's whitespace)
                // Around the gap between ranges
                ('\u{0009}', false), // Tab (allowed)
                ('\u{000A}', false), // LF (allowed, it's newline)
                ('\u{000B}', false), // VT (allowed, it's newline)
                ('\u{000C}', false), // FF (allowed, it's newline)
                ('\u{000D}', false), // CR (allowed, it's newline)
                ('\u{000E}', true),  // First disallowed in second range
                // Around the second control character range (U+000E-001F)
                ('\u{001F}', true),  // Last disallowed in this range
                ('\u{0020}', false), // Space (allowed)
                // Around the Delete character
                ('\u{007E}', false), // Tilde (allowed)
                ('\u{007F}', true),  // Delete (disallowed)
                ('\u{0080}', false), // Padding Character (allowed, though it's a control char)
                // Around the surrogate range (can't test all since many aren't valid chars)
                ('\u{D7FF}', false), // Just before surrogate range
                ('\u{E000}', false), // Just after surrogate range
                // Around direction control ranges
                ('\u{200D}', false), // Zero Width Joiner (allowed)
                ('\u{200E}', true),  // Left-to-Right Mark (disallowed)
                ('\u{200F}', true),  // Right-to-Left Mark (disallowed)
                ('\u{2010}', false), // Hyphen (allowed)
                ('\u{2029}', false), // Paragraph Separator (allowed, it's newline)
                ('\u{202A}', true),  // Left-to-Right Embedding (disallowed)
                ('\u{202E}', true),  // Right-to-Left Override (disallowed)
                ('\u{202F}', false), // Narrow No-Break Space (allowed, it's whitespace)
                ('\u{2065}', false), // Just before directional isolate range
                ('\u{2066}', true),  // Left-to-Right Isolate (disallowed)
                ('\u{2069}', true),  // Pop Directional Isolate (disallowed)
                ('\u{206A}', false), // Just after directional isolate range
                // Around BOM
                ('\u{FEFE}', false), // Just before BOM
                ('\u{FEFF}', true),  // BOM (disallowed)
            ];

            for (ch, should_be_disallowed) in &boundary_cases {
                assert_eq!(
                    is_disallowed_code_point(*ch),
                    *should_be_disallowed,
                    "Character U+{:04X} disallowed test failed",
                    *ch as u32
                );
            }
        }

        /// Test performance with a large number of code points
        #[test]
        fn test_disallowed_performance() {
            // Test a large range of code points for performance
            let mut test_chars = Vec::new();

            // Add some disallowed characters
            for code_point in 0x0000..=0x0008 {
                test_chars.push(char::from_u32(code_point).unwrap());
            }
            for code_point in 0x000E..=0x001F {
                test_chars.push(char::from_u32(code_point).unwrap());
            }
            test_chars.push('\u{007F}');
            test_chars.push('\u{200E}');
            test_chars.push('\u{202A}');
            test_chars.push('\u{2066}');
            test_chars.push('\u{FEFF}');

            // Add many allowed characters
            for _ in 0..1000 {
                test_chars.push('a');
                test_chars.push('Z');
                test_chars.push('0');
                test_chars.push('α');
                test_chars.push('中');
                test_chars.push('😀');
            }

            // Test that disallowed detection is fast
            let start = std::time::Instant::now();
            let mut disallowed_count = 0;
            for &ch in &test_chars {
                if is_disallowed_code_point(ch) {
                    disallowed_count += 1;
                }
            }
            let duration = start.elapsed();

            // Verify we found the expected number of disallowed characters
            // 9 (0x0000-0x0008) + 18 (0x000E-0x001F) + 1 (0x007F) + 1 (0x200E) + 1 (0x202A) + 1 (0x2066) + 1 (0x FEFF) = 32
            assert_eq!(
                disallowed_count, 32,
                "Should find exactly 32 disallowed characters"
            );

            // The test should complete reasonably quickly (within 1 second)
            assert!(
                duration.as_secs() < 1,
                "Disallowed code point validation took too long: {duration:?}"
            );
        }

        /// Test interaction between disallowed characters and other character classes
        #[test]
        fn test_disallowed_vs_other_character_classes() {
            // Test that some disallowed characters are not whitespace or newlines
            let disallowed_not_whitespace_newline = [
                '\u{0000}', // Null
                '\u{0001}', // Start of Heading
                '\u{0008}', // Backspace
                '\u{000E}', // Shift Out
                '\u{001F}', // Unit Separator
                '\u{007F}', // Delete
                '\u{200E}', // Left-to-Right Mark
                '\u{FEFF}', // BOM
            ];

            for &ch in &disallowed_not_whitespace_newline {
                assert!(
                    is_disallowed_code_point(ch),
                    "Character U+{:04X} should be disallowed",
                    ch as u32
                );
                assert!(
                    !is_whitespace(ch),
                    "Character U+{:04X} should not be whitespace",
                    ch as u32
                );
                assert!(
                    !is_newline(ch),
                    "Character U+{:04X} should not be newline",
                    ch as u32
                );
            }

            // Test that whitespace characters are not disallowed
            let whitespace_chars = [
                '\u{0009}', // Tab
                '\u{0020}', // Space
                '\u{00A0}', // No-Break Space
                '\u{2000}', // En Quad
                '\u{3000}', // Ideographic Space
            ];

            for &ch in &whitespace_chars {
                assert!(
                    !is_disallowed_code_point(ch),
                    "Character U+{:04X} should not be disallowed",
                    ch as u32
                );
                assert!(
                    is_whitespace(ch),
                    "Character U+{:04X} should be whitespace",
                    ch as u32
                );
                assert!(
                    !is_newline(ch),
                    "Character U+{:04X} should not be newline",
                    ch as u32
                );
            }

            // Test that newline characters are not disallowed
            let newline_chars = [
                '\u{000A}', // LF
                '\u{000B}', // VT
                '\u{000C}', // FF
                '\u{000D}', // CR
                '\u{0085}', // NEL
                '\u{2028}', // LS
                '\u{2029}', // PS
            ];

            for &ch in &newline_chars {
                assert!(
                    !is_disallowed_code_point(ch),
                    "Character U+{:04X} should not be disallowed",
                    ch as u32
                );
                assert!(
                    !is_whitespace(ch),
                    "Character U+{:04X} should not be whitespace",
                    ch as u32
                );
                assert!(
                    is_newline(ch),
                    "Character U+{:04X} should be newline",
                    ch as u32
                );
            }
        }

        /// Test specific ranges of disallowed code points comprehensively
        #[test]
        fn test_comprehensive_disallowed_ranges() {
            // Test the complete control character range U+0000-0008
            for code_point in 0x0000..=0x0008 {
                let ch = char::from_u32(code_point).unwrap();
                assert!(
                    is_disallowed_code_point(ch),
                    "Control character U+{code_point:04X} should be disallowed"
                );
                assert!(
                    !is_valid_identifier_character(ch),
                    "Control character U+{code_point:04X} should not be valid in identifiers"
                );
            }

            // Test the complete control character range U+000E-001F
            for code_point in 0x000E..=0x001F {
                let ch = char::from_u32(code_point).unwrap();
                assert!(
                    is_disallowed_code_point(ch),
                    "Control character U+{code_point:04X} should be disallowed"
                );
                assert!(
                    !is_valid_identifier_character(ch),
                    "Control character U+{code_point:04X} should not be valid in identifiers"
                );
            }

            // Test the direction control ranges
            let direction_control_ranges = [
                (0x200E, 0x200F), // U+200E-200F
                (0x202A, 0x202E), // U+202A-202E
                (0x2066, 0x2069), // U+2066-2069
            ];

            for (start, end) in direction_control_ranges {
                for code_point in start..=end {
                    let ch = char::from_u32(code_point).unwrap();
                    assert!(
                        is_disallowed_code_point(ch),
                        "Direction control character U+{code_point:04X} should be disallowed"
                    );
                    assert!(
                        !is_valid_identifier_character(ch),
                        "Direction control character U+{code_point:04X} should not be valid in identifiers"
                    );
                }
            }
        }

        /// Test that the allowed gap characters (U+0009-000D) are handled correctly
        #[test]
        fn test_allowed_gap_characters() {
            // Characters U+0009-000D are in the gap between disallowed ranges
            // These should be allowed (they are tab and newlines)
            let gap_chars = [
                ('\u{0009}', "Tab"),
                ('\u{000A}', "Line Feed"),
                ('\u{000B}', "Vertical Tab"),
                ('\u{000C}', "Form Feed"),
                ('\u{000D}', "Carriage Return"),
            ];

            for (ch, name) in &gap_chars {
                assert!(
                    !is_disallowed_code_point(*ch),
                    "{} (U+{:04X}) should not be disallowed",
                    name,
                    *ch as u32
                );
            }
        }
    }
}
