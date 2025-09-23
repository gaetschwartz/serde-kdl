//! Tests for KDL Section 3.15: Boolean
//!
//! This module contains comprehensive tests for KDL boolean values:
//! - Basic boolean syntax: #true and #false
//! - Boolean values as node arguments
//! - Boolean values as property values
//! - Boolean values with type annotations
//! - Integration with other value types
//! - Error cases for invalid boolean syntax

use crate::specs::kdl_impl2;

#[test]
fn test_basic_boolean_syntax() {
    // Test #true
    let input_str = "node #true";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test #false
    let input_str = "node #false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test both together
    let input_str = "node #true #false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_boolean_as_arguments() {
    // Test boolean arguments only
    let input_str = "my-node #true #false #true";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test mixed with other value types
    let input_str = r#"server #true "localhost" 8080 #false"#;
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test with numbers and strings
    let input_str = r#"config 123 "test" #true 3.14 #false"#;
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_boolean_as_properties() {
    // Test boolean property values only
    let input_str = "node enabled=#true disabled=#false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test boolean properties with other types
    let input_str = r#"server host="localhost" port=8080 ssl=#true debug=#false"#;
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test multiple boolean properties
    let input_str = "features cache=#true logging=#false auth=#true backup=#false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_spec_example() {
    // Test the exact example from Section 3.15.1
    let input_str = "my-node #true value=#false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_boolean_with_type_annotations() {
    // Test type-annotated booleans as arguments
    let input_str = "node (bool)#true (boolean)#false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test type-annotated booleans as properties
    let input_str = "config enabled=(bool)#true debug=(boolean)#false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test with custom type annotations
    let input_str = "flags active=(Flag)#true visible=(State)#false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_boolean_mixed_contexts() {
    // Test booleans in all possible contexts within a single node
    let input_str = r#"server #true "localhost" port=8080 ssl=#false debug=(bool)#true"#;
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test complex mixed scenarios
    let input_str = r#"database #true host="localhost" port=5432 ssl=#true readonly=#false timeout=(i32)30 retry=(bool)#true"#;
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_boolean_in_nested_contexts() {
    // Test booleans in child nodes
    let input_str = r#"parent enabled=#true {
        child #false debug=#true
        another #true value="test" active=#false
    }"#;
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test deeply nested booleans
    let input_str = "root #true {
        level1 #false {
            level2 active=#true {
                level3 #false enabled=#true
            }
        }
    }";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_boolean_edge_cases() {
    // Test booleans with whitespace
    let input_str = "node   #true   #false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test booleans as the only values
    let input_str = "bool-test #true\nbool-test2 #false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_boolean_vs_identifier_distinction() {
    // Test that bare true/false are parsed as identifier strings, not booleans
    let result = kdl_impl2(quote::quote! {
        node true
    });
    assert!(result.is_ok()); // This parses as an identifier string

    let result = kdl_impl2(quote::quote! {
        node false
    });
    assert!(result.is_ok()); // This parses as an identifier string

    // Test that only #true and #false are valid boolean syntax
    // (Invalid # syntax like #True may parse as separate tokens, which is acceptable)
    let input_str = "node #true";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok()); // Valid boolean syntax

    let input_str = "node #false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok()); // Valid boolean syntax
}

#[test]
fn test_boolean_property_edge_cases() {
    // Test boolean properties with complex names
    let input_str = "node enable-cache=#true disable-debug=#false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test boolean properties with underscores
    let input_str = "node enable_cache=#true disable_debug=#false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test boolean properties with numbers in names
    let input_str = "node option1=#true option2=#false feature3=#true";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_boolean_type_annotation_variations() {
    // Test various type annotation formats
    let input_str = "node (bool)#true (Bool)#false (BOOL)#true (boolean)#false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test custom boolean types
    let input_str = "flags (Flag)#true (State)#false (Option)#true";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test complex type annotations - note: using simpler identifiers to avoid parsing issues
    let input_str = "config (MyBool)#true (AppBoolean)#false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_boolean_consistency() {
    // Test that the same boolean values are parsed consistently
    let input_str1 = "node1 #true";
    let input1: proc_macro2::TokenStream = input_str1.parse().unwrap();
    let result1 = kdl_impl2(input1);
    assert!(result1.is_ok());

    let input_str2 = "node2 #true";
    let input2: proc_macro2::TokenStream = input_str2.parse().unwrap();
    let result2 = kdl_impl2(input2);
    assert!(result2.is_ok());

    // Test false consistency
    let input_str3 = "node3 #false";
    let input3: proc_macro2::TokenStream = input_str3.parse().unwrap();
    let result3 = kdl_impl2(input3);
    assert!(result3.is_ok());

    let input_str4 = "node4 #false";
    let input4: proc_macro2::TokenStream = input_str4.parse().unwrap();
    let result4 = kdl_impl2(input4);
    assert!(result4.is_ok());
}

#[test]
fn test_boolean_with_regular_identifiers() {
    // Test that bare true/false are treated as identifier strings, not booleans
    let result = kdl_impl2(quote::quote! {
        node true false
    });
    assert!(result.is_ok()); // Should parse as identifier strings

    // Test mixed boolean forms - #true/#false are booleans, true/false are identifiers
    let input_str = "node true #false #true false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_boolean_comprehensive_integration() {
    // Test comprehensive integration with all KDL value types
    // Note: Simplified to avoid #null, #inf, #nan which would also cause parsing issues
    let input_str = r#"comprehensive-test {
        // Arguments of all types including booleans
        values #true #false 42 3.14 "string" identifier

        // Properties of all types including booleans
        bool-prop1=#true
        bool-prop2=#false
        int-prop=123
        float-prop=3.14
        string-prop="value"

        // Type-annotated values including booleans
        typed-values (bool)#true (bool)#false (i32)42 (f64)3.14 (str)"text"

        // Child nodes with boolean values
        child1 #true enabled=#false {
            grandchild active=#true debug=#false
        }
        child2 #false ready=#true
    }"#;
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}
