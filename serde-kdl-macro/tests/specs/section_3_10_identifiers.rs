//! Tests for Section 3.10: Identifier Strings
//!
//! This module tests identifier string validation according to the KDL specification.
//! Section 3.10 defines valid identifier characters, initial character restrictions,
//! and disallowed patterns (numbers, keywords).

use super::doc_to_string;
use insta::assert_snapshot;
use serde_kdl_macro::kdl;

/// Test valid identifier patterns in various contexts
#[test]
fn test_valid_identifiers() {
    // Basic bare identifiers
    let doc = kdl! {
        node "value"
        foo_bar 42
        camelCase #true
        _underscore "start"
    };
    assert_snapshot!(doc_to_string(doc), @r"
    node value
    foo_bar 42
    camelCase #true
    _underscore start
    ");

    // Identifiers with special characters that require quoting
    let doc = kdl! {
        "dash-separated" "arg"
        "with.dots" x=10
        "plus+sign" enabled=#true
    };
    assert_snapshot!(doc_to_string(doc), @"dash-separated arg with.dots plus+sign x=10 enabled=#true");

    // Unicode identifiers
    let doc = kdl! {
        "café" location="Paris"
        "東京" country="Japan"
        "москва" country="Russia"
    };
    assert_snapshot!(doc_to_string(doc), @"café 東京 москва location=Paris country=Japan country=Russia");
}

/// Test that quoted strings bypass identifier restrictions
#[test]
fn test_quoted_strings_bypass_restrictions() {
    // Quoted strings that would be invalid as bare identifiers
    let doc = kdl! {
        "123abc" "starts-with-digit"
        "+123" "plus-digit"
        "-456" "minus-digit"
        ".789" "dot-digit"
        "true" "keyword"
        "null" "keyword"
        "(parens)" "punctuation"
    };
    assert_snapshot!(doc_to_string(doc), @r#""123abc" starts-with-digit "+123" plus-digit "-456" minus-digit ".789" dot-digit "true" keyword "null" keyword "(parens)" punctuation"#);
}

/// Test identifiers in complex document structures
#[test]
fn test_identifiers_in_complex_structures() {
    let doc = kdl! {
        config environment="production" {
            database host="localhost" port=5432
            "cache-settings" {
                enabled #true
                "max-size" 1000
            }
        }

        "my-app" version="1.0.0" {
            features "api" "web" "cli"
            _internal debug=#false
        }
    };
    assert_snapshot!(doc_to_string(doc), @r#"
    config environment=production {
        database cache-settings host=localhost port=5432 {
            enabled #true max-size 1000
        }
    }
    my-app version="1.0.0" {
        features api web cli
        _internal debug=#false
    }
    "#);
}
