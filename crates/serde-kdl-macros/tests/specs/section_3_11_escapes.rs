//! Tests for Section 3.11: Quoted String Escapes
//!
//! This module tests the implementation of escape sequences in quoted strings according to the KDL specification.
//! Section 3.11 defines standard escape sequences and Unicode escape sequences.

use super::doc_to_string;
use insta::assert_snapshot;
use serde_kdl_macros::kdl;

/// Test standard escape sequences: \n, \r, \t, \\, \"
/// Covers Section 3.11.1 - Standard escape sequences
#[test]
fn test_standard_escape_sequences() {
    let doc = kdl! {
        newline "Hello\nWorld"
        tab "Hello\tWorld"
        carriage_return "Hello\rWorld"
        backslash "path\\to\\file"
        quote "He said \"Hello\""
        combined "Line1\nLine2\tTabbed\rReturn\\Backslash\"Quote"
    };

    assert_snapshot!(doc_to_string(doc), @r#"
    newline "Hello\nWorld"
    tab "Hello\tWorld"
    carriage_return "Hello\rWorld"
    backslash "path\\to\\file"
    quote "He said \"Hello\""
    combined "Line1\nLine2\tTabbed\rReturn\\Backslash\"Quote"
    "#);
}

/// Test Unicode escape sequences: \u{XXXXXX}
/// Covers Section 3.11.2 - Unicode escapes with various code points
#[test]
fn test_unicode_escape_sequences() {
    let doc = kdl! {
        ascii "\u{41}\u{42}\u{43}"
        emoji "\u{1F600}\u{1F4DD}\u{2713}"
        chinese "\u{4E2D}\u{6587}"
        null_char "\u{0}"
        max_codepoint "\u{10FFFF}"
        mixed "Hello \u{1F30D} World"
    };

    assert_snapshot!(doc_to_string(doc), @"ascii ABC\nemoji 😀📝✓\nchinese 中文\nnull_char \"\u{0}\"\nmax_codepoint \u{10ffff}\nmixed \"Hello 🌍 World\"");
}

/// Test escape sequences in different contexts: node names, property keys, property values, arguments
/// Verifies escapes work correctly across all KDL syntactic positions
#[test]
fn test_escapes_in_all_contexts() {
    let doc = kdl! {
        "node\nname" "arg\ttab" "quote\"here" key="value\nhere" prop="\u{1F600}"
        escaped_node {
            "child\u{41}" "nested\narg" "property\tkey"="backslash\u{005C}test"
        }
    };

    assert_snapshot!(doc_to_string(doc), @r#"
    "node\nname" "arg\ttab" "quote\"here" key="value\nhere" prop=😀
    escaped_node {
        childA "nested\narg" "property\tkey"="backslash\\test"
    }
    "#);
}
