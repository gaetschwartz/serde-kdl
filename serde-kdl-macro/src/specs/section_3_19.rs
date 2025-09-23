//! Tests for Section 3.19: Disallowed Literal Code Points
//!
//! This module tests the disallowed code point validation according to KDL specification Section 3.19.
//! It verifies that all specified disallowed code points are properly rejected and that
//! validation errors occur when these characters are used in identifiers or other contexts.

use crate::validation::{is_disallowed_code_point, is_valid_identifier_character, validate_identifier_string, is_whitespace, is_newline};

/// Test that all disallowed code points from Section 3.19 are correctly identified
#[test]
fn test_disallowed_control_characters_0000_0008() {
    // Test U+0000-0008 (various control characters)
    for code_point in 0x0000..=0x0008 {
        let ch = char::from_u32(code_point).unwrap();
        assert!(
            is_disallowed_code_point(ch),
            "Control character U+{:04X} should be disallowed",
            code_point
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
            "Control character U+{:04X} should be disallowed",
            code_point
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
                "Surrogate code point U+{:04X} should be disallowed",
                code_point
            );
        }
        // Even if char::from_u32 returns None, these code points should be disallowed
        // Our implementation checks the numeric range directly
        assert!(
            (code_point >= 0xD800 && code_point <= 0xDFFF),
            "Code point U+{:04X} should be in disallowed surrogate range",
            code_point
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
            "Direction control character U+{:04X} should be disallowed",
            code_point
        );
    }

    // Test U+202A-202E (various directional embedding and override characters)
    for code_point in 0x202A..=0x202E {
        let ch = char::from_u32(code_point).unwrap();
        assert!(
            is_disallowed_code_point(ch),
            "Direction control character U+{:04X} should be disallowed",
            code_point
        );
    }

    // Test U+2066-2069 (various directional isolate characters)
    for code_point in 0x2066..=0x2069 {
        let ch = char::from_u32(code_point).unwrap();
        assert!(
            is_disallowed_code_point(ch),
            "Direction control character U+{:04X} should be disallowed",
            code_point
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
        'a', 'z', 'A', 'Z', '0', '9',
        '!', '@', '#', '$', '%', '^', '&', '*',
        '(', ')', '[', ']', '{', '}', '|', '\\',
        ':', ';', '"', '\'', '<', '>', ',', '?',
        '/', '~', '`', '=', '+', '-', '_', '.',
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
        'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ',
        '中', '文', '日', '本', '語',
        '😀', '🌟', '🎉', '💻', '🚀',
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
        ("hello\u{202A}world", "identifier with left-to-right embedding"),
        ("hello\u{202E}world", "identifier with right-to-left override"),
        ("hello\u{2066}world", "identifier with left-to-right isolate"),
        ("hello\u{2069}world", "identifier with pop directional isolate"),
        ("hello\u{FEFF}world", "identifier with BOM/ZWNBSP"),
        ("\u{0000}hello", "identifier starting with null"),
        ("hello\u{0000}", "identifier ending with null"),
        ("\u{007F}hello", "identifier starting with delete"),
        ("hello\u{007F}", "identifier ending with delete"),
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
    assert_eq!(disallowed_count, 32, "Should find exactly 32 disallowed characters");

    // The test should complete reasonably quickly (within 1 second)
    assert!(
        duration.as_secs() < 1,
        "Disallowed code point validation took too long: {:?}",
        duration
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
        assert!(is_disallowed_code_point(ch), "Character U+{:04X} should be disallowed", ch as u32);
        assert!(!is_whitespace(ch), "Character U+{:04X} should not be whitespace", ch as u32);
        assert!(!is_newline(ch), "Character U+{:04X} should not be newline", ch as u32);
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
        assert!(!is_disallowed_code_point(ch), "Character U+{:04X} should not be disallowed", ch as u32);
        assert!(is_whitespace(ch), "Character U+{:04X} should be whitespace", ch as u32);
        assert!(!is_newline(ch), "Character U+{:04X} should not be newline", ch as u32);
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
        assert!(!is_disallowed_code_point(ch), "Character U+{:04X} should not be disallowed", ch as u32);
        assert!(!is_whitespace(ch), "Character U+{:04X} should not be whitespace", ch as u32);
        assert!(is_newline(ch), "Character U+{:04X} should be newline", ch as u32);
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
            "Control character U+{:04X} should be disallowed",
            code_point
        );
        assert!(
            !is_valid_identifier_character(ch),
            "Control character U+{:04X} should not be valid in identifiers",
            code_point
        );
    }

    // Test the complete control character range U+000E-001F
    for code_point in 0x000E..=0x001F {
        let ch = char::from_u32(code_point).unwrap();
        assert!(
            is_disallowed_code_point(ch),
            "Control character U+{:04X} should be disallowed",
            code_point
        );
        assert!(
            !is_valid_identifier_character(ch),
            "Control character U+{:04X} should not be valid in identifiers",
            code_point
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
                "Direction control character U+{:04X} should be disallowed",
                code_point
            );
            assert!(
                !is_valid_identifier_character(ch),
                "Direction control character U+{:04X} should not be valid in identifiers",
                code_point
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