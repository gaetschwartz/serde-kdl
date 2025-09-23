use serde_kdl_macro::kdl;

/// Test basic document structure (section 3.1)
#[test]
fn test_basic_document_structure() {
    // Empty document
    let doc = kdl! {};
    assert_eq!(doc.nodes().len(), 0);

    // Single node
    let doc = kdl! { node };
    assert_eq!(doc.nodes().len(), 1);
    assert_eq!(doc.nodes()[0].name().value(), "node");

    // Multiple nodes
    let doc = kdl! {
        node1
        node2
        node3
    };
    println!("Document nodes: {:?}", doc.nodes().iter().map(|n| n.name().value()).collect::<Vec<_>>());
    assert_eq!(doc.nodes().len(), 3);
}

/// Test node arguments (section 3.5)
#[test]
fn test_node_arguments() {
    // String argument
    let doc = kdl! { node "string_value" };
    let entry = &doc.nodes()[0].entries()[0];
    assert_eq!(entry.value().as_string().unwrap(), "string_value");

    // Integer argument
    let doc = kdl! { node 42 };
    let entry = &doc.nodes()[0].entries()[0];
    assert_eq!(entry.value().as_i64().unwrap(), 42);

    // Float argument
    let doc = kdl! { node 2.5 };
    let entry = &doc.nodes()[0].entries()[0];
    assert_eq!(entry.value().as_f64().unwrap(), 2.5);

    // Boolean arguments
    let doc = kdl! { node true false };
    assert!(doc.nodes()[0].entries()[0].value().as_bool().unwrap());
    assert!(!doc.nodes()[0].entries()[1].value().as_bool().unwrap());

    // Multiple arguments
    let doc = kdl! { node "arg1" 42 true };
    assert_eq!(doc.nodes()[0].entries().len(), 3);
}

/// Test node properties (section 3.4)
#[test]
fn test_node_properties() {
    // Single property
    let doc = kdl! { node key="value" };
    let entry = &doc.nodes()[0].entries()[0];
    assert_eq!(entry.name().unwrap().value(), "key");
    assert_eq!(entry.value().as_string().unwrap(), "value");

    // Multiple properties
    let doc = kdl! { node key1="value1" key2=42 key3=true };
    assert_eq!(doc.nodes()[0].entries().len(), 3);

    // Mixed arguments and properties
    let doc = kdl! { node "arg1" key="value" "arg2" };
    assert_eq!(doc.nodes()[0].entries().len(), 3);
}

/// Test children blocks (section 3.6)
#[test]
fn test_children_blocks() {
    let doc = kdl! {
        parent {
            child1 "value1"
            child2 key="value2"
        }
    };

    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");

    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 2);
    assert_eq!(children.nodes()[0].name().value(), "child1");
    assert_eq!(children.nodes()[1].name().value(), "child2");
}

/// Test boolean values (section 3.15)
#[test]
fn test_boolean_values() {
    // Basic boolean values
    let doc = kdl! { node true false };
    assert!(doc.nodes()[0].entries()[0].value().as_bool().unwrap());
    assert!(!doc.nodes()[0].entries()[1].value().as_bool().unwrap());
}

/// Test null values (section 3.16)
#[test]
fn test_null_values() {
    let doc = kdl! { node null };
    assert!(doc.nodes()[0].entries()[0].value().is_null());
}

/// Test type annotations (section 3.8)
#[test]
fn test_type_annotations() {
    // FIXME: Type annotations need to be implemented properly
    // let doc = kdl! { node (string)"value" };
    // let entry = &doc.nodes()[0].entries()[0];
    // assert_eq!(entry.value().as_string().unwrap(), "value");
}

/// Test hyphenated identifiers
#[test]
fn test_hyphenated_identifiers() {
    let doc = kdl! { node-name "value" };
    assert_eq!(doc.nodes()[0].name().value(), "node-name");
}

/// Test negative numbers
#[test]
fn test_negative_numbers() {
    let doc = kdl! { node -42 -2.5 };
    assert_eq!(doc.nodes()[0].entries()[0].value().as_i64().unwrap(), -42);
    assert_eq!(doc.nodes()[0].entries()[1].value().as_f64().unwrap(), -2.5);
}

/// Test complex nested structures
#[test]
fn test_complex_nesting() {
    let doc = kdl! {
        config app="my-app" {
            database {
                host "localhost"
                port 5432
                ssl true
            }
            cache enabled=true ttl=3600
        }
        logging level="info"
    };

    assert_eq!(doc.nodes().len(), 2);
    assert_eq!(doc.nodes()[0].name().value(), "config");
    assert_eq!(doc.nodes()[1].name().value(), "logging");

    let config_children = doc.nodes()[0].children().unwrap();
    assert_eq!(config_children.nodes().len(), 2);
}

/// Test edge cases and boundary conditions
#[test]
fn test_edge_cases() {
    // Empty children block
    let doc = kdl! { node {} };
    assert!(doc.nodes()[0].children().is_some());
    assert_eq!(doc.nodes()[0].children().unwrap().nodes().len(), 0);

    // Node with only properties, no arguments
    let doc = kdl! { node key1="value1" key2="value2" };
    assert_eq!(doc.nodes()[0].entries().len(), 2);

    // Node with only arguments, no properties
    let doc = kdl! { node "arg1" "arg2" "arg3" };
    assert_eq!(doc.nodes()[0].entries().len(), 3);
}