//! Tests for Section 3.15: Boolean
//!
//! This module contains comprehensive tests for the KDL Boolean specification
//! as defined in section 3.15 of the KDL specification.
//!
//! The tests cover:
//! - Boolean keywords: #true and #false
//! - Boolean value parsing and validation
//! - Invalid boolean syntax detection
//! - Boolean usage in various contexts (arguments, properties, etc.)
//! - Edge cases and boundary conditions for boolean parsing
//! - Error handling for malformed boolean literals

use crate::specs::kdl_impl2;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rstest::rstest;
use seq_macro::seq;
use serde_kdl_macro::kdl;

// ============================================================================
// Section 3.15.1: Valid Boolean Tests
// ============================================================================

/// Test basic #true boolean keyword
#[test]
fn test_true_boolean_keyword() {
    let doc = kdl! {
        node #true
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "node");
    assert_eq!(node.entries().len(), 1);
    assert_eq!(node.entries()[0].value().as_bool().unwrap(), true);
}

/// Test basic #false boolean keyword
#[test]
fn test_false_boolean_keyword() {
    let doc = kdl! {
        node #false
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "node");
    assert_eq!(node.entries().len(), 1);
    assert_eq!(node.entries()[0].value().as_bool().unwrap(), false);
}

/// Test the example from section 3.15.1
#[test]
fn test_section_3_15_1_example() {
    let doc = kdl! {
        my-node #true value=#false
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "my-node");
    assert_eq!(node.entries().len(), 2);

    // First entry should be #true argument
    assert!(node.entries()[0].name().is_none(), "First entry should be an argument, not a property");
    assert_eq!(node.entries()[0].value().as_bool().unwrap(), true);

    // Second entry should be value=#false property
    assert!(node.entries()[1].name().is_some(), "Second entry should be a property");
    assert_eq!(node.entries()[1].name().unwrap().value(), "value");
    assert_eq!(node.entries()[1].value().as_bool().unwrap(), false);
}

/// Test boolean values in different argument positions
#[test]
fn test_boolean_arguments_positions() {
    let doc = kdl! {
        node1 #true
        node2 "string" #false
        node3 #true "string" #false
        node4 42 #true "string" #false 3.14
    };
    assert_eq!(doc.nodes().len(), 4);

    // Test node1: single boolean argument
    let node1 = &doc.nodes()[0];
    assert_eq!(node1.entries().len(), 1);
    assert_eq!(node1.entries()[0].value().as_bool().unwrap(), true);

    // Test node2: string then boolean
    let node2 = &doc.nodes()[1];
    assert_eq!(node2.entries().len(), 2);
    assert_eq!(node2.entries()[0].value().as_string().unwrap(), "string");
    assert_eq!(node2.entries()[1].value().as_bool().unwrap(), false);

    // Test node3: boolean, string, boolean
    let node3 = &doc.nodes()[2];
    assert_eq!(node3.entries().len(), 3);
    assert_eq!(node3.entries()[0].value().as_bool().unwrap(), true);
    assert_eq!(node3.entries()[1].value().as_string().unwrap(), "string");
    assert_eq!(node3.entries()[2].value().as_bool().unwrap(), false);

    // Test node4: mixed types with booleans
    let node4 = &doc.nodes()[3];
    assert_eq!(node4.entries().len(), 5);
    assert_eq!(node4.entries()[0].value().as_i64().unwrap(), 42);
    assert_eq!(node4.entries()[1].value().as_bool().unwrap(), true);
    assert_eq!(node4.entries()[2].value().as_string().unwrap(), "string");
    assert_eq!(node4.entries()[3].value().as_bool().unwrap(), false);
    assert_eq!(node4.entries()[4].value().as_f64().unwrap(), 3.14);
}

/// Test boolean values in properties
#[test]
fn test_boolean_properties() {
    let doc = kdl! {
        node enabled=#true disabled=#false debug=#true
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    // Test enabled=#true
    let enabled = &node.entries()[0];
    assert_eq!(enabled.name().unwrap().value(), "enabled");
    assert_eq!(enabled.value().as_bool().unwrap(), true);

    // Test disabled=#false
    let disabled = &node.entries()[1];
    assert_eq!(disabled.name().unwrap().value(), "disabled");
    assert_eq!(disabled.value().as_bool().unwrap(), false);

    // Test debug=#true
    let debug = &node.entries()[2];
    assert_eq!(debug.name().unwrap().value(), "debug");
    assert_eq!(debug.value().as_bool().unwrap(), true);
}

/// Test mixed arguments and properties with booleans
#[test]
fn test_mixed_boolean_usage() {
    let doc = kdl! {
        node #true "string" 42 enabled=#false debug=#true
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 5);

    // Arguments
    assert_eq!(node.entries()[0].value().as_bool().unwrap(), true);
    assert_eq!(node.entries()[1].value().as_string().unwrap(), "string");
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 42);

    // Properties
    assert_eq!(node.entries()[3].name().unwrap().value(), "enabled");
    assert_eq!(node.entries()[3].value().as_bool().unwrap(), false);
    assert_eq!(node.entries()[4].name().unwrap().value(), "debug");
    assert_eq!(node.entries()[4].value().as_bool().unwrap(), true);
}

/// Test booleans in child nodes
#[test]
fn test_boolean_in_children() {
    let doc = kdl! {
        parent enabled=#true {
            child1 #false
            child2 active=#true inactive=#false
            child3 #true #false
        }
    };
    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");
    assert_eq!(parent.entries().len(), 1);
    assert_eq!(parent.entries()[0].value().as_bool().unwrap(), true);

    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 3);

    // Test child1
    let child1 = &children.nodes()[0];
    assert_eq!(child1.name().value(), "child1");
    assert_eq!(child1.entries().len(), 1);
    assert_eq!(child1.entries()[0].value().as_bool().unwrap(), false);

    // Test child2
    let child2 = &children.nodes()[1];
    assert_eq!(child2.name().value(), "child2");
    assert_eq!(child2.entries().len(), 2);
    assert_eq!(child2.entries()[0].name().unwrap().value(), "active");
    assert_eq!(child2.entries()[0].value().as_bool().unwrap(), true);
    assert_eq!(child2.entries()[1].name().unwrap().value(), "inactive");
    assert_eq!(child2.entries()[1].value().as_bool().unwrap(), false);

    // Test child3
    let child3 = &children.nodes()[2];
    assert_eq!(child3.name().value(), "child3");
    assert_eq!(child3.entries().len(), 2);
    assert_eq!(child3.entries()[0].value().as_bool().unwrap(), true);
    assert_eq!(child3.entries()[1].value().as_bool().unwrap(), false);
}

// ============================================================================
// Section 3.15.2: Boolean Value Type Tests
// ============================================================================

/// Test boolean value type checking
#[test]
fn test_boolean_value_type() {
    let doc = kdl! {
        node #true #false
    };
    let node = &doc.nodes()[0];

    // Verify values are correctly identified as booleans
    assert!(node.entries()[0].value().as_bool().is_some());
    assert!(node.entries()[1].value().as_bool().is_some());

    // Verify they're not other types
    assert!(node.entries()[0].value().as_string().is_none());
    assert!(node.entries()[0].value().as_i64().is_none());
    assert!(node.entries()[0].value().as_f64().is_none());

    assert!(node.entries()[1].value().as_string().is_none());
    assert!(node.entries()[1].value().as_i64().is_none());
    assert!(node.entries()[1].value().as_f64().is_none());
}

/// Test boolean representation consistency
#[test]
fn test_boolean_representation() {
    let doc = kdl! {
        config production=#true development=#false testing=#true
    };
    let node = &doc.nodes()[0];

    // Test that boolean values maintain their logical meaning
    let production = node.entries()[0].value().as_bool().unwrap();
    let development = node.entries()[1].value().as_bool().unwrap();
    let testing = node.entries()[2].value().as_bool().unwrap();

    assert_eq!(production, true);
    assert_eq!(development, false);
    assert_eq!(testing, true);

    // Test logical operations work as expected
    assert!(production && testing);
    assert!(!(production && development));
    assert!(production || development);
    assert!(!development);
}

/// Test boolean values with various data patterns
#[rstest]
#[case::true_only(kdl! { node #true }, vec![true])]
#[case::false_only(kdl! { node #false }, vec![false])]
#[case::true_false(kdl! { node #true #false }, vec![true, false])]
#[case::false_true(kdl! { node #false #true }, vec![false, true])]
#[case::multiple_true(kdl! { node #true #true #true }, vec![true, true, true])]
#[case::multiple_false(kdl! { node #false #false #false }, vec![false, false, false])]
#[case::alternating(kdl! { node #true #false #true #false }, vec![true, false, true, false])]
fn test_boolean_patterns(#[case] doc: kdl::KdlDocument, #[case] expected: Vec<bool>) {
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), expected.len());

    for (i, expected_value) in expected.iter().enumerate() {
        assert_eq!(node.entries()[i].value().as_bool().unwrap(), *expected_value);
    }
}

// ============================================================================
// Section 3.15.3: Boolean Property Name Tests
// ============================================================================

/// Test boolean properties with various property names
#[test]
fn test_boolean_property_names() {
    let doc = kdl! {
        node
            enabled=#true
            disabled=#false
            active=#true
            inactive=#false
            visible=#true
            hidden=#false
    };
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 6);

    let expected_props = [
        ("enabled", true),
        ("disabled", false),
        ("active", true),
        ("inactive", false),
        ("visible", true),
        ("hidden", false),
    ];

    for (i, (name, value)) in expected_props.iter().enumerate() {
        let entry = &node.entries()[i];
        assert_eq!(entry.name().unwrap().value(), *name);
        assert_eq!(entry.value().as_bool().unwrap(), *value);
    }
}

/// Test boolean properties with complex names
#[test]
fn test_complex_boolean_property_names() {
    let doc = kdl! {
        node
            "complex-name"=#true
            "with_underscore"=#false
            "with.dot"=#true
            "123numeric"=#false
    };
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 4);

    let expected_props = [
        ("complex-name", true),
        ("with_underscore", false),
        ("with.dot", true),
        ("123numeric", false),
    ];

    for (i, (name, value)) in expected_props.iter().enumerate() {
        let entry = &node.entries()[i];
        assert_eq!(entry.name().unwrap().value(), *name);
        assert_eq!(entry.value().as_bool().unwrap(), *value);
    }
}

// ============================================================================
// Section 3.15.4: Boolean Context Tests
// ============================================================================

/// Test booleans in configuration-like contexts
#[test]
fn test_boolean_configuration_context() {
    let doc = kdl! {
        server {
            ssl #true
            compression enabled=#true gzip=#false
            logging debug=#false info=#true
            cache enabled=#true ttl=3600
        }
        client {
            retry #true
            timeout 30
            keepalive #false
        }
    };
    assert_eq!(doc.nodes().len(), 2);

    // Test server configuration
    let server = &doc.nodes()[0];
    let server_children = server.children().unwrap();
    assert_eq!(server_children.nodes().len(), 4);

    // SSL node
    let ssl = &server_children.nodes()[0];
    assert_eq!(ssl.name().value(), "ssl");
    assert_eq!(ssl.entries()[0].value().as_bool().unwrap(), true);

    // Compression node
    let compression = &server_children.nodes()[1];
    assert_eq!(compression.name().value(), "compression");
    assert_eq!(compression.entries()[0].value().as_bool().unwrap(), true);
    assert_eq!(compression.entries()[1].value().as_bool().unwrap(), false);

    // Test client configuration
    let client = &doc.nodes()[1];
    let client_children = client.children().unwrap();
    let retry = &client_children.nodes()[0];
    assert_eq!(retry.entries()[0].value().as_bool().unwrap(), true);
    let keepalive = &client_children.nodes()[2];
    assert_eq!(keepalive.entries()[0].value().as_bool().unwrap(), false);
}

/// Test booleans in feature flag contexts
#[test]
fn test_boolean_feature_flags() {
    let doc = kdl! {
        features {
            experimental_ui #false
            new_parser #true
            beta_features enabled=#true rollout=#false
            legacy_support #true
        }
    };
    let features = &doc.nodes()[0];
    let feature_children = features.children().unwrap();
    assert_eq!(feature_children.nodes().len(), 4);

    // Check each feature flag
    assert_eq!(feature_children.nodes()[0].entries()[0].value().as_bool().unwrap(), false);
    assert_eq!(feature_children.nodes()[1].entries()[0].value().as_bool().unwrap(), true);
    assert_eq!(feature_children.nodes()[2].entries()[0].value().as_bool().unwrap(), true);
    assert_eq!(feature_children.nodes()[2].entries()[1].value().as_bool().unwrap(), false);
    assert_eq!(feature_children.nodes()[3].entries()[0].value().as_bool().unwrap(), true);
}

// ============================================================================
// Section 3.15.5: Multiple Boolean Tests
// ============================================================================

/// Test documents with many boolean values
#[test]
fn test_many_booleans() {
    let doc = kdl! {
        booleans #true #false #true #false #true #false #true #false
    };
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 8);

    for (i, entry) in node.entries().iter().enumerate() {
        let expected = i % 2 == 0; // true for even indices, false for odd
        assert_eq!(entry.value().as_bool().unwrap(), expected);
    }
}

/// Test many boolean properties
#[test]
fn test_many_boolean_properties() {
    let doc = kdl! {
        flags
            flag1=#true
            flag2=#false
            flag3=#true
            flag4=#false
            flag5=#true
            flag6=#false
            flag7=#true
            flag8=#false
    };
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 8);

    for (i, entry) in node.entries().iter().enumerate() {
        let expected_name = format!("flag{}", i + 1);
        let expected_value = i % 2 == 0; // true for even indices, false for odd

        assert_eq!(entry.name().unwrap().value(), expected_name);
        assert_eq!(entry.value().as_bool().unwrap(), expected_value);
    }
}

/// Test boolean sequences using seq-macro for alternating patterns
seq!(N in 1..=8 {
    #[test]
    fn test_boolean_sequence_~N() {
        // Create alternating boolean patterns based on N
        let doc = if N % 2 == 1 {
            kdl! { node #true }
        } else {
            kdl! { node #false }
        };
        let node = &doc.nodes()[0];
        assert_eq!(node.entries().len(), 1);
        let expected = N % 2 == 1;
        assert_eq!(node.entries()[0].value().as_bool().unwrap(), expected);
    }
});

// ============================================================================
// Section 3.15.6: Error Cases and Invalid Boolean Syntax
// ============================================================================

/// Test invalid boolean syntax using kdl_impl2
#[test]
fn test_invalid_boolean_syntax() {
    // Test boolean without # prefix
    let result = kdl_impl2(quote! { node true });
    assert!(result.is_err(), "Boolean without # prefix should produce an error");

    let result = kdl_impl2(quote! { node false });
    assert!(result.is_err(), "Boolean without # prefix should produce an error");
}

/// Test malformed boolean keywords
#[test]
fn test_malformed_boolean_keywords() {
    // Test incorrect case
    let result = kdl_impl2(quote! { node #True });
    assert!(result.is_err(), "Incorrect case #True should produce an error");

    let result = kdl_impl2(quote! { node #False });
    assert!(result.is_err(), "Incorrect case #False should produce an error");

    let result = kdl_impl2(quote! { node #TRUE });
    assert!(result.is_err(), "Uppercase #TRUE should produce an error");

    let result = kdl_impl2(quote! { node #FALSE });
    assert!(result.is_err(), "Uppercase #FALSE should produce an error");
}

/// Test invalid boolean-like syntax
#[test]
fn test_invalid_boolean_like_syntax() {
    // Test incomplete boolean
    let result = kdl_impl2(quote! { node # });
    assert!(result.is_err(), "Incomplete # should produce an error");

    // Test boolean with extra characters
    let result = kdl_impl2(quote! { node #trues });
    assert!(result.is_err(), "Boolean with extra characters should produce an error");

    let result = kdl_impl2(quote! { node #falses });
    assert!(result.is_err(), "Boolean with extra characters should produce an error");
}

/// Test boolean in invalid contexts
#[test]
fn test_boolean_invalid_contexts() {
    // Test boolean as node name (should fail in macro context)
    let result = kdl_impl2(quote! { #true });
    assert!(result.is_err(), "Boolean as node name should produce an error");

    let result = kdl_impl2(quote! { #false });
    assert!(result.is_err(), "Boolean as node name should produce an error");
}

/// Test mixed invalid boolean syntax
#[test]
fn test_mixed_invalid_boolean_syntax() {
    // Test mixing valid and invalid booleans
    let result = kdl_impl2(quote! { node #true false #false });
    assert!(result.is_err(), "Mixing valid and invalid booleans should produce an error");

    // Test property with invalid boolean
    let result = kdl_impl2(quote! { node key=true });
    assert!(result.is_err(), "Property with invalid boolean should produce an error");
}

// ============================================================================
// Section 3.15.7: Boolean Edge Cases
// ============================================================================

/// Test boolean with whitespace (macro handles this)
#[test]
fn test_boolean_whitespace_handling() {
    // The kdl! macro handles whitespace parsing internally
    let doc = kdl! {
        node #true #false
    };
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 2);
    assert_eq!(node.entries()[0].value().as_bool().unwrap(), true);
    assert_eq!(node.entries()[1].value().as_bool().unwrap(), false);
}

/// Test booleans with comments (conceptual test)
#[test]
fn test_boolean_conceptual_comments() {
    // Comments are handled at parse level, but we test the concept
    let doc = kdl! {
        node #true #false // This would have comments in real KDL
    };
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 2);
    assert_eq!(node.entries()[0].value().as_bool().unwrap(), true);
    assert_eq!(node.entries()[1].value().as_bool().unwrap(), false);
}

/// Test boolean boundary conditions
#[test]
fn test_boolean_boundary_conditions() {
    // Test boolean as first argument
    let doc = kdl! {
        node #true "after"
    };
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_bool().unwrap(), true);
    assert_eq!(node.entries()[1].value().as_string().unwrap(), "after");

    // Test boolean as last argument
    let doc = kdl! {
        node "before" #false
    };
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "before");
    assert_eq!(node.entries()[1].value().as_bool().unwrap(), false);
}

/// Test boolean with other value types
#[test]
fn test_boolean_with_other_types() {
    let doc = kdl! {
        node #true 42 3.14 "string" null #false
    };
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 6);

    assert_eq!(node.entries()[0].value().as_bool().unwrap(), true);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 42);
    assert_eq!(node.entries()[2].value().as_f64().unwrap(), 3.14);
    assert_eq!(node.entries()[3].value().as_string().unwrap(), "string");
    assert!(node.entries()[4].value().is_null());
    assert_eq!(node.entries()[5].value().as_bool().unwrap(), false);
}

// ============================================================================
// Section 3.15.8: Boolean Specification Conformance Tests
// ============================================================================

/// Test that booleans conform to spec requirements
#[test]
fn test_boolean_spec_conformance() {
    let doc = kdl! {
        config debug=#true verbose=#false
    };
    let node = &doc.nodes()[0];

    // Verify boolean symbols are exactly #true and #false
    assert_eq!(node.entries()[0].value().as_bool().unwrap(), true);
    assert_eq!(node.entries()[1].value().as_bool().unwrap(), false);

    // Verify they represent logical boolean values
    let debug_value = node.entries()[0].value().as_bool().unwrap();
    let verbose_value = node.entries()[1].value().as_bool().unwrap();

    // Test logical operations work correctly
    assert_eq!(debug_value && verbose_value, false);
    assert_eq!(debug_value || verbose_value, true);
    assert_eq!(!debug_value, false);
    assert_eq!(!verbose_value, true);
}

/// Test boolean logical approximation (should work like booleans)
#[test]
fn test_boolean_logical_approximation() {
    let doc = kdl! {
        conditions a=#true b=#false c=#true
    };
    let node = &doc.nodes()[0];

    let a = node.entries()[0].value().as_bool().unwrap();
    let b = node.entries()[1].value().as_bool().unwrap();
    let c = node.entries()[2].value().as_bool().unwrap();

    // Test that they behave as logical boolean values
    assert_eq!(a, true);
    assert_eq!(b, false);
    assert_eq!(c, true);

    // Test logical combinations
    assert_eq!(a && b, false);
    assert_eq!(a && c, true);
    assert_eq!(b || c, true);
    assert_eq!(a && !b, true);
    assert_eq!(!a || b, false);
}

/// Test boolean consistency across document
#[test]
fn test_boolean_consistency() {
    let doc = kdl! {
        node1 #true
        node2 #false
        node3 enabled=#true disabled=#false
        parent {
            child1 #true
            child2 #false
        }
    };

    // Collect all boolean values from the document
    let mut boolean_values = Vec::new();

    // From node1
    boolean_values.push(doc.nodes()[0].entries()[0].value().as_bool().unwrap());

    // From node2
    boolean_values.push(doc.nodes()[1].entries()[0].value().as_bool().unwrap());

    // From node3
    boolean_values.push(doc.nodes()[2].entries()[0].value().as_bool().unwrap());
    boolean_values.push(doc.nodes()[2].entries()[1].value().as_bool().unwrap());

    // From parent children
    let parent_children = doc.nodes()[3].children().unwrap();
    boolean_values.push(parent_children.nodes()[0].entries()[0].value().as_bool().unwrap());
    boolean_values.push(parent_children.nodes()[1].entries()[0].value().as_bool().unwrap());

    // Verify pattern: true, false, true, false, true, false
    let expected = [true, false, true, false, true, false];
    assert_eq!(boolean_values.len(), expected.len());

    for (i, &expected_value) in expected.iter().enumerate() {
        assert_eq!(boolean_values[i], expected_value, "Boolean value at position {} should be {}", i, expected_value);
    }
}

/// Test boolean type safety
#[test]
fn test_boolean_type_safety() {
    let doc = kdl! {
        test #true #false
    };
    let node = &doc.nodes()[0];

    for entry in node.entries() {
        let value = entry.value();

        // Should be boolean
        assert!(value.as_bool().is_some(), "Value should be convertible to boolean");

        // Should not be other types
        assert!(value.as_string().is_none(), "Boolean should not be convertible to string");
        assert!(value.as_i64().is_none(), "Boolean should not be convertible to integer");
        assert!(value.as_f64().is_none(), "Boolean should not be convertible to float");
        assert!(!value.is_null(), "Boolean should not be null");
    }
}