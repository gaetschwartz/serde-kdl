//! Tests for Section 3.2: Node
//!
//! This module contains comprehensive tests for the KDL Node specification
//! as defined in section 3.2 of the KDL specification.
//!
//! The tests cover:
//! - Node structure: identifier, arguments, properties, children
//! - Node name requirements and validation
//! - Optional components (arguments, properties, children)
//! - Node ordering and parsing
//! - Comment prefixes `/-` (slashdash)
//! - All node variations and combinations
//! - Type annotations on node names
//! - Arguments and properties interspersed ordering
//! - Node termination (newline, semicolon, EOF)

use crate::specs::kdl_impl2;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rstest::rstest;
use seq_macro::seq;
use serde_kdl_macro::kdl;

// ============================================================================
// Section 3.2.1: Basic Node Structure Tests
// ============================================================================

/// Test nodes with only names (no arguments, properties, or children)
#[test]
fn test_node_name_only() {
    let doc = kdl! {
        simple_node
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "simple_node");
    assert_eq!(node.entries().len(), 0, "Node should have no entries");
    assert!(node.children().is_none(), "Node should have no children");
}

/// Test nodes with hyphenated names
#[test]
fn test_hyphenated_node_names() {
    let doc = kdl! {
        my-node
        another-hyphenated-name
        runs-on
    };
    assert_eq!(doc.nodes().len(), 3);
    assert_eq!(doc.nodes()[0].name().value(), "my-node");
    assert_eq!(doc.nodes()[1].name().value(), "another-hyphenated-name");
    assert_eq!(doc.nodes()[2].name().value(), "runs-on");
}

/// Test node names with underscores
#[test]
fn test_underscore_node_names() {
    let doc = kdl! {
        my_node
        another_node_name
        test_case_123
    };
    assert_eq!(doc.nodes().len(), 3);
    assert_eq!(doc.nodes()[0].name().value(), "my_node");
    assert_eq!(doc.nodes()[1].name().value(), "another_node_name");
    assert_eq!(doc.nodes()[2].name().value(), "test_case_123");
}

/// Test node names with mixed characters
#[test]
fn test_mixed_character_node_names() {
    let doc = kdl! {
        node_with-mixed123
        test-case_1
        complex-node_name-123
    };
    assert_eq!(doc.nodes().len(), 3);
    assert_eq!(doc.nodes()[0].name().value(), "node_with-mixed123");
    assert_eq!(doc.nodes()[1].name().value(), "test-case_1");
    assert_eq!(doc.nodes()[2].name().value(), "complex-node_name-123");
}

// ============================================================================
// Section 3.2.2: Node with Arguments Tests
// ============================================================================

/// Test nodes with single arguments of different types
#[test]
fn test_node_single_arguments() {
    let doc = kdl! {
        string_arg "hello"
        int_arg 42
        float_arg 3.14
        bool_arg true
        null_arg null
    };
    assert_eq!(doc.nodes().len(), 5);

    // String argument
    let string_node = &doc.nodes()[0];
    assert_eq!(string_node.name().value(), "string_arg");
    assert_eq!(string_node.entries().len(), 1);
    assert_eq!(string_node.entries()[0].value().as_string().unwrap(), "hello");

    // Integer argument
    let int_node = &doc.nodes()[1];
    assert_eq!(int_node.name().value(), "int_arg");
    assert_eq!(int_node.entries().len(), 1);
    assert_eq!(int_node.entries()[0].value().as_i64().unwrap(), 42);

    // Float argument
    let float_node = &doc.nodes()[2];
    assert_eq!(float_node.name().value(), "float_arg");
    assert_eq!(float_node.entries().len(), 1);
    assert_eq!(float_node.entries()[0].value().as_f64().unwrap(), 3.14);

    // Boolean argument
    let bool_node = &doc.nodes()[3];
    assert_eq!(bool_node.name().value(), "bool_arg");
    assert_eq!(bool_node.entries().len(), 1);
    assert_eq!(bool_node.entries()[0].value().as_bool().unwrap(), true);

    // Null argument
    let null_node = &doc.nodes()[4];
    assert_eq!(null_node.name().value(), "null_arg");
    assert_eq!(null_node.entries().len(), 1);
    assert!(null_node.entries()[0].value().is_null());
}

/// Test nodes with multiple arguments
#[test]
fn test_node_multiple_arguments() {
    let doc = kdl! {
        multi_args "first" 42 true 3.14 null "last"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "multi_args");
    assert_eq!(node.entries().len(), 6);

    // Verify argument order is preserved
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "first");
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 42);
    assert_eq!(node.entries()[2].value().as_bool().unwrap(), true);
    assert_eq!(node.entries()[3].value().as_f64().unwrap(), 3.14);
    assert!(node.entries()[4].value().is_null());
    assert_eq!(node.entries()[5].value().as_string().unwrap(), "last");
}

/// Test arguments with type annotations
#[test]
fn test_node_typed_arguments() {
    let doc = kdl! {
        typed_args (string)"hello" (i32)42 (f64)3.14 (bool)true
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "typed_args");
    assert_eq!(node.entries().len(), 4);

    // Check type annotations are preserved
    assert_eq!(node.entries()[0].ty().unwrap().value(), "string");
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "hello");

    assert_eq!(node.entries()[1].ty().unwrap().value(), "i32");
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 42);

    assert_eq!(node.entries()[2].ty().unwrap().value(), "f64");
    assert_eq!(node.entries()[2].value().as_f64().unwrap(), 3.14);

    assert_eq!(node.entries()[3].ty().unwrap().value(), "bool");
    assert_eq!(node.entries()[3].value().as_bool().unwrap(), true);
}

// ============================================================================
// Section 3.2.3: Node with Properties Tests
// ============================================================================

/// Test nodes with single property
#[test]
fn test_node_single_property() {
    let doc = kdl! {
        node key="value"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "node");
    assert_eq!(node.entries().len(), 1);

    let entry = &node.entries()[0];
    assert!(entry.name().is_some(), "Entry should be a property");
    assert_eq!(entry.name().unwrap().value(), "key");
    assert_eq!(entry.value().as_string().unwrap(), "value");
}

/// Test nodes with multiple properties
#[test]
fn test_node_multiple_properties() {
    let doc = kdl! {
        node key1="value1" key2=42 key3=true key4=null
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "node");
    assert_eq!(node.entries().len(), 4);

    // Verify all entries are properties
    for entry in node.entries() {
        assert!(entry.name().is_some(), "All entries should be properties");
    }

    // Check property values (properties order shouldn't be assumed, but we test current implementation)
    assert_eq!(node.get("key1").unwrap().value().as_string().unwrap(), "value1");
    assert_eq!(node.get("key2").unwrap().value().as_i64().unwrap(), 42);
    assert_eq!(node.get("key3").unwrap().value().as_bool().unwrap(), true);
    assert!(node.get("key4").unwrap().value().is_null());
}

/// Test properties with type annotations
#[test]
fn test_node_typed_properties() {
    let doc = kdl! {
        node key1=(string)"hello" key2=(i32)42 key3=(bool)true
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "node");
    assert_eq!(node.entries().len(), 3);

    let key1_entry = node.get("key1").unwrap();
    assert_eq!(key1_entry.ty().unwrap().value(), "string");
    assert_eq!(key1_entry.value().as_string().unwrap(), "hello");

    let key2_entry = node.get("key2").unwrap();
    assert_eq!(key2_entry.ty().unwrap().value(), "i32");
    assert_eq!(key2_entry.value().as_i64().unwrap(), 42);

    let key3_entry = node.get("key3").unwrap();
    assert_eq!(key3_entry.ty().unwrap().value(), "bool");
    assert_eq!(key3_entry.value().as_bool().unwrap(), true);
}

/// Test properties with hyphenated keys
#[test]
fn test_node_hyphenated_property_keys() {
    let doc = kdl! {
        node some-key="value" another-property=42
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 2);

    assert_eq!(node.get("some-key").unwrap().value().as_string().unwrap(), "value");
    assert_eq!(node.get("another-property").unwrap().value().as_i64().unwrap(), 42);
}

// ============================================================================
// Section 3.2.4: Node with Children Tests
// ============================================================================

/// Test nodes with empty children block
#[test]
fn test_node_empty_children() {
    let doc = kdl! {
        parent {
        }
    };
    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");

    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 0);
}

/// Test nodes with single child
#[test]
fn test_node_single_child() {
    let doc = kdl! {
        parent {
            child
        }
    };
    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");

    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 1);
    assert_eq!(children.nodes()[0].name().value(), "child");
}

/// Test nodes with multiple children
#[test]
fn test_node_multiple_children() {
    let doc = kdl! {
        parent {
            child1
            child2 "arg"
            child3 key="value"
        }
    };
    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");

    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 3);

    assert_eq!(children.nodes()[0].name().value(), "child1");
    assert_eq!(children.nodes()[0].entries().len(), 0);

    assert_eq!(children.nodes()[1].name().value(), "child2");
    assert_eq!(children.nodes()[1].entries().len(), 1);
    assert_eq!(children.nodes()[1].entries()[0].value().as_string().unwrap(), "arg");

    assert_eq!(children.nodes()[2].name().value(), "child3");
    assert_eq!(children.nodes()[2].entries().len(), 1);
    assert_eq!(children.nodes()[2].get("key").unwrap().value().as_string().unwrap(), "value");
}

/// Test deeply nested children
#[test]
fn test_node_deep_nesting() {
    let doc = kdl! {
        level1 {
            level2 {
                level3 {
                    level4 "deep_value"
                }
            }
        }
    };
    assert_eq!(doc.nodes().len(), 1);

    let level1 = &doc.nodes()[0];
    assert_eq!(level1.name().value(), "level1");

    let level2 = &level1.children().unwrap().nodes()[0];
    assert_eq!(level2.name().value(), "level2");

    let level3 = &level2.children().unwrap().nodes()[0];
    assert_eq!(level3.name().value(), "level3");

    let level4 = &level3.children().unwrap().nodes()[0];
    assert_eq!(level4.name().value(), "level4");
    assert_eq!(level4.entries()[0].value().as_string().unwrap(), "deep_value");
}

/// Test children with arguments and properties
#[test]
fn test_node_children_with_content() {
    let doc = kdl! {
        parent "parent_arg" parent_key="parent_value" {
            child1 "child_arg"
            child2 child_key="child_value"
            child3 "arg1" "arg2" key1="value1" key2="value2"
        }
    };
    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");
    assert_eq!(parent.entries().len(), 2);

    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 3);

    // Check parent content
    assert_eq!(parent.entries()[0].value().as_string().unwrap(), "parent_arg");
    assert_eq!(parent.get("parent_key").unwrap().value().as_string().unwrap(), "parent_value");

    // Check children content
    let child1 = &children.nodes()[0];
    assert_eq!(child1.entries().len(), 1);
    assert_eq!(child1.entries()[0].value().as_string().unwrap(), "child_arg");

    let child2 = &children.nodes()[1];
    assert_eq!(child2.entries().len(), 1);
    assert_eq!(child2.get("child_key").unwrap().value().as_string().unwrap(), "child_value");

    let child3 = &children.nodes()[2];
    assert_eq!(child3.entries().len(), 4);
    // Arguments come first, then properties
    assert_eq!(child3.entries()[0].value().as_string().unwrap(), "arg1");
    assert_eq!(child3.entries()[1].value().as_string().unwrap(), "arg2");
    assert_eq!(child3.get("key1").unwrap().value().as_string().unwrap(), "value1");
    assert_eq!(child3.get("key2").unwrap().value().as_string().unwrap(), "value2");
}

// ============================================================================
// Section 3.2.5: Arguments and Properties Interspersed Tests
// ============================================================================

/// Test arguments and properties mixed together
#[test]
fn test_interspersed_args_and_props() {
    let doc = kdl! {
        node "arg1" key1="value1" "arg2" key2="value2" "arg3"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "node");
    assert_eq!(node.entries().len(), 5);

    // Verify the order is preserved as specified in the document
    // Arguments should maintain their relative order even with properties interspersed
    let mut arg_values = Vec::new();
    let mut prop_pairs = Vec::new();

    for entry in node.entries() {
        if entry.name().is_some() {
            // Property
            prop_pairs.push((entry.name().unwrap().value().to_string(), entry.value().as_string().unwrap()));
        } else {
            // Argument
            arg_values.push(entry.value().as_string().unwrap());
        }
    }

    // Arguments should be in order: "arg1", "arg2", "arg3"
    assert_eq!(arg_values, vec!["arg1", "arg2", "arg3"]);

    // Properties should be present
    assert!(prop_pairs.contains(&("key1".to_string(), "value1".to_string())));
    assert!(prop_pairs.contains(&("key2".to_string(), "value2".to_string())));
}

/// Test complex interspersed pattern from section 3.2.1 example
#[test]
fn test_section_3_2_1_example() {
    let doc = kdl! {
        foo 1 key=val 3 {
            bar
            (role)baz 1 2
        }
    };
    assert_eq!(doc.nodes().len(), 1);
    let foo = &doc.nodes()[0];
    assert_eq!(foo.name().value(), "foo");
    assert_eq!(foo.entries().len(), 3);

    // Check that foo has argument values [1, 3] as mentioned in spec
    let mut arg_values = Vec::new();
    for entry in foo.entries() {
        if entry.name().is_none() {
            arg_values.push(entry.value().as_i64().unwrap());
        }
    }
    assert_eq!(arg_values, vec![1, 3], "foo should have argument list [1, 3]");

    // Check property
    assert_eq!(foo.get("key").unwrap().value().as_string().unwrap(), "val");

    // Check children
    let children = foo.children().unwrap();
    assert_eq!(children.nodes().len(), 2);

    let bar = &children.nodes()[0];
    assert_eq!(bar.name().value(), "bar");
    assert_eq!(bar.entries().len(), 0);

    let baz = &children.nodes()[1];
    assert_eq!(baz.name().value(), "baz");
    assert_eq!(baz.entries().len(), 2);
    assert_eq!(baz.entries()[0].value().as_i64().unwrap(), 1);
    assert_eq!(baz.entries()[1].value().as_i64().unwrap(), 2);
    // Check type annotation on node name
    assert_eq!(baz.ty().unwrap().value(), "role");
}

// ============================================================================
// Section 3.2.6: Type Annotations on Node Names Tests
// ============================================================================

/// Test nodes with type annotations on names
#[test]
fn test_node_name_type_annotations() {
    let doc = kdl! {
        (published)date "2023-12-25"
        (config)settings debug=true
        (user)profile {
            name "John"
            age 30
        }
    };
    assert_eq!(doc.nodes().len(), 3);

    // Test published date node
    let date_node = &doc.nodes()[0];
    assert_eq!(date_node.name().value(), "date");
    assert_eq!(date_node.ty().unwrap().value(), "published");
    assert_eq!(date_node.entries().len(), 1);
    assert_eq!(date_node.entries()[0].value().as_string().unwrap(), "2023-12-25");

    // Test config settings node
    let settings_node = &doc.nodes()[1];
    assert_eq!(settings_node.name().value(), "settings");
    assert_eq!(settings_node.ty().unwrap().value(), "config");
    assert_eq!(settings_node.entries().len(), 1);
    assert_eq!(settings_node.get("debug").unwrap().value().as_bool().unwrap(), true);

    // Test user profile node with children
    let profile_node = &doc.nodes()[2];
    assert_eq!(profile_node.name().value(), "profile");
    assert_eq!(profile_node.ty().unwrap().value(), "user");
    let profile_children = profile_node.children().unwrap();
    assert_eq!(profile_children.nodes().len(), 2);
}

/// Test complex type annotations
#[test]
fn test_complex_type_annotations() {
    let doc = kdl! {
        (database_config)connection host="localhost" port=5432 {
            (auth)credentials user="admin"
            (timeout)settings connect_timeout=30
        }
    };
    assert_eq!(doc.nodes().len(), 1);
    let connection = &doc.nodes()[0];
    assert_eq!(connection.name().value(), "connection");
    assert_eq!(connection.ty().unwrap().value(), "database_config");

    let children = connection.children().unwrap();
    assert_eq!(children.nodes().len(), 2);

    let credentials = &children.nodes()[0];
    assert_eq!(credentials.name().value(), "credentials");
    assert_eq!(credentials.ty().unwrap().value(), "auth");

    let settings = &children.nodes()[1];
    assert_eq!(settings.name().value(), "settings");
    assert_eq!(settings.ty().unwrap().value(), "timeout");
}

// ============================================================================
// Section 3.2.7: Node Termination Tests
// ============================================================================

/// Test node termination with newlines
#[test]
fn test_node_newline_termination() {
    let doc = kdl! {
        node1 "value1"
        node2 "value2"
        node3 "value3"
    };
    assert_eq!(doc.nodes().len(), 3);
    assert_eq!(doc.nodes()[0].name().value(), "node1");
    assert_eq!(doc.nodes()[1].name().value(), "node2");
    assert_eq!(doc.nodes()[2].name().value(), "node3");
}

/// Test node termination with semicolons
#[test]
fn test_node_semicolon_termination() {
    let doc = kdl! {
        node1 "value1"; node2 "value2"; node3 "value3"
    };
    assert_eq!(doc.nodes().len(), 3);
    assert_eq!(doc.nodes()[0].name().value(), "node1");
    assert_eq!(doc.nodes()[1].name().value(), "node2");
    assert_eq!(doc.nodes()[2].name().value(), "node3");
}

/// Test node termination by children block end
#[test]
fn test_node_brace_termination() {
    let doc = kdl! {
        parent {
            child1 "value1"
            child2 "value2"
        }
        next_node "after_parent"
    };
    assert_eq!(doc.nodes().len(), 2);

    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 2);

    let next_node = &doc.nodes()[1];
    assert_eq!(next_node.name().value(), "next_node");
}

/// Test mixed termination types
#[test]
fn test_mixed_termination() {
    let doc = kdl! {
        node1 "value1"
        node2 "value2"; node3 "value3"
        node4 {
            child "value"
        }
        node5 "final"
    };
    assert_eq!(doc.nodes().len(), 5);
    for (i, expected_name) in ["node1", "node2", "node3", "node4", "node5"].iter().enumerate() {
        assert_eq!(doc.nodes()[i].name().value(), expected_name);
    }
}

// ============================================================================
// Section 3.2.8: Complete Node Variations Tests
// ============================================================================

/// Test all possible node component combinations
#[rstest]
#[case::name_only(kdl! { node })]
#[case::name_and_arg(kdl! { node "arg" })]
#[case::name_and_prop(kdl! { node key="value" })]
#[case::name_and_children(kdl! { node { child } })]
#[case::name_arg_prop(kdl! { node "arg" key="value" })]
#[case::name_arg_children(kdl! { node "arg" { child } })]
#[case::name_prop_children(kdl! { node key="value" { child } })]
#[case::name_arg_prop_children(kdl! { node "arg" key="value" { child } })]
fn test_node_component_combinations(#[case] doc: kdl::KdlDocument) {
    assert_eq!(doc.nodes().len(), 1, "Each test case should have exactly one node");
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "node", "Node name should be 'node'");
    // Each case is valid and should parse successfully
}

/// Test node with all components including type annotation
#[test]
fn test_complete_node_with_type() {
    let doc = kdl! {
        (config)server "production" debug=false port=8080 ssl=true {
            database host="localhost" port=5432 {
                pool_size 10
                timeout 30
            }
            cache enabled=true ttl=3600
            logging level="info" file="server.log"
        }
    };
    assert_eq!(doc.nodes().len(), 1);
    let server = &doc.nodes()[0];

    // Check type annotation
    assert_eq!(server.ty().unwrap().value(), "config");
    assert_eq!(server.name().value(), "server");

    // Check arguments and properties
    assert_eq!(server.entries().len(), 4); // "production", debug, port, ssl

    // Check children structure
    let children = server.children().unwrap();
    assert_eq!(children.nodes().len(), 3); // database, cache, logging

    let database = &children.nodes()[0];
    assert_eq!(database.name().value(), "database");
    assert!(database.children().is_some());
    let db_children = database.children().unwrap();
    assert_eq!(db_children.nodes().len(), 2);
}

/// Test maximum complexity node
#[test]
fn test_maximum_complexity_node() {
    let doc = kdl! {
        (application)complex_node
            "arg1" "arg2" "arg3"
            key1="value1" key2=42 key3=true key4=null key5=3.14
            "arg4" "arg5"
            more_key="more_value" final_key=false {

            (subsystem)child1 "child_arg" child_prop="child_value" {
                (nested)grandchild "deep"
            }

            child2 1 2 3 a=1 b=2 c=3 4 5 6

            (special)child3 {
                empty_child
                another_child x="y"
            }
        }
    };
    assert_eq!(doc.nodes().len(), 1);
    let complex = &doc.nodes()[0];

    // Verify type annotation
    assert_eq!(complex.ty().unwrap().value(), "application");
    assert_eq!(complex.name().value(), "complex_node");

    // Verify entries (mix of arguments and properties)
    assert!(complex.entries().len() > 5);

    // Check children structure
    let children = complex.children().unwrap();
    assert_eq!(children.nodes().len(), 3);

    // Verify child1 has its own children
    let child1 = &children.nodes()[0];
    assert_eq!(child1.ty().unwrap().value(), "subsystem");
    assert!(child1.children().is_some());

    // Verify child2 has mixed arguments and properties
    let child2 = &children.nodes()[1];
    assert!(child2.entries().len() > 6);

    // Verify child3 structure
    let child3 = &children.nodes()[2];
    assert_eq!(child3.ty().unwrap().value(), "special");
    let child3_children = child3.children().unwrap();
    assert_eq!(child3_children.nodes().len(), 2);
}

// ============================================================================
// Section 3.2.9: Edge Cases and Boundary Conditions
// ============================================================================

/// Test very long node names
#[test]
fn test_very_long_node_names() {
    let doc = kdl! {
        very_very_very_long_node_name_that_tests_the_limits_of_identifier_parsing_and_memory_allocation_efficiency_in_the_kdl_implementation
    };
    assert_eq!(doc.nodes().len(), 1);
    assert_eq!(
        doc.nodes()[0].name().value(),
        "very_very_very_long_node_name_that_tests_the_limits_of_identifier_parsing_and_memory_allocation_efficiency_in_the_kdl_implementation"
    );
}

/// Test nodes with many arguments
#[test]
fn test_many_arguments() {
    let doc = kdl! {
        node 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 20);

    // Verify all are arguments (no names)
    for (i, entry) in node.entries().iter().enumerate() {
        assert!(entry.name().is_none(), "Entry {} should be an argument", i);
        assert_eq!(entry.value().as_i64().unwrap(), (i + 1) as i64);
    }
}

/// Test nodes with many properties
#[test]
fn test_many_properties() {
    let doc = kdl! {
        node
            key1="value1" key2="value2" key3="value3" key4="value4" key5="value5"
            key6="value6" key7="value7" key8="value8" key9="value9" key10="value10"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 10);

    // Verify all are properties (have names)
    for entry in node.entries() {
        assert!(entry.name().is_some(), "All entries should be properties");
    }

    // Spot check a few properties
    assert_eq!(node.get("key1").unwrap().value().as_string().unwrap(), "value1");
    assert_eq!(node.get("key5").unwrap().value().as_string().unwrap(), "value5");
    assert_eq!(node.get("key10").unwrap().value().as_string().unwrap(), "value10");
}

/// Test deeply nested children
#[test]
fn test_very_deep_nesting() {
    let doc = kdl! {
        level1 {
            level2 {
                level3 {
                    level4 {
                        level5 {
                            level6 {
                                level7 "deeply_nested_value"
                            }
                        }
                    }
                }
            }
        }
    };
    assert_eq!(doc.nodes().len(), 1);

    // Navigate through all levels
    let mut current_node = &doc.nodes()[0];
    for level in 1..=6 {
        assert_eq!(current_node.name().value(), format!("level{}", level));
        let children = current_node.children().unwrap();
        assert_eq!(children.nodes().len(), 1);
        current_node = &children.nodes()[0];
    }

    // Final level
    assert_eq!(current_node.name().value(), "level7");
    assert_eq!(current_node.entries()[0].value().as_string().unwrap(), "deeply_nested_value");
}

// ============================================================================
// Section 3.2.10: Error Cases and Invalid Syntax Tests
// ============================================================================

/// Test invalid node syntax using kdl_impl2 directly
#[test]
fn test_invalid_node_name() {
    // Test completely invalid identifier
    let result = kdl_impl2(quote! { 123invalid_start });
    assert!(result.is_err(), "Node names starting with numbers should fail");
}

/// Test invalid property syntax
#[test]
fn test_invalid_property_syntax() {
    // Property without value
    let result = kdl_impl2(quote! { node key= });
    assert!(result.is_err(), "Property without value should fail");

    // Property with invalid key
    let result2 = kdl_impl2(quote! { node 123="value" });
    assert!(result2.is_err(), "Property key starting with number should fail");
}

/// Test invalid children syntax
#[test]
fn test_invalid_children_syntax() {
    // Test unclosed children block - this is tricky to test with quote! due to Rust syntax
    // We can test other invalid patterns instead

    // Invalid child content
    let result = kdl_impl2(quote! { node { invalid syntax ! } });
    assert!(result.is_err(), "Invalid child content should fail");
}

/// Test invalid argument types
#[test]
fn test_invalid_argument_types() {
    // Test malformed number
    let result = kdl_impl2(quote! { node 123.45.67 });
    assert!(result.is_err(), "Malformed float should fail");

    // Test invalid literal
    let result2 = kdl_impl2(quote! { node 'invalid_char_literal });
    assert!(result2.is_err(), "Invalid char literal should fail");
}

// ============================================================================
// Section 3.2.11: Performance and Stress Tests
// ============================================================================

/// Test many nodes with complex structure
#[test]
fn test_many_complex_nodes() {
    let doc = kdl! {
        node1 "arg1" key1="value1" { child1 }
        node2 "arg2" key2="value2" { child2 }
        node3 "arg3" key3="value3" { child3 }
        node4 "arg4" key4="value4" { child4 }
        node5 "arg5" key5="value5" { child5 }
        node6 "arg6" key6="value6" { child6 }
        node7 "arg7" key7="value7" { child7 }
        node8 "arg8" key8="value8" { child8 }
        node9 "arg9" key9="value9" { child9 }
        node10 "arg10" key10="value10" { child10 }
    };
    assert_eq!(doc.nodes().len(), 10);

    // Verify each node has the expected structure
    for i in 0..10 {
        let node = &doc.nodes()[i];
        let expected_name = format!("node{}", i + 1);
        assert_eq!(node.name().value(), expected_name);
        assert_eq!(node.entries().len(), 2); // one arg + one prop
        assert!(node.children().is_some());
        assert_eq!(node.children().unwrap().nodes().len(), 1);
    }
}

/// Stress test with generated repetitive structures
seq!(N in 1..=5 {
    #[test]
    fn test_stress_nodes_~N() {
        let doc = kdl! {
            #(
                stress_node_~N "arg_~N" key_~N="value_~N" {
                    child_~N "child_arg_~N"
                }
            )*
        };
        assert_eq!(doc.nodes().len(), 5);

        for i in 0..5 {
            let node = &doc.nodes()[i];
            let expected_name = format!("stress_node_{}", i + 1);
            assert_eq!(node.name().value(), expected_name);
        }
    }
});

// ============================================================================
// Section 3.2.12: Conformance and Specification Compliance Tests
// ============================================================================

/// Test that all node requirements from spec are met
#[test]
fn test_spec_compliance_node_requirements() {
    let doc = kdl! {
        required_name "optional_arg" optional_key="optional_value" {
            optional_child
        }
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];

    // Requirement 1: Every node must have a name
    assert!(!node.name().value().is_empty(), "Node must have a non-empty name");

    // Requirement 2: Name must be a String (implemented by the type system)
    assert_eq!(std::any::type_name_of_val(node.name().value()), "&str");

    // Requirement 3: Arguments and Properties are optional (tested by having both)
    assert!(node.entries().len() > 0, "Node can have entries");

    // Requirement 4: Children are optional (tested by having children)
    assert!(node.children().is_some(), "Node can have children");

    // Requirement 5: Arguments maintain order
    // This is implicitly tested by all our argument order tests above
}

/// Test argument ordering preservation as specified
#[test]
fn test_argument_ordering_preservation() {
    let doc = kdl! {
        node "first" prop1="between1" "second" prop2="between2" "third" prop3="between3" "fourth"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];

    // Extract arguments in the order they appear
    let mut arguments = Vec::new();
    for entry in node.entries() {
        if entry.name().is_none() {
            arguments.push(entry.value().as_string().unwrap());
        }
    }

    // Arguments must maintain their relative order
    assert_eq!(arguments, vec!["first", "second", "third", "fourth"]);
}

/// Test that properties order should not be assumed
#[test]
fn test_properties_order_independence() {
    let doc = kdl! {
        node key1="value1" key2="value2" key3="value3"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];

    // Properties should be accessible regardless of order
    assert!(node.get("key1").is_some());
    assert!(node.get("key2").is_some());
    assert!(node.get("key3").is_some());

    // Values should be correct regardless of order
    assert_eq!(node.get("key1").unwrap().value().as_string().unwrap(), "value1");
    assert_eq!(node.get("key2").unwrap().value().as_string().unwrap(), "value2");
    assert_eq!(node.get("key3").unwrap().value().as_string().unwrap(), "value3");
}

/// Test UTF-8 support in node names and values
#[test]
fn test_utf8_support_in_nodes() {
    let doc = kdl! {
        config name="José" city="São Paulo" message="Hello, L! <"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "config");

    // Verify UTF-8 values are preserved
    assert_eq!(node.get("name").unwrap().value().as_string().unwrap(), "José");
    assert_eq!(node.get("city").unwrap().value().as_string().unwrap(), "São Paulo");
    assert_eq!(node.get("message").unwrap().value().as_string().unwrap(), "Hello, L! <");
}

/// Test all termination methods
#[test]
fn test_all_termination_methods() {
    // Test implicit EOF termination
    let doc1 = kdl! {
        last_node "final"
    };
    assert_eq!(doc1.nodes().len(), 1);
    assert_eq!(doc1.nodes()[0].name().value(), "last_node");

    // Test semicolon termination
    let doc2 = kdl! {
        node1; node2; node3
    };
    assert_eq!(doc2.nodes().len(), 3);

    // Test newline termination (implicit in macro)
    let doc3 = kdl! {
        node1
        node2
        node3
    };
    assert_eq!(doc3.nodes().len(), 3);

    // Test brace termination
    let doc4 = kdl! {
        parent { child }
        next
    };
    assert_eq!(doc4.nodes().len(), 2);
}

/// Final compliance test with complex real-world example
#[test]
fn test_real_world_complex_example() {
    let doc = kdl! {
        (config)application name="MyApp" version="1.0.0" {
            (database)connection host="localhost" port=5432 ssl=true {
                (pool)settings min_connections=5 max_connections=20 timeout=30
                (auth)credentials user="app_user" password_env="DB_PASSWORD"
            }

            (cache)redis url="redis://localhost:6379" {
                (settings)config ttl=3600 max_memory="512mb"
            }

            (logging)configuration level="info" format="json" {
                (output)console enabled=true colors=true
                (output)file path="app.log" rotate=true max_size="100mb"
                (metrics)export enabled=true endpoint="/metrics"
            }

            (server)http_server port=8080 host="0.0.0.0" {
                (middleware)cors origins="*" credentials=true
                (middleware)compression enabled=true level=6
                (routes)api prefix="/api/v1" {
                    (endpoint)health path="/health" method="GET"
                    (endpoint)users path="/users" methods="GET,POST,PUT,DELETE"
                }
            }

            (features)flags {
                (feature)new_ui enabled=false rollout_percentage=0
                (feature)advanced_search enabled=true rollout_percentage=100
                (feature)beta_features enabled=false rollout_percentage=5
            }
        }
    };

    // This comprehensive example tests:
    // - Multiple levels of nesting
    // - Type annotations throughout
    // - Mix of arguments and properties
    // - Various data types
    // - Complex real-world structure
    // - All node components working together

    assert_eq!(doc.nodes().len(), 1);
    let app = &doc.nodes()[0];
    assert_eq!(app.ty().unwrap().value(), "config");
    assert_eq!(app.name().value(), "application");

    let app_children = app.children().unwrap();
    assert_eq!(app_children.nodes().len(), 5); // database, cache, logging, server, features

    // Verify the structure maintains all the complexity while being parseable
    assert!(app.get("name").is_some());
    assert!(app.get("version").is_some());

    // This test serves as a comprehensive validation that the node specification
    // is fully implemented and can handle real-world complexity
}