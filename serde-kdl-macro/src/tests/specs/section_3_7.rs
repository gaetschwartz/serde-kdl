//! Tests for KDL Section 3.7: Value
//!
//! According to the KDL specification Section 3.7:
//! - A value is either: String (Section 3.9), Number (Section 3.14), Boolean (Section 3.15), or Null (Section 3.16)
//! - Values MUST be either Arguments (Section 3.5) or values of Properties (Section 3.4)
//! - Only String values may be used as Node names or Property keys
//! - Values (both as arguments and in properties) MAY be prefixed by a single Type Annotation (Section 3.8)

use crate::tests::specs::kdl_impl2;
use quote::quote;

#[test]
fn test_string_values_as_arguments() {
    // Test basic string values as arguments
    let input = quote! { node "string" };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "String values should be valid arguments");

    // Test multiple string arguments
    let input = quote! { node "string1" "string2" "string3" };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "Multiple string values should be valid arguments"
    );

    // Test bare string identifiers as arguments
    let input = quote! { node bare-string };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "Bare string identifiers should be valid arguments"
    );
}

#[test]
fn test_number_values_as_arguments() {
    // Test integer values as arguments
    let input = quote! { node 42 };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "Integer values should be valid arguments");

    // Test float values as arguments
    let input = quote! { node 3.14 };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "Float values should be valid arguments");

    // Test negative numbers as arguments
    let input = quote! { node -42 -3.14 };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "Negative numbers should be valid arguments");

    // Test multiple number arguments
    let input = quote! { node 42 3.14 -10 0 };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "Multiple number values should be valid arguments"
    );
}

#[test]
fn test_boolean_values_as_arguments() {
    // Test boolean literals as arguments
    let input = quote! { node true false };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "Boolean literals should be valid arguments");

    // Test # syntax for booleans as arguments
    let input = quote! { node #true #false };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "# syntax booleans should be valid arguments"
    );

    // Test mixed boolean forms
    let input = quote! { node true #false #true false };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "Mixed boolean forms should be valid arguments"
    );
}

#[test]
fn test_null_values_as_arguments() {
    // Test null as argument
    let input = quote! { node null };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "Null should be a valid argument");

    // Test # syntax for null through direct parsing
    let input_str = "node #null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "# syntax null should be a valid argument");

    // Test multiple nulls
    let input = quote! { node null null null };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "Multiple null values should be valid arguments"
    );

    // Test mixed null forms through direct parsing
    let input_str = "node null #null null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "Mixed null forms should be valid arguments");
}

#[test]
fn test_mixed_value_types_as_arguments() {
    // Test all value types mixed as arguments
    let input = quote! { node "string" 42 true null };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "Mixed value types should be valid arguments"
    );

    // Test complex mix with different number types
    let input = quote! { node "text" 42 3.14 -10 true false null "end" };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "Complex mix of value types should be valid arguments"
    );
}

#[test]
fn test_string_values_as_property_values() {
    // Test string property values
    let input = quote! { node key="string" };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "String should be valid as property value");

    // Test multiple string properties
    let input = quote! { node key1="value1" key2="value2" };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "Multiple string properties should be valid");
}

#[test]
fn test_number_values_as_property_values() {
    // Test integer property values
    let input = quote! { node key=42 };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "Integer should be valid as property value");

    // Test float property values
    let input = quote! { node key=3.14 };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "Float should be valid as property value");

    // Test negative numbers as property values
    let input = quote! { node key=-42 };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "Negative number should be valid as property value"
    );
}

#[test]
fn test_boolean_values_as_property_values() {
    // Test boolean property values
    let input = quote! { node key=true };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "Boolean should be valid as property value");

    // Test false property value
    let input = quote! { node key=false };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "False should be valid as property value");

    // Test # syntax booleans as property values
    let input = quote! { node key=#true };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "# syntax boolean should be valid as property value"
    );
}

#[test]
fn test_null_values_as_property_values() {
    // Test null property values
    let input = quote! { node key=null };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "Null should be valid as property value");

    // Test # syntax null as property value through direct parsing
    let input_str = "node key=#null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "# syntax null should be valid as property value"
    );
}

#[test]
fn test_mixed_property_value_types() {
    // Test all value types as properties
    let input = quote! { node str="text" num=42 bool=true null_val=null };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "All value types should be valid as property values"
    );

    // Test complex mix with floats and negative numbers
    let input = quote! { node a="string" b=42 c=3.14 d=-10 e=true f=false g=null };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "Complex mix should be valid as property values"
    );
}

#[test]
fn test_string_node_names() {
    // Test quoted string node names
    let input = quote! { "string-node" arg };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "Quoted string should be valid as node name");

    // Test bare string identifiers as node names
    let input = quote! { bare-string-node arg };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "Bare string identifier should be valid as node name"
    );

    // Test hyphenated node names
    let input = quote! { multi-part-name arg };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "Hyphenated names should be valid as node names"
    );
}

#[test]
fn test_string_property_keys() {
    // Test quoted string property keys
    let input = quote! { node "string-key"=value };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "Quoted string should be valid as property key"
    );

    // Test bare string identifiers as property keys
    let input = quote! { node bare-key=value };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "Bare string identifier should be valid as property key"
    );

    // Test hyphenated property keys
    let input = quote! { node multi-part-key=value };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "Hyphenated keys should be valid as property keys"
    );
}

#[test]
fn test_complex_value_combinations() {
    // Test complex document with all value types in various positions
    let input = quote! {
        "root-node" "string-arg" 42 true null {
            child-node str="text" num=42 bool=true null_val=null "another-arg" 3.14
            "quoted-child" key="value" -10 false
        }
    };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "Complex value combinations should be valid");
}

#[test]
fn test_arguments_and_properties_mixed() {
    // Test that values work correctly as both arguments and properties in the same node
    let input = quote! { node "arg1" key1="prop1" 42 key2=true "arg2" key3=null false };
    let result = kdl_impl2(input);
    assert!(
        result.is_ok(),
        "Mixed arguments and properties should be valid"
    );

    // Test order independence
    let input = quote! { node key1="first" "arg1" key2=42 "arg2" key3=true 100 key4=null };
    let result = kdl_impl2(input);
    assert!(result.is_ok(), "Order-independent mixing should be valid");
}

#[test]
fn test_value_type_annotation_support() {
    // Test that type-annotated values are supported (preparing for Section 3.8)
    let input_str = "node (type)\"string\"";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    if let Err(e) = &result {
        println!("Error: {}", e);
    }
    assert!(result.is_ok(), "Type-annotated string should be supported");

    // Test simple type-annotated integer
    let input_str = "node (i32)42";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    if let Err(e) = &result {
        println!("Error: {}", e);
    }
    assert!(result.is_ok(), "Type-annotated integer should be supported");
}

#[test]
fn test_section_3_7_compliance() {
    // Comprehensive test that demonstrates Section 3.7 compliance
    // Note: Simplified to avoid #null syntax which requires type annotation parsing
    let input = quote! {
        // String values in all valid positions
        "string-node" "string-arg" string-key="string-value"

        // Number values as arguments and property values
        number-test 42 3.14 -10 0 int-key=100 float-key=2.71

        // Boolean values as arguments and property values
        bool-test true false bool-key=true flag=false

        // Mixed value types
        mixed "text" 42 true text="string" num=100 flag=false
    };
    let result = kdl_impl2(input);
    if let Err(e) = &result {
        println!("Error: {}", e);
    }
    assert!(
        result.is_ok(),
        "Section 3.7 compliant document should be valid"
    );
}
