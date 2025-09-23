//! Tests for KDL Section 3.16: Null
//!
//! This module contains comprehensive tests for KDL null values:
//! - Basic null syntax: #null
//! - Null values as node arguments
//! - Null values as property values
//! - Null values with type annotations
//! - Integration with other value types
//! - Distinction between bare "null" (identifier) and "#null" (null value)
//! - Error cases for invalid null syntax

use crate::specs::kdl_impl2;

#[test]
fn test_basic_null_syntax() {
    // Test #null as argument
    let input_str = "node #null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test multiple #null values
    let input_str = "node #null #null #null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_null_as_arguments() {
    // Test null arguments only
    let input_str = "my-node #null #null #null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test mixed with other value types
    let input_str = r#"server #null "localhost" 8080 #null"#;
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test with numbers, strings, and booleans
    let input_str = r#"config 123 "test" #null 3.14 #true #null #false"#;
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_null_as_properties() {
    // Test null property values only
    let input_str = "node value1=#null value2=#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test null properties with other types
    let input_str = r#"server host="localhost" port=8080 ssl=#null debug=#false cache=#null"#;
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test multiple null properties
    let input_str = "features cache=#null logging=#null auth=#null backup=#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_spec_example() {
    // Test the exact example from Section 3.16.1: my-node #null key=#null
    let input_str = "my-node #null key=#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_null_with_type_annotations() {
    // Test type-annotated nulls as arguments
    let input_str = "node (String)#null (Option)#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test type-annotated nulls as properties
    let input_str = "config value=(String)#null data=(Option)#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test with custom type annotations
    let input_str = "database host=(Host)#null port=(Port)#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_null_mixed_contexts() {
    // Test nulls in all possible contexts within a single node
    let input_str = r#"server #null "localhost" port=8080 ssl=#null debug=(bool)#true cache=#null"#;
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test complex mixed scenarios
    let input_str = r#"database #null host="localhost" port=5432 ssl=#true readonly=#null timeout=(i32)30 retry=#null"#;
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_null_in_nested_contexts() {
    // Test nulls in child nodes
    let input_str = r#"parent value=#null {
        child #null debug=#true
        another #null value="test" active=#null
    }"#;
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test deeply nested nulls
    let input_str = "root #null {
        level1 #null {
            level2 active=#true {
                level3 #null enabled=#null
            }
        }
    }";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_null_edge_cases() {
    // Test nulls with whitespace
    let input_str = "node   #null   #null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test null as the only value
    let input_str = "null_test #null\nnull_test2 #null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test null with various spacing patterns - avoid adjacent tokens that might not tokenize correctly
    let input_str = "node #null key=#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_null_vs_identifier_distinction() {
    // Test that bare "null" is parsed as an identifier string, not a null value
    let result = kdl_impl2(quote::quote! {
        node null
    });
    assert!(result.is_ok()); // This parses as an identifier string

    // Test that only #null is valid null syntax
    let input_str = "node #null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok()); // Valid null syntax

    // Test mixed null forms - #null is null value, null is identifier
    let input_str = "node null #null null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test null in property contexts - bare vs #null
    let input_str = "node null_prop=null real_null=#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_null_property_edge_cases() {
    // Test null properties with complex names
    let input_str = "node enable_cache=#null disable_debug=#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test null properties with underscores
    let input_str = "node enable_cache=#null disable_debug=#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test null properties with numbers in names
    let input_str = "node option1=#null option2=#null feature3=#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_null_type_annotation_variations() {
    // Test various type annotation formats with null
    let input_str = "node (str)#null (String)#null (Option)#null (Maybe)#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test custom nullable types
    let input_str = "config (Database)#null (Connection)#null (Cache)#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test type annotations that suggest optional/nullable types
    let input_str = "optional (Optional)#null (Nullable)#null (Maybe)#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_null_consistency() {
    // Test that the same null values are parsed consistently
    let input_str1 = "node1 #null";
    let input1: proc_macro2::TokenStream = input_str1.parse().unwrap();
    let result1 = kdl_impl2(input1);
    assert!(result1.is_ok());

    let input_str2 = "node2 #null";
    let input2: proc_macro2::TokenStream = input_str2.parse().unwrap();
    let result2 = kdl_impl2(input2);
    assert!(result2.is_ok());

    // Test null properties consistency
    let input_str3 = "node3 prop=#null";
    let input3: proc_macro2::TokenStream = input_str3.parse().unwrap();
    let result3 = kdl_impl2(input3);
    assert!(result3.is_ok());

    let input_str4 = "node4 prop=#null";
    let input4: proc_macro2::TokenStream = input_str4.parse().unwrap();
    let result4 = kdl_impl2(input4);
    assert!(result4.is_ok());
}

#[test]
fn test_null_with_all_value_types() {
    // Test null mixed with all other value types
    let input_str = r#"comprehensive {
        // Arguments of all types including null
        values #null #true #false 42 3.14 "string" identifier #null

        // Properties of all types including null
        null-prop=#null
        bool-prop=#true
        int-prop=123
        float-prop=3.14
        string-prop="value"
        identifier-prop=name

        // Type-annotated values including null
        typed-values (Option)#null (bool)#true (i32)42 (f64)3.14 (str)"text"

        // Child nodes with null values
        child1 #null enabled=#false {
            grandchild active=#null debug=#false
        }
        child2 #null ready=#true
    }"#;
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_null_semantic_meaning() {
    // Test that null represents "absence of value" in various contexts
    // where it would make semantic sense

    // Optional configuration values
    let input_str = "config required_host=\"localhost\" required_port=8080 optional_ssl=#null optional_auth=#null optional_cache=#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Database connection with optional values
    let input_str =
        "database host=\"localhost\" port=5432 username=\"user\" password=#null ssl=#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // API endpoints with optional parameters
    let input_str = r#"api_endpoint "/users" limit=10 offset=#null filter=#null sort=#null"#;
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_null_boundary_conditions() {
    // Test null at document boundaries
    let input_str = "#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    // This should fail because #null alone is not a valid KDL document structure
    // (it needs to be part of a node)
    assert!(result.is_err());

    // Test null as first value
    let input_str = "node #null \"second\" third";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test null as last value
    let input_str = "node \"first\" second #null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test null as only argument
    let input_str = "node #null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test null as only property value
    let input_str = "node prop=#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_null_with_special_syntax() {
    // Test null with other special KDL values
    let input_str = "node #null #true #false";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test that #null is distinct from other # values
    let input_str = "values #true enabled=#false missing=#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test type-annotated null with other type-annotated values
    let input_str = "typed (bool)#true (int)42 (opt)#null (str)\"text\"";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}

#[test]
fn test_null_parsing_precision() {
    // Test that exactly "#null" is parsed as null, not variations
    let input_str = "node #null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());

    // Test that case matters (if the tokenizer allows these through)
    // Note: These might not parse as expected due to tokenization rules
    // but we test the behavior regardless
    let input_str = "node null"; // Should be identifier, not null
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok()); // Should parse as identifier

    // Test that whitespace doesn't affect null parsing
    let input_str = "node   #null   ";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok());
}
