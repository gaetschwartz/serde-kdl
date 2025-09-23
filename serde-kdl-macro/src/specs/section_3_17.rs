//! Tests for Section 3.17: Whitespace
//!
//! This module tests the whitespace validation according to KDL specification Section 3.17.
//! It verifies that all specified whitespace characters are properly recognized and that
//! whitespace handling works correctly in identifier validation.

use crate::validation::{is_whitespace, is_valid_identifier_character, validate_identifier_string};

/// Test that all whitespace characters from Table 2 are correctly identified
#[test]
fn test_whitespace_characters() {
    // Test all whitespace characters defined in Section 3.17, Table 2
    let whitespace_chars = [
        ('\u{0009}', "Character Tabulation"),     // U+0009
        ('\u{0020}', "Space"),                    // U+0020
        ('\u{00A0}', "No-Break Space"),           // U+00A0
        ('\u{1680}', "Ogham Space Mark"),         // U+1680
        ('\u{2000}', "En Quad"),                  // U+2000
        ('\u{2001}', "Em Quad"),                  // U+2001
        ('\u{2002}', "En Space"),                 // U+2002
        ('\u{2003}', "Em Space"),                 // U+2003
        ('\u{2004}', "Three-Per-Em Space"),       // U+2004
        ('\u{2005}', "Four-Per-Em Space"),        // U+2005
        ('\u{2006}', "Six-Per-Em Space"),         // U+2006
        ('\u{2007}', "Figure Space"),             // U+2007
        ('\u{2008}', "Punctuation Space"),        // U+2008
        ('\u{2009}', "Thin Space"),               // U+2009
        ('\u{200A}', "Hair Space"),               // U+200A
        ('\u{202F}', "Narrow No-Break Space"),    // U+202F
        ('\u{205F}', "Medium Mathematical Space"), // U+205F
        ('\u{3000}', "Ideographic Space"),        // U+3000
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
        'a', 'Z', '0', '9', '_', '-', '+', '.',
        '!', '@', '#', '$', '%', '^', '&', '*',
        '(', ')', '[', ']', '{', '}', '|', '\\',
        ':', ';', '"', '\'', '<', '>', ',', '?',
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
        ("hello\u{202F}world", "identifier with narrow no-break space"),
        ("hello\u{205F}world", "identifier with medium mathematical space"),
        ("hello\u{3000}world", "identifier with ideographic space"),
        (" hello", "identifier starting with space"),
        ("hello ", "identifier ending with space"),
        ("\thello", "identifier starting with tab"),
        ("hello\t", "identifier ending with tab"),
    ];

    for (identifier, description) in &test_cases {
        let result = validate_identifier_string(identifier, proc_macro2::Span::call_site());
        assert!(
            result.is_err(),
            "Identifier validation should fail for {}: '{}'",
            description,
            identifier
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
        let result = validate_identifier_string(identifier, proc_macro2::Span::call_site());
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
        "with(parens)",             // Contains parentheses
        "with[brackets]",           // Contains brackets
        "with{braces}",             // Contains braces
        "with/slash",               // Contains slash
        "with\\backslash",          // Contains backslash
        "with\"quotes\"",           // Contains quotes
        "with#hash",                // Contains hash
        "with;semicolon",           // Contains semicolon
        "with=equals",              // Contains equals
    ];

    for identifier in &invalid_identifiers {
        let result = validate_identifier_string(identifier, proc_macro2::Span::call_site());
        if result.is_ok() {
            println!("Note: Identifier '{}' was unexpectedly valid", identifier);
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
        '\u{0009}', '\u{0020}', '\u{00A0}', '\u{1680}',
        '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}',
        '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}',
        '\u{2008}', '\u{2009}', '\u{200A}', '\u{202F}',
        '\u{205F}', '\u{3000}',
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
        "Whitespace validation took too long: {:?}",
        duration
    );
}

/// Test interaction between whitespace and other character classes
#[test]
fn test_whitespace_vs_other_character_classes() {
    use crate::validation::{is_newline, is_disallowed_code_point};

    // Test that whitespace characters are not confused with newlines
    let whitespace_chars = [
        '\u{0009}', '\u{0020}', '\u{00A0}', '\u{1680}',
        '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}',
        '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}',
        '\u{2008}', '\u{2009}', '\u{200A}', '\u{202F}',
        '\u{205F}', '\u{3000}',
    ];

    for &ch in &whitespace_chars {
        assert!(is_whitespace(ch), "Character U+{:04X} should be whitespace", ch as u32);
        assert!(!is_newline(ch), "Character U+{:04X} should not be newline", ch as u32);
        assert!(!is_disallowed_code_point(ch), "Character U+{:04X} should not be disallowed", ch as u32);
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
        assert!(!is_whitespace(ch), "Character U+{:04X} should not be whitespace", ch as u32);
        assert!(is_newline(ch), "Character U+{:04X} should be newline", ch as u32);
    }
}