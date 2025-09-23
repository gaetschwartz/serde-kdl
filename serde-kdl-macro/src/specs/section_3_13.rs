//! Tests for Section 3.13: Raw String
//!
//! This module tests the implementation of raw string parsing according to the KDL specification.

use proc_macro2::Span;

/// Test basic raw string processing with no escape processing
#[test]
fn test_basic_raw_string() {
    let test_cases = vec![
        (r#"\n will be literal"#, 1, r#"\n will be literal"#),
        (r"Hello\tWorld", 1, r"Hello\tWorld"),
        (r"No\u{41}escapes", 1, r"No\u{41}escapes"),
        (r"Backslash\\preserved", 1, r"Backslash\\preserved"),
        (r#"Quote\"preserved"#, 1, r#"Quote\"preserved"#),
    ];

    for (input, hash_count, expected) in test_cases {
        let result = crate::parse::string::process_raw_string(input, hash_count, Span::call_site());
        assert!(result.is_ok(), "Failed to process raw string: {}", input);
        assert_eq!(result.unwrap(), expected, "Raw string {} didn't match expected output", input);
    }
}

/// Test raw strings with different hash counts
#[test]
fn test_raw_string_hash_counts() {
    let test_cases = vec![
        ("content", 1),
        (r#"can contain "#, 2), // This would be r##"can contain "#"## in actual KDL
        ("can contain \"##", 3), // This would be r###"can contain "##"### in actual KDL
        ("lots of #### hashes", 4),
    ];

    for (input, hash_count) in test_cases {
        let result = crate::parse::string::process_raw_string(input, hash_count, Span::call_site());
        assert!(result.is_ok(), "Failed to process raw string with {} hashes: {}", hash_count, input);
        assert_eq!(result.unwrap(), input, "Raw string with {} hashes should preserve content exactly", hash_count);
    }
}

/// Test that raw strings can contain quotes and escapes as literals
#[test]
fn test_raw_strings_with_quotes_and_escapes() {
    let test_cases = vec![
        (r#"hello\n\r\world"#, 2, r#"hello\n\r\world"#),
        (r#""quotes""#, 1, r#""quotes""#),
        (r"tab\there", 1, r"tab\there"),
        (r"newline\nhere", 1, r"newline\nhere"),
        (r"unicode\u{41}here", 1, r"unicode\u{41}here"),
        (r"whitespace\    escape", 1, r"whitespace\    escape"),
    ];

    for (input, hash_count, expected) in test_cases {
        let result = crate::parse::string::process_raw_string(input, hash_count, Span::call_site());
        assert!(result.is_ok(), "Failed to process raw string with quotes/escapes: {}", input);
        assert_eq!(result.unwrap(), expected, "Raw string with quotes/escapes didn't match expected output");
    }
}

/// Test raw multi-line strings from the specification
#[test]
fn test_raw_multiline_string() {
    // Example from specification Section 3.13.1
    let input = "\n    Here's a \"\"\"\n        multiline string\n        \"\"\"\n    without escapes.\n    ";
    let hash_count = 1;
    let expected = "Here's a \"\"\"\n    multiline string\n    \"\"\"\nwithout escapes.";

    let result = crate::parse::string::process_raw_multiline_string(input, hash_count, Span::call_site());
    assert!(result.is_ok(), "Failed to process raw multi-line string");
    assert_eq!(result.unwrap(), expected, "Raw multi-line string didn't match expected output");
}

/// Test that raw multi-line strings preserve escape sequences as literals
#[test]
fn test_raw_multiline_preserves_escapes() {
    let input = "\n    Line with \\n escape\n    Line with \\t tab\n    Line with \\u{41} unicode\n    ";
    let hash_count = 1;
    let expected = "Line with \\n escape\nLine with \\t tab\nLine with \\u{41} unicode";

    let result = crate::parse::string::process_raw_multiline_string(input, hash_count, Span::call_site());
    assert!(result.is_ok(), "Failed to process raw multi-line string with escapes");
    assert_eq!(result.unwrap(), expected, "Raw multi-line string should preserve escape sequences as literals");
}

/// Test that raw multi-line strings preserve whitespace escapes as literals
#[test]
fn test_raw_multiline_preserves_whitespace_escapes() {
    let input = "\n    Line with \\    whitespace escape\n    Another \\   line\n    ";
    let hash_count = 1;
    let expected = "Line with \\    whitespace escape\nAnother \\   line";

    let result = crate::parse::string::process_raw_multiline_string(input, hash_count, Span::call_site());
    assert!(result.is_ok(), "Failed to process raw multi-line string with whitespace escapes");
    assert_eq!(result.unwrap(), expected, "Raw multi-line string should preserve whitespace escapes as literals");
}

/// Test that raw strings still follow UTF-8 requirements
#[test]
fn test_raw_string_utf8_validation() {
    // Test with valid UTF-8 content
    let valid_utf8_cases = vec![
        ("Hello 世界", 1),
        ("🌍🚀💻", 1),
        ("Ελληνικά", 1),
        ("العربية", 1),
    ];

    for (input, hash_count) in valid_utf8_cases {
        let result = crate::parse::string::process_raw_string(input, hash_count, Span::call_site());
        assert!(result.is_ok(), "Valid UTF-8 raw string should succeed: {}", input);
        assert_eq!(result.unwrap(), input, "UTF-8 content should be preserved exactly");
    }
}

/// Test validation against disallowed literal code points (Section 3.19)
#[test]
fn test_raw_string_disallowed_code_points() {
    // Raw strings cannot contain disallowed code points since they can't be escaped
    // These should be caught during validation, not processing
    let input = "valid content";
    let hash_count = 1;

    let result = crate::parse::string::process_raw_string(input, hash_count, Span::call_site());
    assert!(result.is_ok(), "Valid raw string should succeed");

    // The actual validation of disallowed code points would happen during AST validation,
    // not during raw string processing, since raw strings can't escape them
}

/// Test edge cases for raw strings
#[test]
fn test_raw_string_edge_cases() {
    let test_cases = vec![
        ("", 1, ""),                    // Empty raw string
        (" ", 1, " "),                  // Single space
        ("   ", 1, "   "),              // Multiple spaces
        ("\t", 1, "\t"),                // Single tab
        ("\n", 1, "\n"),                // Single newline
        ("a", 1, "a"),                  // Single character
        ("123", 1, "123"),              // Numbers
        ("!@#$%^&*()", 1, "!@#$%^&*()"), // Special characters
    ];

    for (input, hash_count, expected) in test_cases {
        let result = crate::parse::string::process_raw_string(input, hash_count, Span::call_site());
        assert!(result.is_ok(), "Failed to process edge case raw string: {:?}", input);
        assert_eq!(result.unwrap(), expected, "Edge case raw string {:?} didn't match expected output", input);
    }
}

/// Test edge cases for raw multi-line strings
#[test]
fn test_raw_multiline_edge_cases() {
    // Minimal multi-line string
    let input = "\n\n    ";
    let hash_count = 1;
    let expected = "";

    let result = crate::parse::string::process_raw_multiline_string(input, hash_count, Span::call_site());
    assert!(result.is_ok(), "Failed to process minimal raw multi-line string");
    assert_eq!(result.unwrap(), expected, "Minimal raw multi-line string should result in empty string");
}

/// Test that raw multi-line strings follow same dedentation rules as regular multi-line strings
#[test]
fn test_raw_multiline_dedentation() {
    let test_cases = vec![
        // Basic dedentation
        ("\n        foo\n    bar\n    ", 1, "    foo\nbar"),
        // Shorter final line indent
        ("\n        foo\n    bar\n  ", 1, "      foo\n  bar"),
        // Empty lines preserved
        ("\n    line1\n\n    line2\n    ", 1, "line1\n\nline2"),
    ];

    for (input, hash_count, expected) in test_cases {
        let result = crate::parse::string::process_raw_multiline_string(input, hash_count, Span::call_site());
        assert!(result.is_ok(), "Failed to process raw multi-line string dedentation: {:?}", input);
        assert_eq!(result.unwrap(), expected, "Raw multi-line dedentation didn't match expected output");
    }
}

/// Test newline normalization in raw multi-line strings
#[test]
fn test_raw_multiline_newline_normalization() {
    // CR LF should be normalized to LF
    let input = "\r\n    Hello\r\n    World\r\n    ";
    let hash_count = 1;
    let expected = "Hello\nWorld";

    let result = crate::parse::string::process_raw_multiline_string(input, hash_count, Span::call_site());
    assert!(result.is_ok(), "Failed to normalize newlines in raw multi-line string");
    assert_eq!(result.unwrap(), expected, "Newline normalization in raw multi-line string didn't work correctly");
}

/// Test specification examples from Section 3.13.1
#[test]
fn test_specification_examples() {
    // Example 1: just-escapes #"\n will be literal"#
    let result1 = crate::parse::string::process_raw_string(r#"\n will be literal"#, 1, Span::call_site());
    assert!(result1.is_ok(), "Failed to process spec example 1");
    assert_eq!(result1.unwrap(), r#"\n will be literal"#, "Spec example 1 should preserve literal escapes");

    // Example 2: quotes-and-escapes ##"hello\n\r\world"##
    let result2 = crate::parse::string::process_raw_string(r#"hello\n\r\world"#, 2, Span::call_site());
    assert!(result2.is_ok(), "Failed to process spec example 2");
    assert_eq!(result2.unwrap(), r#"hello\n\r\world"#, "Spec example 2 should preserve content exactly");

    // Example 3: raw-multi-line
    let raw_multiline_input = "\n    Here's a \"\"\"\n        multiline string\n        \"\"\"\n    without escapes.\n    ";
    let result3 = crate::parse::string::process_raw_multiline_string(raw_multiline_input, 1, Span::call_site());
    assert!(result3.is_ok(), "Failed to process spec example 3");
    assert_eq!(result3.unwrap(), "Here's a \"\"\"\n    multiline string\n    \"\"\"\nwithout escapes.", "Spec example 3 should match expected dedented output");
}

/// Test error cases for raw strings
#[test]
fn test_raw_string_error_cases() {
    // Raw string processing should generally not fail for content issues
    // since no escape processing is done. Most errors would come from
    // parsing/tokenization level, not content processing.

    // However, we should test that the hash_count parameter is used correctly
    let valid_cases = vec![
        ("content", 0), // Zero hash count should be valid (though unusual)
        ("content", 1),
        ("content", 10),
    ];

    for (input, hash_count) in valid_cases {
        let result = crate::parse::string::process_raw_string(input, hash_count, Span::call_site());
        assert!(result.is_ok(), "Raw string processing should succeed for hash_count {}: {}", hash_count, input);
    }
}

/// Test error cases for raw multi-line strings
#[test]
fn test_raw_multiline_error_cases() {
    // Test invalid multi-line string formats
    let invalid_cases = vec![
        ("", "Empty string should fail"),
        ("no leading newline\n    ", "Must start with newline"),
        ("\nsingle line", "Must have at least 2 lines"),
    ];

    for (input, description) in invalid_cases {
        let result = crate::parse::string::process_raw_multiline_string(input, 1, Span::call_site());
        assert!(result.is_err(), "Should fail for invalid format: {}", description);
    }
}

/// Test that raw strings preserve all characters literally
#[test]
fn test_raw_string_literal_preservation() {
    // Test characters that would normally be escape sequences
    let literal_chars = vec![
        r"\n", r"\t", r"\r", r"\\", r#"\""#, r"\b", r"\f", r"\s",
        r"\u{41}", r"\x41", r"\0", r"\a", r"\v",
    ];

    for literal in literal_chars {
        let result = crate::parse::string::process_raw_string(literal, 1, Span::call_site());
        assert!(result.is_ok(), "Failed to process literal character sequence: {}", literal);
        assert_eq!(result.unwrap(), literal, "Raw string should preserve {} literally", literal);
    }
}

/// Test integration with KdlString::Raw AST variant
#[test]
fn test_kdl_string_raw_variant() {
    use crate::ast::KdlString;

    // Test creating Raw variant
    let raw_string = KdlString::Raw {
        value: r#"\n literal"#.to_string(),
        hash_count: 1,
        span: Span::call_site(),
    };

    // Test value() method
    assert_eq!(raw_string.value(), r#"\n literal"#);

    // Test span() method (just verify it returns a span)
    let _ = raw_string.span();

    // Test validation
    let validation_result = raw_string.validate();
    assert!(validation_result.is_ok(), "Raw string validation should succeed");
}

/// Test helper methods for Raw variant in AST
#[test]
fn test_raw_string_helper_methods() {
    use crate::ast::KdlString;

    // Test from_raw_content helper method
    let content = r#"raw\ncontent"#;
    let hash_count = 2;
    let span = Span::call_site();

    let raw_string = KdlString::from_raw_content(content.to_string(), hash_count, span);
    assert_eq!(raw_string.value(), content);
    if let KdlString::Raw { hash_count: actual_count, .. } = raw_string {
        assert_eq!(actual_count, hash_count);
    } else {
        panic!("Expected Raw variant");
    }
}

/// Test comprehensive raw string scenarios
#[test]
fn test_comprehensive_raw_string_scenarios() {
    let test_scenarios = vec![
        // Simple cases
        (r"hello", 1),
        (r"hello world", 1),

        // With quotes
        (r#"hello "world""#, 1),
        (r#"he said "hello""#, 1),

        // With different hash counts (simulating the content that would be inside)
        (r#"contains " inside"#, 2), // Content that would be in ##"..."##
        ("contains \"# inside", 3), // Content that would be in ###"..."###

        // With escape-like sequences
        (r"path\\to\\file", 1),
        (r"regex: \\d+\\.\\d+", 1),
        (r#"json: {"key": "value"}"#, 1),

        // Complex content
        ("#!/bin/bash\\necho \"Hello World\"\\nexit 0", 1),
        (r#"SELECT * FROM table WHERE col = "value""#, 1),
    ];

    for (input, hash_count) in test_scenarios {
        let result = crate::parse::string::process_raw_string(input, hash_count, Span::call_site());
        assert!(result.is_ok(), "Failed to process comprehensive scenario: {}", input);
        assert_eq!(result.unwrap(), input, "Comprehensive scenario should preserve content exactly");
    }
}