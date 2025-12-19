//! Tests for KDL Section 3.15: Boolean
//!
//! Boolean values in KDL are represented as `#true` and `#false`.
//! Note: Bare `true` and `false` (without `#`) are parsed as identifier strings, not booleans.

use super::doc_to_string;
use insta::assert_snapshot;
use serde_kdl_macros::kdl;

#[test]
fn test_boolean_literals_and_identifiers() {
    // Test valid boolean syntax with # prefix
    let doc = kdl! {
        node #true #false
    };
    assert_snapshot!(doc_to_string(doc), @"node #true #false");
}

#[test]
fn test_booleans_in_various_contexts() {
    // Test booleans as arguments and properties
    let doc = kdl! {
        server #true "localhost" 8080 enabled=#false debug=#true
    };
    assert_snapshot!(doc_to_string(doc), @"server enabled=#false debug=#true #true localhost 8080");

    // Test booleans with type annotations
    let doc = kdl! {
        config (bool)#true (boolean)#false value=(Flag)#true
    };
    assert_snapshot!(doc_to_string(doc), @"config value=(Flag)#true (bool)#true (boolean)#false");

    // Test booleans in nested nodes
    let doc = kdl! {
        parent enabled=#true {
            child #false active=#true
            sibling readonly=#false
        }
    };
    assert_snapshot!(doc_to_string(doc), @r"
    parent enabled=#true {
        child active=#true #false
        sibling readonly=#false
    }
    ");
}
