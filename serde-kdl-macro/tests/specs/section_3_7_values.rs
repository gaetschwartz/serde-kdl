//! Tests for KDL Section 3.7: Value
//!
//! According to the KDL specification Section 3.7:
//! - A value is either: String (Section 3.9), Number (Section 3.14), Boolean (Section 3.15), or Null (Section 3.16)
//! - Values MUST be either Arguments (Section 3.5) or values of Properties (Section 3.4)
//! - Only String values may be used as Node names or Property keys
//! - Values (both as arguments and in properties) MAY be prefixed by a single Type Annotation (Section 3.8)

use super::doc_to_string;
use insta::assert_snapshot;
use serde_kdl_macro::kdl;

#[test]
fn test_all_value_types_as_arguments_and_properties() {
    // Test that all four value types (String, Number, Boolean, Null) work as both
    // arguments and property values, demonstrating Section 3.7 compliance
    let doc = kdl! {
        // String values
        strings "hello" "world" text="value" name="test"

        // Number values (integers and floats)
        numbers 42 -10 3.14 -2.71 count=100 ratio=0.5

        // Boolean values
        booleans #true #false enabled=#true debug=#false

        // Null values
        nulls #null #null data=#null cache=#null

        // All types mixed together
        mixed "text" 42 #true #null str="hello" num=99 bool=#false null_val=#null
    };

    assert_snapshot!(doc_to_string(doc), @r#"
    strings hello world text=value name=test
    numbers 42 -10 3.14 -2.71 count=100 ratio=0.5
    booleans #true #false enabled=#true debug=#false
    nulls #null #null data=#null cache=#null
    mixed text 42 #true #null str=hello num=99 bool=#false null_val=#null
    "#);
}

#[test]
fn test_strings_as_node_names_and_property_keys() {
    // Test that only String values are valid for node names and property keys,
    // as specified in Section 3.7
    let doc = kdl! {
        simple_node "arg"
        "quoted-node" "arg"
        node1 "simple-key"="value"
        node2 "quoted-key"="value"
    };

    assert_snapshot!(doc_to_string(doc), @r#"
    simple_node arg quoted-node arg
    node1 simple-key=value
    node2 quoted-key=value
    "#);
}

#[test]
fn test_values_with_type_annotations() {
    // Test that values can be prefixed with type annotations (Section 3.8),
    // verifying Section 3.7's statement that values MAY have type annotations
    let doc = kdl! {
        node (i32)42 (f64)3.14 (str)"hello" (bool)#true (opt)#null
        typed num=(i32)100 text=(String)"value" flag=(bool)#false
    };

    assert_snapshot!(doc_to_string(doc), @r#"
    node (i32)42 (f64)3.14 (str)hello (bool)#true (opt)#null
    typed num=(i32)100 text=(String)value flag=(bool)#false
    "#);
}
