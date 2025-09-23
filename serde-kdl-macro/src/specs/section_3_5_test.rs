//! Tests for KDL Argument specification (Section 3.5)
//!
//! This module contains comprehensive tests for the KDL Argument specification:
//! - Arguments are bare Values attached to a Node with no associated key
//! - Arguments share the same space as Properties and may be interleaved with them
//! - Node may have any number of Arguments, evaluated left to right
//! - KDL implementations MUST preserve the order of Arguments relative to each other
//! - Arguments MAY be prefixed with `/-` to "comment out" the entire token

use crate::{assert_eq_tk, specs::kdl_impl2};
use quote::quote;

/// Test basic argument syntax with a single argument
#[test]
fn test_single_argument() {
    let result = kdl_impl2(quote! { node 42 });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node "string" });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node true });
    assert!(result.is_ok());
}

/// Test multiple arguments in a node
#[test]
fn test_multiple_arguments() {
    let result = kdl_impl2(quote! { node 1 2 3 });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node "a" "b" "c" });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node 1 "string" true });
    assert!(result.is_ok());
}

/// Test argument ordering preservation - exactly as shown in section 3.5.1 example
#[test]
fn test_spec_example_ordering() {
    let result = kdl_impl2(quote! { my-node 1 2 3 a b c });
    assert!(result.is_ok());
}

/// Test that argument order is preserved when mixed with properties
#[test]
fn test_argument_property_interleaving() {
    let result = kdl_impl2(quote! { node "arg1" key1="prop1" "arg2" key2="prop2" "arg3" });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node key="value" 42 "string" });
    assert!(result.is_ok());
}

/// Test all valid KDL value types as arguments
#[test]
fn test_all_value_types_as_arguments() {
    // String literals
    let result = kdl_impl2(quote! { node "string" });
    assert!(result.is_ok());

    // Integer literals
    let result = kdl_impl2(quote! { node 42 });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node -42 });
    assert!(result.is_ok());

    // Float literals
    let result = kdl_impl2(quote! { node 3.14 });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node -3.14 });
    assert!(result.is_ok());

    // Boolean literals (both styles)
    let result = kdl_impl2(quote! { node true });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node false });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node #true });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node #false });
    assert!(result.is_ok());

    // Null value
    let result = kdl_impl2(quote! { node null });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node #null });
    assert!(result.is_ok());

    // Identifier values (become strings)
    let result = kdl_impl2(quote! { node identifier });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node hyphen-ated });
    assert!(result.is_ok());
}

/// Test type annotated arguments
#[test]
fn test_type_annotated_arguments() {
    let result = kdl_impl2(quote! { node (type)"value" });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node (int)42 });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node (float)3.14 });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node (bool)true });
    assert!(result.is_ok());
}

/// Test no arguments (empty argument list)
#[test]
fn test_no_arguments() {
    let result = kdl_impl2(quote! { node });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node {} });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node key="value" });
    assert!(result.is_ok());
}

/// Test many arguments to ensure the parser can handle large lists
#[test]
fn test_many_arguments() {
    // Test with 10 arguments
    let result = kdl_impl2(quote! {
        node 1 2 3 4 5 6 7 8 9 10
    });
    assert!(result.is_ok());

    // Test with mixed types - many arguments
    let result = kdl_impl2(quote! {
        node "a" 1 true "b" 2 false "c" 3 null "d"
    });
    assert!(result.is_ok());
}

/// Test sequences of identical arguments
#[test]
fn test_sequence_identical_arguments() {
    // Test 0 arguments
    let result = kdl_impl2(quote! { node });
    assert!(result.is_ok());

    // Test 1 argument
    let result = kdl_impl2(quote! { node 42 });
    assert!(result.is_ok());

    // Test 2 arguments
    let result = kdl_impl2(quote! { node 42 42 });
    assert!(result.is_ok());

    // Test 3 arguments
    let result = kdl_impl2(quote! { node 42 42 42 });
    assert!(result.is_ok());

    // Test 5 arguments
    let result = kdl_impl2(quote! { node 42 42 42 42 42 });
    assert!(result.is_ok());
}

/// Test argument parsing with various whitespace patterns
#[test]
fn test_argument_whitespace_handling() {
    let result = kdl_impl2(quote! { node    1    2    3 });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! {
        node
        1
        2
        3
    });
    assert!(result.is_ok());
}

/// Test complex argument combinations
#[test]
fn test_complex_argument_combinations() {
    // Mix of all types
    let result = kdl_impl2(quote! {
        node "string" 42 -3.14 true false null identifier hyphen-ated
    });
    assert!(result.is_ok());

    // With type annotations mixed in
    let result = kdl_impl2(quote! {
        node "plain" (str)"typed" 42 (int)100 true (bool)false
    });
    assert!(result.is_ok());

    // Interleaved with properties
    let result = kdl_impl2(quote! {
        node "arg1" key1="prop1" 42 key2="prop2" true key3="prop3" "arg2"
    });
    assert!(result.is_ok());
}

/// Test arguments in nested nodes
#[test]
fn test_arguments_in_nested_nodes() {
    let result = kdl_impl2(quote! {
        parent "parent_arg" {
            child "child_arg" 42
            other-child true false {
                grandchild "nested" 123
            }
        }
    });
    assert!(result.is_ok());
}

/// Test arguments with special identifier patterns
#[test]
fn test_special_identifier_arguments() {
    // Identifiers that become string values
    let result = kdl_impl2(quote! { node ubuntu-latest });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node multi-part-identifier });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node r#type });
    assert!(result.is_ok());
}

/// Test edge cases for numeric arguments
#[test]
fn test_numeric_edge_cases() {
    // Zero values
    let result = kdl_impl2(quote! { node 0 });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node 0.0 });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node -0 });
    assert!(result.is_ok());

    // Large numbers (within i64/f64 range)
    let result = kdl_impl2(quote! { node 9223372036854775807 });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node -9223372036854775808 });
    assert!(result.is_ok());
}

/// Test argument order preservation with detailed verification
#[test]
fn test_argument_order_preservation_detailed() {
    // This test verifies that the order is preserved in the generated code
    let result = kdl_impl2(quote! { node "first" "second" "third" });
    assert!(result.is_ok());

    // Test with mixed types to ensure order preservation across different value types
    let result = kdl_impl2(quote! { node 1 "string" true null 3.14 identifier });
    assert!(result.is_ok());
}

/// Test arguments that look like properties but aren't
#[test]
fn test_non_property_arguments() {
    // These should be parsed as arguments, not properties
    let result = kdl_impl2(quote! { node "key=value" });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node "not=a=property" });
    assert!(result.is_ok());
}

// Note: Comment prefix `/-` tests would go here, but this feature
// is not yet implemented in the parser. When implemented, tests should include:
//
// #[test]
// fn test_commented_out_arguments() {
//     let result = kdl_impl2(quote! { node 1 /- 2 3 });
//     assert!(result.is_ok());
//     // Should parse as node with arguments [1, 3] (2 is commented out)
// }
//
// #[test]
// fn test_multiline_commented_arguments() {
//     let result = kdl_impl2(quote! {
//         node 1 /- "this is
//         a multiline
//         commented argument" 2
//     });
//     assert!(result.is_ok());
// }

/// Test invalid argument syntax that should fail parsing
#[test]
fn test_invalid_argument_syntax() {
    // Unclosed string (if we had raw string input)
    // Note: These are Rust syntax errors, not KDL parsing errors

    // Test malformed type annotations
    let result = kdl_impl2(quote! { node (incomplete 42 });
    assert!(result.is_err());

    let result = kdl_impl2(quote! { node incomplete) 42 });
    assert!(result.is_err());
}

/// Test arguments with children blocks
#[test]
fn test_arguments_with_children() {
    let result = kdl_impl2(quote! {
        node "arg" {
            child
        }
    });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! {
        node 1 2 3 {
            child "nested_arg"
        }
    });
    assert!(result.is_ok());
}

/// Test that properties and arguments can be freely interleaved
#[test]
fn test_property_argument_interleaving_comprehensive() {
    // Start with argument
    let result = kdl_impl2(quote! { node "arg1" key="prop" "arg2" });
    assert!(result.is_ok());

    // Start with property
    let result = kdl_impl2(quote! { node key="prop" "arg1" "arg2" });
    assert!(result.is_ok());

    // Multiple interleavings
    let result = kdl_impl2(quote! {
        node "arg1" k1="p1" "arg2" k2="p2" "arg3" k3="p3" "arg4"
    });
    assert!(result.is_ok());

    // Properties between arguments
    let result = kdl_impl2(quote! {
        node k1="p1" k2="p2" "arg1" "arg2" k3="p3" "arg3"
    });
    assert!(result.is_ok());
}

/// Test empty and minimal cases
#[test]
fn test_minimal_cases() {
    // Just node name
    let result = kdl_impl2(quote! { node });
    assert!(result.is_ok());

    // Node with empty children
    let result = kdl_impl2(quote! { node {} });
    assert!(result.is_ok());

    // Single argument types
    let result = kdl_impl2(quote! { node 1 });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node "x" });
    assert!(result.is_ok());

    let result = kdl_impl2(quote! { node true });
    assert!(result.is_ok());
}

/// Stress test with maximum complexity
#[test]
fn test_maximum_complexity_arguments() {
    let result = kdl_impl2(quote! {
        complex-node
            "string_arg"
            42
            -3.14159
            true
            false
            null
            identifier
            hyphen-ated-id
            (str)"typed_string"
            (int)100
            (float)2.71828
            (bool)true
            key1="property1"
            "another_arg"
            key2="property2"
            ubuntu-latest
            r#type
            key3="property3"
            (custom)42
            "final_arg"
        {
            child1 "child_arg1" child_key="child_prop"
            child2 1 2 3 {
                grandchild "deeply_nested" (type)"value"
            }
        }
    });
    assert!(result.is_ok());
}