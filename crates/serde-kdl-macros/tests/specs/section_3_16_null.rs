//! Tests for KDL Section 3.16: Null
//!
//! This module tests KDL null values (`#null`):
//! - Null as arguments and properties
//! - Null with type annotations
//! - Null mixed with other value types

use super::doc_to_string;
use insta::assert_snapshot;
use serde_kdl_macros::kdl;

#[test]
fn test_null_as_arguments_and_properties() {
    // Test null in various contexts: as arguments, properties, and with type annotations
    let doc = kdl! {
        "my-node" #null key=#null
        server #null "localhost" port=8080 ssl=#null
        config value=(String)#null data=(Option)#null
    };

    assert_snapshot!(doc_to_string(doc), @r"
    my-node key=#null #null
    server port=8080 ssl=#null #null localhost
    config value=(String)#null data=(Option)#null
    ");
}

#[test]
fn test_null_with_mixed_types() {
    // Test null alongside other KDL value types to ensure proper parsing
    let doc = kdl! {
        mixed #null #true #false 42 3.14 "string"
        props null_val=#null bool_val=#true int_val=123 str_val="text"
        typed (opt)#null (bool)#true (i32)42 (str)"hello"
    };

    assert_snapshot!(doc_to_string(doc), @r#"
mixed #null #true #false 42 3.14 string
props null_val=#null bool_val=#true int_val=123 str_val=text
typed (opt)#null (bool)#true (i32)42 (str)hello
"#);
}

#[test]
fn test_null_in_nested_structures() {
    // Test null values in nested node hierarchies
    let doc = kdl! {
        parent value=#null {
            child1 #null enabled=#true
            child2 data="test" cache=#null {
                grandchild #null active=#null
            }
        }
        root #null {
            nested #null prop=#null
        }
    };

    assert_snapshot!(doc_to_string(doc), @r"
    parent value=#null {
        child1 enabled=#true #null
        child2 data=test cache=#null {
            grandchild active=#null #null
        }
    }
    root #null {
        nested prop=#null #null
    }
    ");
}
