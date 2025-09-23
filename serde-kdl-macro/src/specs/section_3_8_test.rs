//! Tests for KDL Type Annotation specification (Section 3.8)
//!
//! This module contains comprehensive tests for type annotation syntax,
//! covering all aspects of the KDL Type Annotation specification.

use crate::kdl;
use quote::quote;
use seq_macro::seq;

/// Test basic type annotation syntax with (type)value pattern
#[test]
fn test_basic_type_annotation_syntax() {
    let doc = kdl! {
        node (u8)123
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "node");
    assert_eq!(node.entries().len(), 1);

    let entry = &node.entries()[0];
    assert!(entry.value().as_i64().is_some());
    assert_eq!(entry.value().type_().map(|s| s.value()), Some("u8"));
}

/// Test type annotation with property values
#[test]
fn test_type_annotation_on_properties() {
    let doc = kdl! {
        node prop=(regex).*
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);

    let entry = &node.entries()[0];
    assert_eq!(entry.name().unwrap().value(), "prop");
    assert_eq!(entry.value().as_string(), Some(".*"));
    assert_eq!(entry.value().type_().map(|s| s.value()), Some("regex"));
}

/// Test type annotation with whitespace variations
#[test]
fn test_type_annotation_with_whitespace() {
    // Test whitespace after ( and before )
    let doc = kdl! {
        node ( u16 ) 42
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    let entry = &node.entries()[0];
    assert_eq!(entry.value().type_().map(|s| s.value()), Some("u16"));

    // Test whitespace between annotation and value
    let doc2 = kdl! {
        node (u32)    42
    };
    assert_eq!(doc2.nodes().len(), 1);
    let node2 = &doc2.nodes()[0];
    let entry2 = &node2.entries()[0];
    assert_eq!(entry2.value().type_().map(|s| s.value()), Some("u32"));
}

/// Test all reserved signed integer type annotations
#[test]
fn test_signed_integer_type_annotations() {
    seq!(N in 8, 16, 32, 64, 128 {
        let doc = kdl! {
            node (i~N)42
        };
        assert_eq!(doc.nodes().len(), 1);
        let node = &doc.nodes()[0];
        let entry = &node.entries()[0];
        assert_eq!(entry.value().type_().map(|s| s.value()), Some(concat!("i", N)));
    });
}

/// Test all reserved unsigned integer type annotations
#[test]
fn test_unsigned_integer_type_annotations() {
    seq!(N in 8, 16, 32, 64, 128 {
        let doc = kdl! {
            node (u~N)42
        };
        assert_eq!(doc.nodes().len(), 1);
        let node = &doc.nodes()[0];
        let entry = &node.entries()[0];
        assert_eq!(entry.value().type_().map(|s| s.value()), Some(concat!("u", N)));
    });
}

/// Test platform-dependent integer type annotations
#[test]
fn test_platform_dependent_integer_types() {
    let doc = kdl! {
        node (isize)42 (usize)24
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 2);

    let entry1 = &node.entries()[0];
    assert_eq!(entry1.value().type_().map(|s| s.value()), Some("isize"));

    let entry2 = &node.entries()[1];
    assert_eq!(entry2.value().type_().map(|s| s.value()), Some("usize"));
}

/// Test IEEE 754 floating point type annotations
#[test]
fn test_floating_point_type_annotations() {
    let doc = kdl! {
        node (f32)3.14 (f64)2.71828
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 2);

    let entry1 = &node.entries()[0];
    assert_eq!(entry1.value().type_().map(|s| s.value()), Some("f32"));

    let entry2 = &node.entries()[1];
    assert_eq!(entry2.value().type_().map(|s| s.value()), Some("f64"));
}

/// Test IEEE 754-2008 decimal floating point type annotations
#[test]
fn test_decimal_floating_point_type_annotations() {
    let doc = kdl! {
        node (decimal64)"123.456" (decimal128)"987.654321"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 2);

    let entry1 = &node.entries()[0];
    assert_eq!(entry1.value().type_().map(|s| s.value()), Some("decimal64"));

    let entry2 = &node.entries()[1];
    assert_eq!(entry2.value().type_().map(|s| s.value()), Some("decimal128"));
}

/// Test all reserved string type annotations
#[test]
fn test_string_type_annotations() {
    let string_types = [
        "date-time", "time", "date", "duration", "decimal", "currency",
        "country-2", "country-3", "country-subdivision", "email", "idn-email",
        "hostname", "idn-hostname", "ipv4", "ipv6", "url", "url-reference",
        "irl", "irl-reference", "url-template", "uuid", "regex", "base64"
    ];

    for type_name in string_types {
        let result = crate::kdl_impl2(quote! {
            node (#type_name)"test-value"
        });
        assert!(result.is_ok(), "Failed to parse type annotation: {}", type_name);
    }
}

/// Test specific string type annotation examples
#[test]
fn test_specific_string_type_examples() {
    // Date-time example
    let doc = kdl! {
        node (date-time)"2023-12-25T10:30:00Z"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    let entry = &node.entries()[0];
    assert_eq!(entry.value().type_().map(|s| s.value()), Some("date-time"));
    assert_eq!(entry.value().as_string(), Some("2023-12-25T10:30:00Z"));

    // UUID example
    let doc2 = kdl! {
        node (uuid)"550e8400-e29b-41d4-a716-446655440000"
    };
    let node2 = &doc2.nodes()[0];
    let entry2 = &node2.entries()[0];
    assert_eq!(entry2.value().type_().map(|s| s.value()), Some("uuid"));

    // Base64 example
    let doc3 = kdl! {
        node (base64)"SGVsbG8gV29ybGQ="
    };
    let node3 = &doc3.nodes()[0];
    let entry3 = &node3.entries()[0];
    assert_eq!(entry3.value().type_().map(|s| s.value()), Some("base64"));
}

/// Test type annotations on all value types
#[test]
fn test_type_annotation_on_all_value_types() {
    // String value
    let doc = kdl! {
        node (string)"hello"
    };
    let node = &doc.nodes()[0];
    let entry = &node.entries()[0];
    assert_eq!(entry.value().type_().map(|s| s.value()), Some("string"));
    assert_eq!(entry.value().as_string(), Some("hello"));

    // Integer value
    let doc2 = kdl! {
        node (int)42
    };
    let node2 = &doc2.nodes()[0];
    let entry2 = &node2.entries()[0];
    assert_eq!(entry2.value().type_().map(|s| s.value()), Some("int"));
    assert!(entry2.value().as_i64().is_some());

    // Float value
    let doc3 = kdl! {
        node (float)3.14
    };
    let node3 = &doc3.nodes()[0];
    let entry3 = &node3.entries()[0];
    assert_eq!(entry3.value().type_().map(|s| s.value()), Some("float"));
    assert!(entry3.value().as_f64().is_some());

    // Boolean value
    let doc4 = kdl! {
        node (bool)true
    };
    let node4 = &doc4.nodes()[0];
    let entry4 = &node4.entries()[0];
    assert_eq!(entry4.value().type_().map(|s| s.value()), Some("bool"));
    assert_eq!(entry4.value().as_bool(), Some(true));

    // Null value
    let doc5 = kdl! {
        node (null)null
    };
    let node5 = &doc5.nodes()[0];
    let entry5 = &node5.entries()[0];
    assert_eq!(entry5.value().type_().map(|s| s.value()), Some("null"));
    assert!(entry5.value().as_null());
}

/// Test type annotations from spec examples
#[test]
fn test_spec_examples() {
    // Example: node (u8)123
    let doc1 = kdl! {
        node (u8)123
    };
    assert_eq!(doc1.nodes().len(), 1);
    let node1 = &doc1.nodes()[0];
    assert_eq!(node1.name().value(), "node");
    let entry1 = &node1.entries()[0];
    assert_eq!(entry1.value().type_().map(|s| s.value()), Some("u8"));
    assert_eq!(entry1.value().as_i64(), Some(123));

    // Example: node prop=(regex).*
    let doc2 = kdl! {
        node prop=(regex).*
    };
    let node2 = &doc2.nodes()[0];
    let entry2 = &node2.entries()[0];
    assert_eq!(entry2.name().unwrap().value(), "prop");
    assert_eq!(entry2.value().type_().map(|s| s.value()), Some("regex"));
    assert_eq!(entry2.value().as_string(), Some(".*"));
}

/// Test node name type annotations
#[test]
fn test_node_name_type_annotations() {
    // Example: (published)date "1970-01-01"
    let doc = kdl! {
        (published)date "1970-01-01"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "date");
    assert_eq!(node.name().type_().map(|s| s.value()), Some("published"));
    assert_eq!(node.entries().len(), 1);
    assert_eq!(node.entries()[0].value().as_string(), Some("1970-01-01"));

    // Example: (contributor)person name="Foo McBar"
    let doc2 = kdl! {
        (contributor)person name="Foo McBar"
    };
    let node2 = &doc2.nodes()[0];
    assert_eq!(node2.name().value(), "person");
    assert_eq!(node2.name().type_().map(|s| s.value()), Some("contributor"));
    assert_eq!(node2.entries().len(), 1);
    let entry = &node2.entries()[0];
    assert_eq!(entry.name().unwrap().value(), "name");
    assert_eq!(entry.value().as_string(), Some("Foo McBar"));
}

/// Test custom (non-reserved) type annotations
#[test]
fn test_custom_type_annotations() {
    let doc = kdl! {
        node (custom-type)"value" (my_type)42 (namespace:type)"data"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    let entry1 = &node.entries()[0];
    assert_eq!(entry1.value().type_().map(|s| s.value()), Some("custom-type"));

    let entry2 = &node.entries()[1];
    assert_eq!(entry2.value().type_().map(|s| s.value()), Some("my_type"));

    let entry3 = &node.entries()[2];
    assert_eq!(entry3.value().type_().map(|s| s.value()), Some("namespace:type"));
}

/// Test nested type annotations
#[test]
fn test_nested_type_annotations() {
    let doc = kdl! {
        parent {
            (typed)child (u32)42
            another prop=(string)"value"
        }
    };
    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 2);

    let child1 = &children.nodes()[0];
    assert_eq!(child1.name().value(), "child");
    assert_eq!(child1.name().type_().map(|s| s.value()), Some("typed"));
    let entry1 = &child1.entries()[0];
    assert_eq!(entry1.value().type_().map(|s| s.value()), Some("u32"));

    let child2 = &children.nodes()[1];
    let entry2 = &child2.entries()[0];
    assert_eq!(entry2.value().type_().map(|s| s.value()), Some("string"));
}

/// Test multiple type annotations in single node
#[test]
fn test_multiple_type_annotations() {
    let doc = kdl! {
        node (u8)1 (u16)2 (u32)3 prop1=(string)"a" prop2=(regex)".*"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 5);

    // Check arguments with type annotations
    let arg1 = &node.entries()[0];
    assert_eq!(arg1.value().type_().map(|s| s.value()), Some("u8"));
    assert_eq!(arg1.value().as_i64(), Some(1));

    let arg2 = &node.entries()[1];
    assert_eq!(arg2.value().type_().map(|s| s.value()), Some("u16"));
    assert_eq!(arg2.value().as_i64(), Some(2));

    let arg3 = &node.entries()[2];
    assert_eq!(arg3.value().type_().map(|s| s.value()), Some("u32"));
    assert_eq!(arg3.value().as_i64(), Some(3));

    // Check properties with type annotations
    let prop1 = &node.entries()[3];
    assert_eq!(prop1.name().unwrap().value(), "prop1");
    assert_eq!(prop1.value().type_().map(|s| s.value()), Some("string"));

    let prop2 = &node.entries()[4];
    assert_eq!(prop2.name().unwrap().value(), "prop2");
    assert_eq!(prop2.value().type_().map(|s| s.value()), Some("regex"));
}

/// Test edge cases for type annotation parsing
#[test]
fn test_type_annotation_edge_cases() {
    // Single character type
    let doc1 = kdl! {
        node (a)42
    };
    let node1 = &doc1.nodes()[0];
    let entry1 = &node1.entries()[0];
    assert_eq!(entry1.value().type_().map(|s| s.value()), Some("a"));

    // Long type name
    let doc2 = kdl! {
        node (very-long-type-name-with-dashes)42
    };
    let node2 = &doc2.nodes()[0];
    let entry2 = &node2.entries()[0];
    assert_eq!(entry2.value().type_().map(|s| s.value()), Some("very-long-type-name-with-dashes"));

    // Type with numbers
    let doc3 = kdl! {
        node (type123)"value"
    };
    let node3 = &doc3.nodes()[0];
    let entry3 = &node3.entries()[0];
    assert_eq!(entry3.value().type_().map(|s| s.value()), Some("type123"));
}

/// Test that type annotations are optional and don't affect parsing without them
#[test]
fn test_values_without_type_annotations() {
    let doc = kdl! {
        node 42 "string" true null prop="value"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 5);

    // All values should have no type annotation
    for entry in node.entries() {
        assert!(entry.value().type_().is_none());
    }

    // But values should parse correctly
    assert_eq!(node.entries()[0].value().as_i64(), Some(42));
    assert_eq!(node.entries()[1].value().as_string(), Some("string"));
    assert_eq!(node.entries()[2].value().as_bool(), Some(true));
    assert!(node.entries()[3].value().as_null());
    assert_eq!(node.entries()[4].name().unwrap().value(), "prop");
    assert_eq!(node.entries()[4].value().as_string(), Some("value"));
}

/// Test type annotations with negative numbers
#[test]
fn test_type_annotation_with_negative_numbers() {
    let doc = kdl! {
        node (i32)-42 (f64)-3.14
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 2);

    let entry1 = &node.entries()[0];
    assert_eq!(entry1.value().type_().map(|s| s.value()), Some("i32"));
    assert_eq!(entry1.value().as_i64(), Some(-42));

    let entry2 = &node.entries()[1];
    assert_eq!(entry2.value().type_().map(|s| s.value()), Some("f64"));
    assert_eq!(entry2.value().as_f64(), Some(-3.14));
}

/// Test error cases for invalid type annotation syntax
#[test]
fn test_invalid_type_annotation_syntax() {
    // Missing closing parenthesis
    let result1 = crate::kdl_impl2(quote! {
        node (type value
    });
    assert!(result1.is_err());

    // Missing opening parenthesis
    let result2 = crate::kdl_impl2(quote! {
        node type)value
    });
    assert!(result2.is_err());

    // Empty type annotation
    let result3 = crate::kdl_impl2(quote! {
        node ()value
    });
    assert!(result3.is_err());

    // Type annotation with no value
    let result4 = crate::kdl_impl2(quote! {
        node (type)
    });
    assert!(result4.is_err());
}

/// Test that implementations are free to ignore type annotations
#[test]
fn test_type_annotations_are_suggestions() {
    // Test that values with type annotations still work like regular values
    let doc = kdl! {
        node (string)42 (integer)"hello" (bool)null
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    // The underlying values should be accessible regardless of type annotation
    let entry1 = &node.entries()[0];
    assert_eq!(entry1.value().type_().map(|s| s.value()), Some("string"));
    assert_eq!(entry1.value().as_i64(), Some(42)); // Still an integer

    let entry2 = &node.entries()[1];
    assert_eq!(entry2.value().type_().map(|s| s.value()), Some("integer"));
    assert_eq!(entry2.value().as_string(), Some("hello")); // Still a string

    let entry3 = &node.entries()[2];
    assert_eq!(entry3.value().type_().map(|s| s.value()), Some("bool"));
    assert!(entry3.value().as_null()); // Still null
}

/// Test complex real-world scenarios with type annotations
#[test]
fn test_complex_real_world_scenarios() {
    let doc = kdl! {
        (config)database {
            host (hostname)"localhost"
            port (u16)5432
            username (string)"postgres"
            password (base64)"cGFzc3dvcmQ="
            ssl_mode (bool)true
            timeout (duration)"30s"
            connection_pool {
                min_size (u32)5
                max_size (u32)50
                idle_timeout (duration)"600s"
            }
        }
        (api)server {
            bind_address (ipv4)"0.0.0.0"
            port (u16)8080
            workers (usize)4
            (cors)allowed_origins (url)"https://example.com" (url)"https://api.example.com"
        }
        (logging)log_config {
            level (string)"INFO"
            format (string)"json"
            output (string)"stdout"
            (timestamp)created_at (date-time)"2023-12-25T10:30:00Z"
        }
    };

    assert_eq!(doc.nodes().len(), 3);

    // Check database config
    let db_node = &doc.nodes()[0];
    assert_eq!(db_node.name().value(), "database");
    assert_eq!(db_node.name().type_().map(|s| s.value()), Some("config"));

    let db_children = db_node.children().unwrap();
    assert_eq!(db_children.nodes().len(), 7);

    // Check specific typed values
    let host_node = db_children.nodes().iter().find(|n| n.name().value() == "host").unwrap();
    let host_entry = &host_node.entries()[0];
    assert_eq!(host_entry.value().type_().map(|s| s.value()), Some("hostname"));
    assert_eq!(host_entry.value().as_string(), Some("localhost"));

    let port_node = db_children.nodes().iter().find(|n| n.name().value() == "port").unwrap();
    let port_entry = &port_node.entries()[0];
    assert_eq!(port_entry.value().type_().map(|s| s.value()), Some("u16"));
    assert_eq!(port_entry.value().as_i64(), Some(5432));

    // Check nested pool config
    let pool_node = db_children.nodes().iter().find(|n| n.name().value() == "connection_pool").unwrap();
    let pool_children = pool_node.children().unwrap();
    assert_eq!(pool_children.nodes().len(), 3);

    // Check API server config
    let api_node = &doc.nodes()[1];
    assert_eq!(api_node.name().value(), "server");
    assert_eq!(api_node.name().type_().map(|s| s.value()), Some("api"));

    let api_children = api_node.children().unwrap();
    let cors_node = api_children.nodes().iter().find(|n| n.name().value() == "allowed_origins").unwrap();
    assert_eq!(cors_node.name().type_().map(|s| s.value()), Some("cors"));
    assert_eq!(cors_node.entries().len(), 2);

    for entry in cors_node.entries() {
        assert_eq!(entry.value().type_().map(|s| s.value()), Some("url"));
    }

    // Check logging config
    let log_node = &doc.nodes()[2];
    assert_eq!(log_node.name().value(), "log_config");
    assert_eq!(log_node.name().type_().map(|s| s.value()), Some("logging"));

    let log_children = log_node.children().unwrap();
    let timestamp_node = log_children.nodes().iter().find(|n| n.name().value() == "created_at").unwrap();
    assert_eq!(timestamp_node.name().type_().map(|s| s.value()), Some("timestamp"));
    let timestamp_entry = &timestamp_node.entries()[0];
    assert_eq!(timestamp_entry.value().type_().map(|s| s.value()), Some("date-time"));
    assert_eq!(timestamp_entry.value().as_string(), Some("2023-12-25T10:30:00Z"));
}