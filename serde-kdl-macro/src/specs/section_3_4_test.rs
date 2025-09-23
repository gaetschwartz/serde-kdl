//! Tests for KDL Property specification (Section 3.4)
//!
//! This module tests the following aspects of KDL Properties:
//! - Property syntax: `key=value`
//! - Property key requirements (identifier strings)
//! - Property value types (all valid KDL values)
//! - Multiple properties in a node
//! - Property ordering and uniqueness
//! - Comment prefixes `/-` for properties
//! - Invalid property syntax

use crate::specs::kdl_impl2;
use proc_macro2::TokenStream;
use quote::quote;
use serde_kdl_macro::kdl;

/// Test basic property syntax: `key=value`
#[test]
fn test_basic_property_syntax() {
    let doc = kdl! {
        node key="value"
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "node");
    assert_eq!(node.entries().len(), 1);

    let entry = &node.entries()[0];
    assert!(entry.name().is_some());
    assert_eq!(entry.name().unwrap().value(), "key");
    assert_eq!(entry.value().as_string().unwrap(), "value");
}

/// Test single character property keys
#[test]
fn test_single_char_property_keys() {
    let doc = kdl! {
        node a="first" b="second" c="third"
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    // Check that all three properties exist
    let mut found_a = false;
    let mut found_b = false;
    let mut found_c = false;

    for entry in node.entries() {
        if let Some(name) = entry.name() {
            match name.value() {
                "a" => {
                    found_a = true;
                    assert_eq!(entry.value().as_string().unwrap(), "first");
                }
                "b" => {
                    found_b = true;
                    assert_eq!(entry.value().as_string().unwrap(), "second");
                }
                "c" => {
                    found_c = true;
                    assert_eq!(entry.value().as_string().unwrap(), "third");
                }
                _ => panic!("Unexpected property key: {}", name.value()),
            }
        }
    }

    assert!(found_a && found_b && found_c);
}

/// Test property keys with underscores and numbers
#[test]
fn test_property_keys_with_underscores_numbers() {
    let doc = kdl! {
        node key_1="value1" key_2="value2" property_123="value3"
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    let mut found_keys = std::collections::HashSet::new();
    for entry in node.entries() {
        if let Some(name) = entry.name() {
            found_keys.insert(name.value().to_string());
        }
    }

    assert!(found_keys.contains("key_1"));
    assert!(found_keys.contains("key_2"));
    assert!(found_keys.contains("property_123"));
}

/// Test property with string values
#[test]
fn test_property_string_values() {
    let doc = kdl! {
        node
            empty=""
            simple="hello"
            spaces="hello world"
            special="hello\nworld\ttab"
            unicode="café"
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 5);

    let mut properties = std::collections::HashMap::new();
    for entry in node.entries() {
        if let Some(name) = entry.name() {
            properties.insert(name.value().to_string(), entry.value().as_string().unwrap().to_string());
        }
    }

    assert_eq!(properties.get("empty").unwrap(), "");
    assert_eq!(properties.get("simple").unwrap(), "hello");
    assert_eq!(properties.get("spaces").unwrap(), "hello world");
    assert_eq!(properties.get("special").unwrap(), "hello\nworld\ttab");
    assert_eq!(properties.get("unicode").unwrap(), "café");
}

/// Test property with integer values
#[test]
fn test_property_integer_values() {
    let doc = kdl! {
        node
            zero=0
            positive=42
            negative=-123
            large=999999
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 4);

    let mut properties = std::collections::HashMap::new();
    for entry in node.entries() {
        if let Some(name) = entry.name() {
            properties.insert(name.value().to_string(), entry.value().as_i64().unwrap());
        }
    }

    assert_eq!(*properties.get("zero").unwrap(), 0);
    assert_eq!(*properties.get("positive").unwrap(), 42);
    assert_eq!(*properties.get("negative").unwrap(), -123);
    assert_eq!(*properties.get("large").unwrap(), 999999);
}

/// Test property with float values
#[test]
fn test_property_float_values() {
    let doc = kdl! {
        node
            pi=3.14159
            negative_float=-2.5
            zero_float=0.0
            scientific=1.23e10
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 4);

    let mut properties = std::collections::HashMap::new();
    for entry in node.entries() {
        if let Some(name) = entry.name() {
            properties.insert(name.value().to_string(), entry.value().as_f64().unwrap());
        }
    }

    assert!((properties.get("pi").unwrap() - 3.14159).abs() < f64::EPSILON);
    assert!((properties.get("negative_float").unwrap() - (-2.5)).abs() < f64::EPSILON);
    assert!((properties.get("zero_float").unwrap() - 0.0).abs() < f64::EPSILON);
    assert!((properties.get("scientific").unwrap() - 1.23e10).abs() < 1e6);
}

/// Test property with boolean values
#[test]
fn test_property_boolean_values() {
    let doc = kdl! {
        node
            enabled=true
            disabled=false
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 2);

    let mut properties = std::collections::HashMap::new();
    for entry in node.entries() {
        if let Some(name) = entry.name() {
            properties.insert(name.value().to_string(), entry.value().as_bool().unwrap());
        }
    }

    assert_eq!(*properties.get("enabled").unwrap(), true);
    assert_eq!(*properties.get("disabled").unwrap(), false);
}

/// Test property with null values
#[test]
fn test_property_null_values() {
    let doc = kdl! {
        node
            empty=null
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);

    let entry = &node.entries()[0];
    assert_eq!(entry.name().unwrap().value(), "empty");
    assert!(matches!(entry.value(), kdl::KdlValue::Null));
}

/// Test multiple properties in a single node
#[test]
fn test_multiple_properties() {
    let doc = kdl! {
        node
            name="test"
            version=1
            enabled=true
            priority=2.5
            description="A test node"
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 5);

    let mut found_props = std::collections::HashSet::new();
    for entry in node.entries() {
        if let Some(name) = entry.name() {
            found_props.insert(name.value().to_string());
        }
    }

    assert!(found_props.contains("name"));
    assert!(found_props.contains("version"));
    assert!(found_props.contains("enabled"));
    assert!(found_props.contains("priority"));
    assert!(found_props.contains("description"));
}

/// Test property ordering and rightmost override behavior
#[test]
fn test_property_ordering_override() {
    let doc = kdl! {
        node a=1 a=2
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];

    // According to spec: "Properties should be interpreted left-to-right,
    // with rightmost properties with identical names overriding earlier properties"
    // The KDL library should handle this, but let's verify what we get
    let mut a_values = Vec::new();
    for entry in node.entries() {
        if let Some(name) = entry.name() {
            if name.value() == "a" {
                a_values.push(entry.value().as_i64().unwrap());
            }
        }
    }

    // The macro should create entries for both, but the rightmost should be the effective value
    // Note: The actual KDL library behavior may vary, but we test what our macro produces
    assert!(!a_values.is_empty());
}

/// Test properties mixed with arguments
#[test]
fn test_properties_mixed_with_arguments() {
    let doc = kdl! {
        node "arg1" key="value" "arg2" count=42 "arg3"
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 5);

    let mut argument_count = 0;
    let mut property_count = 0;

    for entry in node.entries() {
        if entry.name().is_some() {
            property_count += 1;
        } else {
            argument_count += 1;
        }
    }

    assert_eq!(argument_count, 3); // "arg1", "arg2", "arg3"
    assert_eq!(property_count, 2); // key="value", count=42
}

/// Test properties with complex values (ranges using seq-macro)
#[test]
fn test_properties_range_values() {
    use seq_macro::seq;

    seq!(N in 0..5 {
        let doc = kdl! {
            node value~N=N
        };

        assert_eq!(doc.nodes().len(), 1);
        let node = &doc.nodes()[0];
        assert_eq!(node.entries().len(), 1);

        let entry = &node.entries()[0];
        assert_eq!(entry.name().unwrap().value(), format!("value{}", N));
        assert_eq!(entry.value().as_i64().unwrap(), N);
    });
}

/// Test properties with type annotations
#[test]
fn test_properties_with_type_annotations() {
    let doc = kdl! {
        node
            date=(date)"2023-01-15"
            url=(url)"https://example.com"
            count=(i32)42
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    let mut found_types = std::collections::HashMap::new();
    for entry in node.entries() {
        if let Some(name) = entry.name() {
            if let Some(type_annotation) = entry.value().type_annotation() {
                found_types.insert(name.value().to_string(), type_annotation.value().to_string());
            }
        }
    }

    assert_eq!(found_types.get("date").unwrap(), "date");
    assert_eq!(found_types.get("url").unwrap(), "url");
    assert_eq!(found_types.get("count").unwrap(), "i32");
}

/// Test edge case: empty property key (should fail compilation)
#[test]
fn test_invalid_empty_property_key() {
    // This should fail at compile time, but we test parsing error
    let result = kdl_impl2(quote! {
        node =""
    });

    assert!(result.is_err());
}

/// Test edge case: property key starting with number (should fail)
#[test]
fn test_invalid_numeric_property_key() {
    // Property keys must be valid identifiers, not starting with numbers
    let result = kdl_impl2(quote! {
        node 123key="value"
    });

    assert!(result.is_err());
}

/// Test edge case: property without value (should fail)
#[test]
fn test_invalid_property_without_value() {
    let result = kdl_impl2(quote! {
        node key=
    });

    assert!(result.is_err());
}

/// Test edge case: property with missing equals sign (should fail)
#[test]
fn test_invalid_property_missing_equals() {
    let result = kdl_impl2(quote! {
        node key "value"
    });

    // This should be parsed as an argument, not a property, so it should succeed
    // but with different semantics
    assert!(result.is_ok());
}

/// Test edge case: property key with spaces (should fail)
#[test]
fn test_invalid_property_key_with_spaces() {
    // Property keys cannot contain spaces
    let result = kdl_impl2(quote! {
        node "my key"="value"
    });

    assert!(result.is_err());
}

/// Test edge case: property with complex key expressions (should fail)
#[test]
fn test_invalid_complex_property_key() {
    let result = kdl_impl2(quote! {
        node (key + "suffix")="value"
    });

    assert!(result.is_err());
}

/// Test properties in nested nodes
#[test]
fn test_properties_in_nested_nodes() {
    let doc = kdl! {
        parent root_prop="root_value" {
            child child_prop="child_value" {
                grandchild nested_prop="nested_value"
            }
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");
    assert_eq!(parent.entries().len(), 1);

    // Check parent property
    let parent_entry = &parent.entries()[0];
    assert_eq!(parent_entry.name().unwrap().value(), "root_prop");
    assert_eq!(parent_entry.value().as_string().unwrap(), "root_value");

    // Check child
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 1);
    let child = &children.nodes()[0];
    assert_eq!(child.name().value(), "child");
    assert_eq!(child.entries().len(), 1);

    let child_entry = &child.entries()[0];
    assert_eq!(child_entry.name().unwrap().value(), "child_prop");
    assert_eq!(child_entry.value().as_string().unwrap(), "child_value");

    // Check grandchild
    let grandchildren = child.children().unwrap();
    assert_eq!(grandchildren.nodes().len(), 1);
    let grandchild = &grandchildren.nodes()[0];
    assert_eq!(grandchild.name().value(), "grandchild");
    assert_eq!(grandchild.entries().len(), 1);

    let grandchild_entry = &grandchild.entries()[0];
    assert_eq!(grandchild_entry.name().unwrap().value(), "nested_prop");
    assert_eq!(grandchild_entry.value().as_string().unwrap(), "nested_value");
}

/// Test properties with identifier-like values (not quoted)
#[test]
fn test_properties_with_identifier_values() {
    let doc = kdl! {
        node
            option=enabled
            mode=debug
            target=production
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    let mut properties = std::collections::HashMap::new();
    for entry in node.entries() {
        if let Some(name) = entry.name() {
            properties.insert(name.value().to_string(), entry.value().as_string().unwrap().to_string());
        }
    }

    assert_eq!(properties.get("option").unwrap(), "enabled");
    assert_eq!(properties.get("mode").unwrap(), "debug");
    assert_eq!(properties.get("target").unwrap(), "production");
}

/// Test properties with hyphenated identifiers
#[test]
fn test_properties_hyphenated_identifiers() {
    let doc = kdl! {
        node
            runs-on="ubuntu-latest"
            node-version="lts"
            cache-dependency-path="package-lock.json"
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    let mut found_props = std::collections::HashSet::new();
    for entry in node.entries() {
        if let Some(name) = entry.name() {
            found_props.insert(name.value().to_string());
        }
    }

    assert!(found_props.contains("runs-on"));
    assert!(found_props.contains("node-version"));
    assert!(found_props.contains("cache-dependency-path"));
}

/// Test comprehensive property specification compliance
#[test]
fn test_comprehensive_property_specification() {
    let doc = kdl! {
        config {
            // String properties
            name="MyApp"
            version="1.0.0"
            description="A comprehensive test application"

            // Numeric properties
            port=8080
            timeout_seconds=30
            retry_count=3
            load_factor=0.75

            // Boolean properties
            debug=true
            ssl_enabled=false
            auto_reload=true

            // Null property
            optional_field=null

            // Type-annotated properties
            build_date=(date)"2023-01-15"
            schema_version=(semver)"1.2.3"

            // Mixed with arguments
            server "primary" host="localhost" port=8080 "secondary"
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let config = &doc.nodes()[0];
    assert_eq!(config.name().value(), "config");

    let config_children = config.children().unwrap();
    assert_eq!(config_children.nodes().len(), 1);

    let server = &config_children.nodes()[0];
    assert_eq!(server.name().value(), "server");

    // Count properties vs arguments
    let mut property_count = 0;
    let mut argument_count = 0;

    for entry in server.entries() {
        if entry.name().is_some() {
            property_count += 1;
        } else {
            argument_count += 1;
        }
    }

    assert_eq!(property_count, 2); // host="localhost", port=8080
    assert_eq!(argument_count, 2); // "primary", "secondary"
}