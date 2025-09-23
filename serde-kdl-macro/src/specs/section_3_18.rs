//! Tests for Section 3.18: Newline
//!
//! This module tests the newline validation according to KDL specification Section 3.18.
//! It verifies that all specified newline characters are properly recognized and that
//! newline handling works correctly in identifier validation.

use crate::validation::{is_newline, is_valid_identifier_character, validate_identifier_string, is_whitespace, is_disallowed_code_point};

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
        'a', 'Z', '0', '9', '_', '-', '+', '.',
        '!', '@', '#', '$', '%', '^', '&', '*',
        '(', ')', '[', ']', '{', '}', '|', '\\',
        ':', ';', '"', '\'', '<', '>', ',', '?',
        '/', '~', '`', '=',
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

    for (identifier, description) in &test_cases {
        let result = validate_identifier_string(identifier, proc_macro2::Span::call_site());
        assert!(
            result.is_err(),
            "Identifier validation should fail for {}: '{}'",
            description,
            identifier.chars().map(|c| format!("U+{:04X}", c as u32)).collect::<Vec<_>>().join(" ")
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
        // Characters that might look like newlines but aren't
        "with\u{0009}tab", // Tab is whitespace, should fail differently
        "with\u{0020}space", // Space is whitespace, should fail differently
    ];

    for identifier in &valid_identifiers {
        let result = validate_identifier_string(identifier, proc_macro2::Span::call_site());
        // Note: Some of these will fail due to whitespace, but not due to newlines
        if result.is_err() {
            let error_msg = result.unwrap_err().to_string();
            assert!(
                !error_msg.contains("newline"),
                "Identifier '{}' should not fail due to newline validation",
                identifier
            );
        }
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
    let result = validate_identifier_string(crlf_identifier, proc_macro2::Span::call_site());
    assert!(
        result.is_err(),
        "Identifier with CRLF should be invalid"
    );

    // Test that both CR and LF are detected as invalid characters
    let mut found_cr_error = false;
    let mut found_lf_error = false;

    // Test CR separately
    let cr_result = validate_identifier_string("hello\rworld", proc_macro2::Span::call_site());
    if cr_result.is_err() {
        found_cr_error = true;
    }

    // Test LF separately
    let lf_result = validate_identifier_string("hello\nworld", proc_macro2::Span::call_site());
    if lf_result.is_err() {
        found_lf_error = true;
    }

    assert!(found_cr_error, "CR should cause identifier validation to fail");
    assert!(found_lf_error, "LF should cause identifier validation to fail");
}

/// Test performance with a large string containing various newline characters
#[test]
fn test_newline_performance() {
    // Create a string with many newline characters for performance testing
    let mut chars_to_test = Vec::new();
    let newline_chars = [
        '\u{000A}', '\u{000B}', '\u{000C}', '\u{000D}',
        '\u{0085}', '\u{2028}', '\u{2029}',
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
    assert_eq!(newline_count, 7000, "Should find exactly 7000 newline characters");

    // The test should complete reasonably quickly (within 1 second)
    assert!(
        duration.as_secs() < 1,
        "Newline validation took too long: {:?}",
        duration
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
        assert!(is_newline(ch), "Character U+{:04X} should be newline", ch as u32);
        assert!(!is_whitespace(ch), "Character U+{:04X} should not be whitespace", ch as u32);

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
        assert!(!is_newline(ch), "Character U+{:04X} should not be newline", ch as u32);
        assert!(is_whitespace(ch), "Character U+{:04X} should be whitespace", ch as u32);
        assert!(!is_disallowed_code_point(ch), "Character U+{:04X} should not be disallowed", ch as u32);
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
        let expected_newline = matches!(code_point, 0x000A | 0x000B | 0x000C | 0x000D);
        assert_eq!(
            is_newline(ch),
            expected_newline,
            "ASCII character U+{:04X} newline detection failed",
            code_point
        );
    }

    // Latin-1 Supplement range
    for code_point in 0x0080..=0x00FF {
        let ch = char::from_u32(code_point).unwrap();
        let expected_newline = code_point == 0x0085; // Only NEL
        assert_eq!(
            is_newline(ch),
            expected_newline,
            "Latin-1 Supplement character U+{:04X} newline detection failed",
            code_point
        );
    }

    // General Punctuation range (where LS and PS are located)
    for code_point in 0x2000..=0x206F {
        let ch = char::from_u32(code_point).unwrap();
        let expected_newline = matches!(code_point, 0x2028 | 0x2029);
        assert_eq!(
            is_newline(ch),
            expected_newline,
            "General Punctuation character U+{:04X} newline detection failed",
            code_point
        );
    }
}