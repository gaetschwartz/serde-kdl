//! Tests for Section 3.12: Multi-line String
//!
//! This module tests the implementation of multi-line string parsing according to the KDL specification.

use proc_macro2::Span;

/// Test basic multi-line string processing with simple indentation
#[test]
fn test_basic_multiline_string() {
    // Example from specification Section 3.12.2.1
    let input = "\n        foo\n    This is the base indentation\n            bar\n    ";
    let expected = "    foo\nThis is the base indentation\n        bar";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_ok(), "Failed to process basic multi-line string");
    assert_eq!(result.unwrap(), expected, "Basic multi-line string didn't match expected output");
}

/// Test shorter last-line indent from specification Section 3.12.2.2
#[test]
fn test_shorter_last_line_indent() {
    let input = "\n        foo\n    This is no longer on the left edge\n            bar\n  ";
    let expected = "      foo\n  This is no longer on the left edge\n          bar";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_ok(), "Failed to process multi-line string with shorter indent");
    assert_eq!(result.unwrap(), expected, "Multi-line string with shorter indent didn't match expected output");
}

/// Test empty lines from specification Section 3.12.2.3
#[test]
fn test_empty_lines() {
    let input = "\n    Indented a bit\n\n    A second indented paragraph.\n    ";
    let expected = "Indented a bit\n\nA second indented paragraph.";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_ok(), "Failed to process multi-line string with empty lines");
    assert_eq!(result.unwrap(), expected, "Multi-line string with empty lines didn't match expected output");
}

/// Test that empty lines with various whitespace are preserved as empty
#[test]
fn test_empty_lines_with_whitespace() {
    let input = "\n    Line 1\n    \n        \n\t\t\n    Line 2\n    ";
    let expected = "Line 1\n\n\n\nLine 2";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_ok(), "Failed to process multi-line string with whitespace-only lines");
    assert_eq!(result.unwrap(), expected, "Multi-line string with whitespace lines didn't match expected output");
}

/// Test newline normalization (CR LF -> LF) from Section 3.12.1
#[test]
fn test_newline_normalization() {
    // Test that CR LF becomes LF
    let input = "\r\n    Hello\r\n    World\r\n    ";
    let expected = "Hello\nWorld";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_ok(), "Failed to process multi-line string with CR LF");
    assert_eq!(result.unwrap(), expected, "Newline normalization didn't work correctly");
}

/// Test that individual CR and LF are preserved (only CR LF sequences are normalized)
#[test]
fn test_individual_cr_lf_preservation() {
    // Test with escape sequences for CR and LF within content
    let input = "\n    Line with \\r here\n    Line with \\n there\n    ";
    let expected = "Line with \r here\nLine with \n there";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_ok(), "Failed to preserve individual CR/LF characters");
    assert_eq!(result.unwrap(), expected, "Individual CR/LF weren't preserved correctly");
}

/// Test whitespace escape interaction from Section 3.12.3
#[test]
fn test_whitespace_escape_interaction() {
    // Valid example from spec: whitespace escapes are processed before dedentation
    let input = "\n  foo \\    bar\n  baz\n  \\   ";
    let expected = "foo bar\nbaz";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_ok(), "Failed to process multi-line string with whitespace escapes");
    assert_eq!(result.unwrap(), expected, "Whitespace escape interaction didn't work correctly");
}

/// Test Unicode escapes in multi-line strings
#[test]
fn test_unicode_escapes_in_multiline() {
    let input = "\n    Hello \\u{1F600}\n    World \\u{41}\n    ";
    let expected = "Hello 😀\nWorld A";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_ok(), "Failed to process Unicode escapes in multi-line string");
    assert_eq!(result.unwrap(), expected, "Unicode escapes in multi-line string didn't work correctly");
}

/// Test standard escape sequences in multi-line strings
#[test]
fn test_standard_escapes_in_multiline() {
    let input = "\n    Line 1\\nLine 2\n    Tab:\\tHere\n    Quote: \\\"Hello\\\"\n    ";
    let expected = "Line 1\nLine 2\nTab:\tHere\nQuote: \"Hello\"";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_ok(), "Failed to process standard escapes in multi-line string");
    assert_eq!(result.unwrap(), expected, "Standard escapes in multi-line string didn't work correctly");
}

/// Test minimum valid multi-line string (empty content)
#[test]
fn test_empty_multiline_string() {
    let input = "\n    ";
    let expected = "";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_ok(), "Failed to process empty multi-line string");
    assert_eq!(result.unwrap(), expected, "Empty multi-line string didn't match expected output");
}

/// Test multi-line string with no indentation
#[test]
fn test_no_indentation() {
    // The final empty line represents the closing line with no indentation prefix
    // Input has structure: \nHello\nWorld\n(empty final line)
    let input = "\nHello\nWorld\n";
    let expected = "Hello\nWorld";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_ok(), "Failed to process multi-line string with no indentation");
    assert_eq!(result.unwrap(), expected, "No indentation multi-line string didn't match expected output");
}

/// Test complex indentation with mixed spaces and tabs
#[test]
fn test_mixed_whitespace_indentation() {
    // Using tabs as the dedentation prefix
    let input = "\n\t\tLine 1\n\t\tLine 2\n\t\t\tIndented more\n\t\t";
    let expected = "Line 1\nLine 2\n\tIndented more";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_ok(), "Failed to process multi-line string with tab indentation");
    assert_eq!(result.unwrap(), expected, "Tab indentation multi-line string didn't match expected output");
}

// Error cases - these should all fail

/// Test error: empty string (no content at all)
#[test]
fn test_error_completely_empty() {
    let input = "";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_err(), "Empty string should be an error");
}

/// Test error: single line (no opening newline)
#[test]
fn test_error_no_opening_newline() {
    let input = "Hello World    ";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_err(), "String without opening newline should be an error");
}

/// Test error: first line is not empty
#[test]
fn test_error_first_line_not_empty() {
    let input = "stuff\n    content\n    ";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_err(), "String with non-empty first line should be an error");
}

/// Test error: final line contains non-whitespace
#[test]
fn test_error_final_line_non_whitespace() {
    let input = "\n    Hello\n    World\n    content";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_err(), "String with non-whitespace in final line should be an error");
}

/// Test error: line doesn't match dedentation prefix
#[test]
fn test_error_inconsistent_dedentation() {
    let input = "\n    Line 1\n  Line 2\n    ";  // Line 2 has less indentation than final line

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_err(), "String with inconsistent dedentation should be an error");
}

/// Test error: mixed whitespace types in dedentation
#[test]
fn test_error_mixed_whitespace_types() {
    // Tabs vs spaces mismatch
    let input = "\n    Line with spaces\n\tLine with tab\n    ";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_err(), "Mixed whitespace types should be an error");
}

/// Test error: only one line (missing closing line)
#[test]
fn test_error_only_one_line() {
    // This input has no closing line (no final line after content)
    // It would represent """ followed by content but no closing """
    let input = "\nsome content";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_err(), "Multi-line string with only one line should be an error");
}

/// Test complex example from the spec with escape sequences
#[test]
fn test_spec_complex_example() {
    // From the spec: whitespace escapes then dedentation
    let input = "\n  foo\n  bar\\  \n  more content\n  baz\n  ";
    let expected = "foo\nbar\nmore content\nbaz";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_ok(), "Failed to process complex spec example");
    assert_eq!(result.unwrap(), expected, "Complex spec example didn't match expected output");
}

/// Test newline normalization doesn't affect escape sequences
#[test]
fn test_newline_normalization_vs_escapes() {
    // From spec Section 3.12.1: literal CRLF becomes LF, but \r\n escape sequences remain as control chars
    let input = "\n    \\r\\n[CRLF]\r\n    foo\r\n    ";
    let expected = "\r\n[CRLF]\nfoo";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_ok(), "Failed to process newline normalization with escapes");
    assert_eq!(result.unwrap(), expected, "Newline normalization vs escapes didn't work correctly");
}

/// Test that processing order is correct: normalization -> escapes -> dedentation
#[test]
fn test_processing_order() {
    // Test a complex case that exercises all processing steps
    let input = "\r\n    Hello\\    World\r\n    Line 2\r\n    ";
    let expected = "HelloWorld\nLine 2";

    let result = crate::parse::string::process_multiline_string(input, Span::call_site());
    assert!(result.is_ok(), "Failed to process complex example testing processing order");
    assert_eq!(result.unwrap(), expected, "Processing order test didn't work correctly");
}

/// Integration test: test that KdlString::from_multiline_content works
#[test]
fn test_kdlstring_integration() {
    let input = "\n    Hello\n    World\n    ".to_string();
    let expected = "Hello\nWorld";

    let result = crate::ast::KdlString::from_multiline_content(input, Span::call_site());
    assert!(result.is_ok(), "KdlString::from_multiline_content failed");

    let kdl_string = result.unwrap();
    assert_eq!(kdl_string.value(), expected, "KdlString integration didn't work correctly");

    // Verify it's the correct variant
    match kdl_string {
        crate::ast::KdlString::MultiLine { .. } => {},
        _ => panic!("Expected MultiLine variant"),
    }
}

/// Test error messages are helpful
#[test]
fn test_error_messages() {
    let test_cases = vec![
        ("", "empty"),
        ("no newline", "newline"),
        ("stuff\n    ", "newline"),  // Error: doesn't start with newline
        ("\n    content", "whitespace"),
        ("\n    line1\n  line2\n    ", "prefix"),
    ];

    for (input, expected_msg_part) in test_cases {
        let result = crate::parse::string::process_multiline_string(input, Span::call_site());
        assert!(result.is_err(), "Expected error for: {:?}", input);
        let error_msg = result.unwrap_err().to_string().to_lowercase();
        assert!(error_msg.contains(&expected_msg_part.to_lowercase()),
                "Error message '{}' should contain '{}'", error_msg, expected_msg_part);
    }
}