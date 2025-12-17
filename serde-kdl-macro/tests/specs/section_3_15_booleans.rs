//! Tests for KDL Section 3.15: Boolean
//!
//! Boolean values in KDL are represented as `#true` and `#false`.
//! Note: Bare `true` and `false` (without `#`) are parsed as identifier strings, not booleans.

use insta::assert_snapshot;
use kdl::KdlDocument;
use serde_kdl_macro::kdl;

fn doc_to_string(doc: KdlDocument) -> String {
    doc.to_string()
}

#[test]
fn test_boolean_literals_and_identifiers() {
    // Test valid boolean syntax with # prefix
    let doc = kdl! {
        node #true #false
    };
    assert_snapshot!(doc_to_string(doc), @"node #true #false");

    // Test that Rust boolean literals (true/false) are converted to KDL booleans (#true/#false)
    let doc = kdl! {
        node true false
    };
    assert_snapshot!(doc_to_string(doc), @"node #true #false");

    // Test mixed: booleans and identifier strings can coexist
    let doc = kdl! {
        node #true true #false false
    };
    assert_snapshot!(doc_to_string(doc), @"node #true #true #false #false");
}

#[test]
fn test_booleans_in_various_contexts() {
    // Test booleans as arguments and properties
    let doc = kdl! {
        server #true "localhost" 8080 enabled=#false debug=#true
    };
    assert_snapshot!(doc_to_string(doc), @"server #true localhost 8080 enabled=#false debug=#true");

    // Test booleans with type annotations
    let doc = kdl! {
        config (bool)#true (boolean)#false value=(Flag)#true
    };
    assert_snapshot!(doc_to_string(doc), @"config (bool)#true (boolean)#false value=(Flag)#true");

    // Test booleans in nested nodes
    let doc = kdl! {
        parent enabled=#true {
            child #false active=#true
            sibling readonly=#false
        }
    };
    assert_snapshot!(doc_to_string(doc), @r"
    parent enabled=#true{
    child #false active=#true
    sibling readonly=#false
    }
    ");
}

#[test]
fn test_booleans_with_mixed_types() {
    // Test comprehensive integration: booleans alongside all other KDL value types
    let doc = kdl! {
        config {
            // Integers, floats, booleans, strings, and special values
            values 42 3.14 #true #false "text" #null #inf #-inf

            // Properties with different types including booleans
            count=100 rate=0.5 enabled=#true name="test" debug=#false

            // Type-annotated values
            flags (i32)0xFF (f64)1.5e10 (bool)#true (str)"value"
        }
    };
    assert_snapshot!(doc_to_string(doc), @r"
    config{
    values 42 3.14 #true #false text #null #inf #-inf count=100 rate=0.5 enabled=#true name=test debug=#false
    flags (i32)255 (f64)15000000000.0 (bool)#true (str)value
    }
    ");
}
