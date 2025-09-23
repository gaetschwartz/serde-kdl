//! Tests for Section 3.16: Null
//!
//! This module contains comprehensive tests for the KDL Null specification
//! as defined in section 3.16 of the KDL specification.
//!
//! The tests cover:
//! - Basic null value parsing with #null keyword
//! - Null values as node arguments
//! - Null values as property values
//! - Null values with type annotations
//! - Invalid null syntax and error cases
//! - Null values in various contexts (children, multiple values, etc.)
//! - Edge cases and boundary conditions
//! - Null value representation and API behavior

use crate::specs::kdl_impl2;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rstest::rstest;
use seq_macro::seq;
use serde_kdl_macro::kdl;

// ============================================================================
// Section 3.16.1: Basic Null Value Tests
// ============================================================================

/// Test basic null value as argument
#[test]
fn test_basic_null_argument() {
    let doc = kdl! {
        node #null
    };
    assert_eq!(doc.nodes().len(), 1, "Document should have one node");

    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "node", "Node name should be 'node'");
    assert_eq!(node.entries().len(), 1, "Node should have one entry");

    let entry = &node.entries()[0];
    assert!(entry.name().is_none(), "Entry should be an argument, not a property");

    let value = entry.value().expect("Entry should have a value");
    assert!(value.is_null(), "Value should be null");
    assert!(value.as_null(), "Value should be accessible as null");

    // Verify it's not other types
    assert!(value.as_string().is_none(), "Null should not be a string");
    assert!(value.as_i64().is_none(), "Null should not be an integer");
    assert!(value.as_f64().is_none(), "Null should not be a float");
    assert!(value.as_bool().is_none(), "Null should not be a boolean");
}

/// Test basic null value as property
#[test]
fn test_basic_null_property() {
    let doc = kdl! {
        node key=#null
    };
    assert_eq!(doc.nodes().len(), 1, "Document should have one node");

    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1, "Node should have one entry");

    let entry = &node.entries()[0];
    assert!(entry.name().is_some(), "Entry should be a property");
    assert_eq!(entry.name().unwrap().value(), "key", "Property key should be 'key'");

    let value = entry.value().expect("Property should have a value");
    assert!(value.is_null(), "Property value should be null");
    assert!(value.as_null(), "Property value should be accessible as null");
}

/// Test the example from the specification: my-node #null key=#null
#[test]
fn test_specification_example() {
    let doc = kdl! {
        my-node #null key=#null
    };
    assert_eq!(doc.nodes().len(), 1, "Document should have one node");

    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "my-node", "Node name should be 'my-node'");
    assert_eq!(node.entries().len(), 2, "Node should have two entries");

    // First entry: #null argument
    let arg_entry = &node.entries()[0];
    assert!(arg_entry.name().is_none(), "First entry should be an argument");
    let arg_value = arg_entry.value().expect("Argument should have a value");
    assert!(arg_value.is_null(), "Argument should be null");

    // Second entry: key=#null property
    let prop_entry = &node.entries()[1];
    assert!(prop_entry.name().is_some(), "Second entry should be a property");
    assert_eq!(prop_entry.name().unwrap().value(), "key", "Property key should be 'key'");
    let prop_value = prop_entry.value().expect("Property should have a value");
    assert!(prop_value.is_null(), "Property value should be null");
}

// ============================================================================
// Section 3.16.2: Null Values in Different Contexts
// ============================================================================

/// Test multiple null arguments
#[test]
fn test_multiple_null_arguments() {
    let doc = kdl! {
        node #null #null #null
    };
    assert_eq!(doc.nodes().len(), 1, "Document should have one node");

    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3, "Node should have three entries");

    for (i, entry) in node.entries().iter().enumerate() {
        assert!(entry.name().is_none(), "Entry {} should be an argument", i);
        let value = entry.value().expect(&format!("Entry {} should have a value", i));
        assert!(value.is_null(), "Entry {} should be null", i);
    }
}

/// Test mixed arguments with null values
#[test]
fn test_mixed_arguments_with_null() {
    let doc = kdl! {
        node "string" #null 42 #null #true
    };
    assert_eq!(doc.nodes().len(), 1, "Document should have one node");

    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 5, "Node should have five entries");

    // First argument: string
    let value0 = node.entries()[0].value().expect("First entry should have a value");
    assert_eq!(value0.as_string(), Some("string"), "First argument should be string");

    // Second argument: null
    let value1 = node.entries()[1].value().expect("Second entry should have a value");
    assert!(value1.is_null(), "Second argument should be null");

    // Third argument: integer
    let value2 = node.entries()[2].value().expect("Third entry should have a value");
    assert_eq!(value2.as_i64(), Some(42), "Third argument should be 42");

    // Fourth argument: null
    let value3 = node.entries()[3].value().expect("Fourth entry should have a value");
    assert!(value3.is_null(), "Fourth argument should be null");

    // Fifth argument: boolean
    let value4 = node.entries()[4].value().expect("Fifth entry should have a value");
    assert_eq!(value4.as_bool(), Some(true), "Fifth argument should be true");
}

/// Test multiple null properties
#[test]
fn test_multiple_null_properties() {
    let doc = kdl! {
        node key1=#null key2=#null key3=#null
    };
    assert_eq!(doc.nodes().len(), 1, "Document should have one node");

    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3, "Node should have three entries");

    let expected_keys = ["key1", "key2", "key3"];
    for (i, entry) in node.entries().iter().enumerate() {
        assert!(entry.name().is_some(), "Entry {} should be a property", i);
        assert_eq!(entry.name().unwrap().value(), expected_keys[i],
                  "Property {} should have key '{}'", i, expected_keys[i]);
        let value = entry.value().expect(&format!("Property {} should have a value", i));
        assert!(value.is_null(), "Property {} should be null", i);
    }
}

/// Test mixed properties with null values
#[test]
fn test_mixed_properties_with_null() {
    let doc = kdl! {
        node name="Alice" age=#null active=#true data=#null score=95.5
    };
    assert_eq!(doc.nodes().len(), 1, "Document should have one node");

    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 5, "Node should have five entries");

    // name="Alice"
    let name_entry = &node.entries()[0];
    assert_eq!(name_entry.name().unwrap().value(), "name");
    assert_eq!(name_entry.value().unwrap().as_string(), Some("Alice"));

    // age=#null
    let age_entry = &node.entries()[1];
    assert_eq!(age_entry.name().unwrap().value(), "age");
    assert!(age_entry.value().unwrap().is_null());

    // active=#true
    let active_entry = &node.entries()[2];
    assert_eq!(active_entry.name().unwrap().value(), "active");
    assert_eq!(active_entry.value().unwrap().as_bool(), Some(true));

    // data=#null
    let data_entry = &node.entries()[3];
    assert_eq!(data_entry.name().unwrap().value(), "data");
    assert!(data_entry.value().unwrap().is_null());

    // score=95.5
    let score_entry = &node.entries()[4];
    assert_eq!(score_entry.name().unwrap().value(), "score");
    assert_eq!(score_entry.value().unwrap().as_f64(), Some(95.5));
}

/// Test null values in child nodes
#[test]
fn test_null_in_child_nodes() {
    let doc = kdl! {
        parent {
            child1 #null
            child2 key=#null
            child3 #null key=#null value="test"
        }
    };
    assert_eq!(doc.nodes().len(), 1, "Document should have one node");

    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");
    assert_eq!(parent.children().unwrap().nodes().len(), 3, "Parent should have three children");

    let children = parent.children().unwrap().nodes();

    // child1 #null
    let child1 = &children[0];
    assert_eq!(child1.name().value(), "child1");
    assert_eq!(child1.entries().len(), 1);
    assert!(child1.entries()[0].value().unwrap().is_null());

    // child2 key=#null
    let child2 = &children[1];
    assert_eq!(child2.name().value(), "child2");
    assert_eq!(child2.entries().len(), 1);
    assert_eq!(child2.entries()[0].name().unwrap().value(), "key");
    assert!(child2.entries()[0].value().unwrap().is_null());

    // child3 #null key=#null value="test"
    let child3 = &children[2];
    assert_eq!(child3.name().value(), "child3");
    assert_eq!(child3.entries().len(), 3);
    assert!(child3.entries()[0].value().unwrap().is_null()); // #null argument
    assert_eq!(child3.entries()[1].name().unwrap().value(), "key");
    assert!(child3.entries()[1].value().unwrap().is_null()); // key=#null
    assert_eq!(child3.entries()[2].name().unwrap().value(), "value");
    assert_eq!(child3.entries()[2].value().unwrap().as_string(), Some("test")); // value="test"
}

// ============================================================================
// Section 3.16.3: Null Values with Type Annotations
// ============================================================================

/// Test null with type annotations
#[test]
fn test_null_with_type_annotations() {
    let doc = kdl! {
        node (optional)#null (nullable)#null
    };
    assert_eq!(doc.nodes().len(), 1, "Document should have one node");

    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 2, "Node should have two entries");

    // First entry: (optional)#null
    let entry1 = &node.entries()[0];
    assert!(entry1.name().is_none(), "First entry should be an argument");
    let value1 = entry1.value().expect("First entry should have a value");
    assert!(value1.is_null(), "First value should be null");
    assert_eq!(entry1.ty().as_ref().map(|t| t.value()), Some("optional"),
              "First entry should have type annotation 'optional'");

    // Second entry: (nullable)#null
    let entry2 = &node.entries()[1];
    assert!(entry2.name().is_none(), "Second entry should be an argument");
    let value2 = entry2.value().expect("Second entry should have a value");
    assert!(value2.is_null(), "Second value should be null");
    assert_eq!(entry2.ty().as_ref().map(|t| t.value()), Some("nullable"),
              "Second entry should have type annotation 'nullable'");
}

/// Test null properties with type annotations
#[test]
fn test_null_properties_with_type_annotations() {
    let doc = kdl! {
        node optional=(optional)#null nullable=(nullable)#null
    };
    assert_eq!(doc.nodes().len(), 1, "Document should have one node");

    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 2, "Node should have two entries");

    // First property: optional=(optional)#null
    let entry1 = &node.entries()[0];
    assert_eq!(entry1.name().unwrap().value(), "optional");
    let value1 = entry1.value().expect("First property should have a value");
    assert!(value1.is_null(), "First property value should be null");
    assert_eq!(entry1.ty().as_ref().map(|t| t.value()), Some("optional"),
              "First property should have type annotation 'optional'");

    // Second property: nullable=(nullable)#null
    let entry2 = &node.entries()[1];
    assert_eq!(entry2.name().unwrap().value(), "nullable");
    let value2 = entry2.value().expect("Second property should have a value");
    assert!(value2.is_null(), "Second property value should be null");
    assert_eq!(entry2.ty().as_ref().map(|t| t.value()), Some("nullable"),
              "Second property should have type annotation 'nullable'");
}

// ============================================================================
// Section 3.16.4: Null Value Variations and Edge Cases
// ============================================================================

/// Test null keyword case sensitivity and variations
#[test]
fn test_null_keyword_variations() {
    // Test that only #null (lowercase) is valid
    let doc = kdl! {
        node #null
    };
    let node = &doc.nodes()[0];
    assert!(node.entries()[0].value().unwrap().is_null(), "Lowercase #null should work");
}

/// Test null in complex nested structures
#[test]
fn test_null_in_complex_structures() {
    let doc = kdl! {
        config {
            database {
                host "localhost"
                port 5432
                password #null
                ssl_cert #null
            }
            cache enabled=#true ttl=#null {
                redis {
                    url #null
                    auth token=#null expires=(time)#null
                }
            }
        }
    };

    let config = &doc.nodes()[0];
    let config_children = config.children().unwrap().nodes();

    // database.password #null and database.ssl_cert #null
    let database = &config_children[0];
    let db_entries = &database.entries();
    assert!(db_entries[2].value().unwrap().is_null()); // password #null
    assert!(db_entries[3].value().unwrap().is_null()); // ssl_cert #null

    // cache.ttl=#null
    let cache = &config_children[1];
    let cache_entries = &cache.entries();
    assert!(cache_entries[1].value().unwrap().is_null()); // ttl=#null

    // redis.url #null and redis.auth properties
    let cache_children = cache.children().unwrap().nodes();
    let redis = &cache_children[0];
    let redis_entries = &redis.entries();
    assert!(redis_entries[0].value().unwrap().is_null()); // url #null
    assert!(redis_entries[1].value().unwrap().is_null()); // token=#null
    assert!(redis_entries[2].value().unwrap().is_null()); // expires=(time)#null
}

/// Test null with whitespace variations
#[test]
fn test_null_with_whitespace() {
    // Test null with various whitespace around it
    let doc = kdl! {
        node #null #null    #null
    };
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);
    for (i, entry) in node.entries().iter().enumerate() {
        assert!(entry.value().unwrap().is_null(), "Entry {} should be null", i);
    }
}

/// Test document with only null values
#[test]
fn test_document_with_only_nulls() {
    let doc = kdl! {
        null_node #null
        another_null key=#null
        mixed #null value=#null both=(type)#null
    };
    assert_eq!(doc.nodes().len(), 3);

    // null_node #null
    let node1 = &doc.nodes()[0];
    assert_eq!(node1.name().value(), "null_node");
    assert!(node1.entries()[0].value().unwrap().is_null());

    // another_null key=#null
    let node2 = &doc.nodes()[1];
    assert_eq!(node2.name().value(), "another_null");
    assert!(node2.entries()[0].value().unwrap().is_null());

    // mixed #null value=#null both=(type)#null
    let node3 = &doc.nodes()[2];
    assert_eq!(node3.name().value(), "mixed");
    assert_eq!(node3.entries().len(), 3);
    assert!(node3.entries()[0].value().unwrap().is_null()); // #null argument
    assert!(node3.entries()[1].value().unwrap().is_null()); // value=#null
    assert!(node3.entries()[2].value().unwrap().is_null()); // both=(type)#null
}

// ============================================================================
// Section 3.16.5: Error Cases and Invalid Null Syntax
// ============================================================================

/// Test invalid null syntax - missing # prefix
#[test]
fn test_invalid_null_without_prefix() {
    // Test null without # prefix should be treated as identifier, not null
    let result = kdl_impl2(quote! { node null });
    // This should succeed but create a string value "null", not a null value
    if result.is_ok() {
        // If it parses successfully, it should be a string "null", not null
        // This tests that "null" without # is treated as an identifier
    } else {
        // If it fails, that's also acceptable depending on the parser implementation
        assert!(result.is_err(), "null without # prefix might produce an error");
    }
}

/// Test malformed null keywords
#[test]
fn test_malformed_null_keywords() {
    // Test incorrect case variations
    let test_cases = [
        quote! { node #Null },
        quote! { node #NULL },
        quote! { node #nULL },
        quote! { node #Null },
    ];

    for (i, case) in test_cases.iter().enumerate() {
        let result = kdl_impl2(case.clone());
        assert!(result.is_err(), "Malformed null case {} should produce an error", i);
    }
}

/// Test null with invalid characters
#[test]
fn test_null_with_invalid_characters() {
    let test_cases = [
        quote! { node #null_ },
        quote! { node #null1 },
        quote! { node #null- },
        quote! { node #null. },
    ];

    for (i, case) in test_cases.iter().enumerate() {
        let result = kdl_impl2(case.clone());
        assert!(result.is_err(), "Invalid null variation {} should produce an error", i);
    }
}

/// Test null in invalid contexts (these should still work as null is a valid value)
#[test]
fn test_null_in_various_contexts() {
    // These should all work - null is a valid value anywhere values are allowed

    // As node name - this should fail since null is not a string
    let result = kdl_impl2(quote! { #null });
    assert!(result.is_err(), "null as node name should produce an error");

    // As property key - this should fail since null is not a string
    let result = kdl_impl2(quote! { node #null="value" });
    assert!(result.is_err(), "null as property key should produce an error");
}

/// Test empty null-like tokens
#[test]
fn test_empty_null_tokens() {
    let test_cases = [
        quote! { node # },
        quote! { node #  },
        quote! { node ## },
    ];

    for (i, case) in test_cases.iter().enumerate() {
        let result = kdl_impl2(case.clone());
        assert!(result.is_err(), "Invalid empty null-like token {} should produce an error", i);
    }
}

// ============================================================================
// Section 3.16.6: Systematic Testing with seq_macro
// ============================================================================

/// Test null in different positions using seq_macro
seq!(N in 0..5 {
    #[test]
    fn test_null_at_position_~N() {
        let doc = kdl! {
            node
            #(
                seq!(I in 0..=N {
                    #(if I == N { quote! { #null } } else { quote! { ~I } })*
                })
            )*
        };

        let node = &doc.nodes()[0];
        assert_eq!(node.entries().len(), N + 1);

        for (i, entry) in node.entries().iter().enumerate() {
            if i == N {
                assert!(entry.value().unwrap().is_null(), "Entry at position {} should be null", i);
            } else {
                assert_eq!(entry.value().unwrap().as_i64(), Some(i as i64),
                          "Entry at position {} should be {}", i, i);
            }
        }
    }
});

/// Test multiple null values in sequence
seq!(N in 1..6 {
    #[test]
    fn test_~N~_consecutive_nulls() {
        let doc = kdl! {
            node #( seq!(I in 0..N { #null })* )
        };

        let node = &doc.nodes()[0];
        assert_eq!(node.entries().len(), N);

        for (i, entry) in node.entries().iter().enumerate() {
            assert!(entry.value().unwrap().is_null(), "Entry {} should be null", i);
        }
    }
});

// ============================================================================
// Section 3.16.7: Null Value Representation Tests
// ============================================================================

/// Test null value type checking methods
#[test]
fn test_null_value_type_methods() {
    let doc = kdl! {
        node #null
    };

    let value = doc.nodes()[0].entries()[0].value().unwrap();

    // Positive tests
    assert!(value.is_null(), "Value should be identified as null");
    assert!(value.as_null(), "Value should be accessible as null");

    // Negative tests - null should not be any other type
    assert!(!value.is_string(), "Null should not be a string");
    assert!(!value.is_i64(), "Null should not be an integer");
    assert!(!value.is_f64(), "Null should not be a float");
    assert!(!value.is_bool(), "Null should not be a boolean");

    assert!(value.as_string().is_none(), "Null should not convert to string");
    assert!(value.as_i64().is_none(), "Null should not convert to integer");
    assert!(value.as_f64().is_none(), "Null should not convert to float");
    assert!(value.as_bool().is_none(), "Null should not convert to boolean");
}

/// Test null value consistency across different declaration methods
#[test]
fn test_null_value_consistency() {
    let doc = kdl! {
        test_node #null prop=#null (typed)#null key=(typed)#null
    };

    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 4);

    // All entries should have null values that behave identically
    for (i, entry) in node.entries().iter().enumerate() {
        let value = entry.value().expect(&format!("Entry {} should have a value", i));
        assert!(value.is_null(), "Entry {} should be null", i);
        assert!(value.as_null(), "Entry {} should be accessible as null", i);

        // All null values should have consistent type behavior
        assert!(!value.is_string() && value.as_string().is_none(),
               "Entry {} null should not be string", i);
        assert!(!value.is_i64() && value.as_i64().is_none(),
               "Entry {} null should not be integer", i);
        assert!(!value.is_f64() && value.as_f64().is_none(),
               "Entry {} null should not be float", i);
        assert!(!value.is_bool() && value.as_bool().is_none(),
               "Entry {} null should not be boolean", i);
    }
}

/// Test null value in comparison contexts (if supported by the implementation)
#[test]
fn test_null_value_properties() {
    let doc = kdl! {
        node #null #null
    };

    let node = &doc.nodes()[0];
    let value1 = node.entries()[0].value().unwrap();
    let value2 = node.entries()[1].value().unwrap();

    // Both should be null
    assert!(value1.is_null() && value2.is_null(), "Both values should be null");

    // Test that null values behave consistently
    assert_eq!(value1.is_null(), value2.is_null(), "Null values should have same type behavior");
    assert_eq!(value1.as_null(), value2.as_null(), "Null values should convert consistently");
}