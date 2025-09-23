//! Tests for Section 3.11: Quoted String
//!
//! This module tests the implementation of quoted string parsing according to the KDL specification.

use super::*;
use proc_macro2::Span;

/// Test all standard escape sequences from Section 3.11.1
#[test]
fn test_standard_escape_sequences() {
    let test_cases = vec![
        (r"\n", "\u{000A}"),   // Line Feed
        (r"\r", "\u{000D}"),   // Carriage Return
        (r"\t", "\u{0009}"),   // Character Tabulation (Tab)
        (r"\\", "\u{005C}"),   // Reverse Solidus (Backslash)
        (r#"\""#, "\u{0022}"), // Quotation Mark (Double Quote)
        (r"\b", "\u{0008}"),   // Backspace
        (r"\f", "\u{000C}"),   // Form Feed
        (r"\s", "\u{0020}"),   // Space
    ];

    for (input, expected) in test_cases {
        let result = crate::parse::string::process_string_escapes(input, Span::call_site());
        assert!(
            result.is_ok(),
            "Failed to process escape sequence: {}",
            input
        );
        assert_eq!(
            result.unwrap(),
            expected,
            "Escape sequence {} didn't match expected output",
            input
        );
    }
}

/// Test Unicode escape sequences
#[test]
fn test_unicode_escape_sequences() {
    let test_cases = vec![
        (r"\u{41}", "A"),              // Simple ASCII
        (r"\u{1F600}", "😀"),          // Emoji
        (r"\u{0}", "\u{0000}"),        // Null character
        (r"\u{10FFFF}", "\u{10FFFF}"), // Maximum Unicode code point
    ];

    for (input, expected) in test_cases {
        let result = crate::parse::string::process_string_escapes(input, Span::call_site());
        assert!(
            result.is_ok(),
            "Failed to process Unicode escape: {}",
            input
        );
        assert_eq!(
            result.unwrap(),
            expected,
            "Unicode escape {} didn't match expected output",
            input
        );
    }
}

/// Test escaped whitespace functionality from Section 3.11.1.1
#[test]
fn test_escaped_whitespace() {
    let test_cases = vec![
        (r"Hello\    World", "HelloWorld"),       // Spaces
        (r"Hello\	World", "HelloWorld"),          // Tab
        (r"Hello\ World", "HelloWorld"),          // Single space
        (r"Hello\        	 World", "HelloWorld"), // Mixed spaces and tabs
        (
            r"Hello\
World",
            "HelloWorld",
        ), // Newline (literal newline in string)
    ];

    for (input, expected) in test_cases {
        let result = crate::parse::string::process_string_escapes(input, Span::call_site());
        assert!(
            result.is_ok(),
            "Failed to process escaped whitespace: {:?}",
            input
        );
        assert_eq!(
            result.unwrap(),
            expected,
            "Escaped whitespace {:?} didn't match expected output",
            input
        );
    }
}

/// Test that escaped whitespace doesn't affect escape sequences
#[test]
fn test_escaped_whitespace_preserves_escapes() {
    let test_cases = vec![
        (r"Hello\       \nWorld", "Hello\nWorld"), // \n should be preserved
        (r"Hello\n\    World", "Hello\nWorld"),    // Both \n and escaped whitespace
        (r"Hello\t\     \rWorld", "Hello\t\rWorld"), // Multiple escapes with whitespace
    ];

    for (input, expected) in test_cases {
        let result = crate::parse::string::process_string_escapes(input, Span::call_site());
        assert!(
            result.is_ok(),
            "Failed to process mixed escapes and whitespace: {:?}",
            input
        );
        assert_eq!(
            result.unwrap(),
            expected,
            "Mixed escapes {:?} didn't match expected output",
            input
        );
    }
}

/// Test invalid escape sequences from Section 3.11.1.2
#[test]
fn test_invalid_escape_sequences() {
    let invalid_escapes = vec![
        r"\x", // Invalid escape character
        r"\a", // Invalid escape character
        r"\z", // Invalid escape character
        r"\1", // Invalid escape character
        r"\@", // Invalid escape character
        r"\",  // Backslash at end of string
    ];

    for input in invalid_escapes {
        let result = crate::parse::string::process_string_escapes(input, Span::call_site());
        assert!(
            result.is_err(),
            "Expected error for invalid escape sequence: {:?}",
            input
        );
    }
}

/// Test invalid Unicode escape sequences
#[test]
fn test_invalid_unicode_escapes() {
    let invalid_unicode = vec![
        r"\u{",         // Incomplete
        r"\u{1234567}", // Too many hex digits (7, max is 6)
        r"\u{}",        // Empty
        r"\u{GGGG}",    // Invalid hex characters
        r"\u{110000}",  // Beyond Unicode range
        r"\u{D800}",    // Surrogate (not a Unicode scalar value)
        r"\u{DFFF}",    // Surrogate (not a Unicode scalar value)
    ];

    for input in invalid_unicode {
        let result = crate::parse::string::process_string_escapes(input, Span::call_site());
        assert!(
            result.is_err(),
            "Expected error for invalid Unicode escape: {:?}",
            input
        );
    }
}

/// Test complex strings with multiple escape types
#[test]
fn test_complex_escape_combinations() {
    let test_cases = vec![
        (r"Hello\nWorld\tTest\u{41}", "Hello\nWorld\tTestA"),
        (
            r#"Quote: \"Hello\", Backslash: \\"#,
            r#"Quote: "Hello", Backslash: \"#,
        ),
        (r"Tab:\t\    Space:\s\nLine", "Tab:\tSpace: \nLine"),
        (r"\u{48}\u{65}\u{6C}\u{6C}\u{6F}", "Hello"), // "Hello" in Unicode escapes
    ];

    for (input, expected) in test_cases {
        let result = crate::parse::string::process_string_escapes(input, Span::call_site());
        assert!(
            result.is_ok(),
            "Failed to process complex escape combination: {:?}",
            input
        );
        assert_eq!(
            result.unwrap(),
            expected,
            "Complex escape {:?} didn't match expected output",
            input
        );
    }
}

/// Test edge cases
#[test]
fn test_edge_cases() {
    let test_cases = vec![
        ("", ""),                            // Empty string
        ("Hello", "Hello"),                  // No escapes
        (r"\u{0}\u{0}", "\u{0000}\u{0000}"), // Multiple null characters
        (r"\s\s\s", "   "),                  // Multiple spaces
        (r"\t\n\r", "\t\n\r"),               // Multiple different escapes
    ];

    for (input, expected) in test_cases {
        let result = crate::parse::string::process_string_escapes(input, Span::call_site());
        assert!(result.is_ok(), "Failed to process edge case: {:?}", input);
        assert_eq!(
            result.unwrap(),
            expected,
            "Edge case {:?} didn't match expected output",
            input
        );
    }
}

/// Test the examples from the specification
#[test]
fn test_specification_examples() {
    // From Section 3.11.1.1 - semantically identical strings
    let semantically_identical = vec!["Hello World", r"Hello \    World"];

    let expected = "Hello World";

    for input in semantically_identical {
        let result = crate::parse::string::process_string_escapes(input, Span::call_site());
        assert!(
            result.is_ok(),
            "Failed to process spec example: {:?}",
            input
        );
        assert_eq!(
            result.unwrap(),
            expected,
            "Spec example {:?} didn't match expected output",
            input
        );
    }

    // Complex example from specification
    let complex_examples = vec![
        r"Hello\       \nWorld",
        r"Hello\n\
    World",
        r"Hello\nWorld",
    ];

    let expected_complex = "Hello\nWorld";

    for input in complex_examples {
        let result = crate::parse::string::process_string_escapes(input, Span::call_site());
        assert!(
            result.is_ok(),
            "Failed to process complex spec example: {:?}",
            input
        );
        assert_eq!(
            result.unwrap(),
            expected_complex,
            "Complex spec example {:?} didn't match expected output",
            input
        );
    }
}

/// Integration test: test with actual KDL parsing
#[test]
fn test_quoted_string_integration() {
    let kdl_input = quote::quote! {
        node "Hello\nWorld" key="value\ttab" "another\u{41}string"
    };

    let result = kdl_impl2(kdl_input);
    assert!(
        result.is_ok(),
        "KDL parsing with quoted strings should succeed"
    );

    // The generated code should compile and work correctly
    let generated = result.unwrap();
    // We can't easily test the exact output here, but we can verify it parses
    assert!(!generated.is_empty(), "Generated code should not be empty");
}

/// Test error messages are helpful
#[test]
fn test_error_messages() {
    let test_cases = vec![
        (r"\x", "Invalid escape sequence"),
        (r"\u{GGGG}", "Invalid character"),
        (r"\u{}", "Empty Unicode escape"),
        (r"\", "backslash at end of string"),
    ];

    for (input, expected_msg_part) in test_cases {
        let result = crate::parse::string::process_string_escapes(input, Span::call_site());
        assert!(result.is_err(), "Expected error for: {:?}", input);
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg
                .to_lowercase()
                .contains(&expected_msg_part.to_lowercase()),
            "Error message '{}' should contain '{}'",
            error_msg,
            expected_msg_part
        );
    }
}
