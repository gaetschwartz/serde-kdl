//! Tests for KDL Section 3.3 - Line Continuation
//!
//! Line continuations allow Nodes to be spread across multiple lines.
//! A line continuation is a `\` character followed by zero or more whitespace
//! items and an optional single-line comment, terminated by a newline.

use crate::assert_eq_tk;
use crate::tests::specs::kdl_impl2;
use quote::quote;

#[test]
fn test_process_line_continuation_string_basic() {
    let input = "my-node 1 2 \\\n        3 4";
    let result = crate::utils::process_line_continuation_string(input).unwrap();
    assert_eq!(result, "my-node 1 2 3 4");
}

#[test]
fn test_process_line_continuation_string_with_comment() {
    let input = "my-node 1 2 \\  // comments are ok after \\\n        3 4";
    let result = crate::utils::process_line_continuation_string(input).unwrap();
    assert_eq!(result, "my-node 1 2 3 4");
}

#[test]
fn test_process_line_continuation_string_with_whitespace() {
    let input = "my-node 1 2 \\   \t  \n        3 4";
    let result = crate::utils::process_line_continuation_string(input).unwrap();
    assert_eq!(result, "my-node 1 2 3 4");
}

#[test]
fn test_process_line_continuation_string_multiple() {
    let input = "node 1 \\\n    2 \\\n    3 4";
    let result = crate::utils::process_line_continuation_string(input).unwrap();
    assert_eq!(result, "node 1 2 3 4");
}

#[test]
fn test_process_line_continuation_string_not_continuation() {
    // Backslash not followed by newline should be preserved
    let input = "node \"path\\to\\file\" 42";
    let result = crate::utils::process_line_continuation_string(input).unwrap();
    assert_eq!(result, "node \"path\\to\\file\" 42");
}

#[test]
fn test_process_line_continuation_string_carriage_return() {
    // Test with \r\n line endings
    let input = "my-node 1 2 \\\r\n        3 4";
    let result = crate::utils::process_line_continuation_string(input).unwrap();
    assert_eq!(result, "my-node 1 2 3 4");
}

#[test]
fn test_process_line_continuation_string_carriage_return_only() {
    // Test with \r line endings
    let input = "my-node 1 2 \\\r        3 4";
    let result = crate::utils::process_line_continuation_string(input).unwrap();
    assert_eq!(result, "my-node 1 2 3 4");
}

#[test]
fn test_process_line_continuation_string_with_comment_after_whitespace() {
    let input = "node value1 \\   // some comment\n    value2";
    let result = crate::utils::process_line_continuation_string(input).unwrap();
    assert_eq!(result, "node value1 value2");
}

#[test]
fn test_process_line_continuation_string_preserves_unrelated_backslashes() {
    // Test that backslashes in contexts other than line continuation are preserved
    let input = "node \"C:\\\\path\\\\to\\\\file\" normal-arg";
    let result = crate::utils::process_line_continuation_string(input).unwrap();
    assert_eq!(result, "node \"C:\\\\path\\\\to\\\\file\" normal-arg");
}

#[test]
fn test_process_line_continuation_string_incomplete_comment() {
    // Test backslash followed by single slash (not a comment) should not be treated as line continuation
    let input = "node value \\/not-a-comment";
    let result = crate::utils::process_line_continuation_string(input).unwrap();
    assert_eq!(result, "node value \\/not-a-comment");
}

#[test]
fn test_process_line_continuation_string_end_of_input() {
    // Test backslash at end of input (not a line continuation)
    let input = "node value \\";
    let result = crate::utils::process_line_continuation_string(input).unwrap();
    assert_eq!(result, "node value \\");
}

#[test]
fn test_process_line_continuation_string_spec_example() {
    // Test the exact example from the KDL specification
    let input = "my-node 1 2 \\  // comments are ok after \\\n                3 4    // This is the actual end of the Node.";
    let result = crate::utils::process_line_continuation_string(input).unwrap();
    assert_eq!(
        result,
        "my-node 1 2 3 4    // This is the actual end of the Node."
    );
}

// Integration tests using valid Rust token streams

#[test]
fn test_backslash_in_strings_not_affected() {
    // Test that backslashes in strings are not treated as line continuations
    let input = quote! {
        node "path\\to\\file"
    };

    let output = kdl_impl2(input).unwrap();

    // Should remain unchanged
    let expected_input = quote! {
        node "path\\to\\file"
    };
    let expected_output = kdl_impl2(expected_input).unwrap();

    assert_eq_tk!(output, expected_output);
}

#[test]
fn test_simple_valid_kdl_without_line_continuations() {
    // Test that normal KDL still works correctly
    let input = quote! {
        node "arg1" key="value" "arg2"
    };

    let output = kdl_impl2(input).unwrap();

    // Should remain unchanged
    let expected_input = quote! {
        node "arg1" key="value" "arg2"
    };
    let expected_output = kdl_impl2(expected_input).unwrap();

    assert_eq_tk!(output, expected_output);
}

#[test]
fn test_nested_nodes_without_line_continuations() {
    // Test that nested nodes still work correctly
    let input = quote! {
        parent {
            child "value"
        }
    };

    let output = kdl_impl2(input).unwrap();

    // Should remain unchanged
    let expected_input = quote! {
        parent {
            child "value"
        }
    };
    let expected_output = kdl_impl2(expected_input).unwrap();

    assert_eq_tk!(output, expected_output);
}
