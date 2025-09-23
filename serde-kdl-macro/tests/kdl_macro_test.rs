use serde_kdl_macro::kdl;

#[test]
fn test_basic_macro_functionality() {
    // Test empty document
    let empty_doc = kdl! {};
    assert!(empty_doc.nodes().is_empty());

    // Test simple node
    let simple_doc = kdl! {
        node
    };
    assert_eq!(simple_doc.nodes().len(), 1);
    assert_eq!(simple_doc.nodes()[0].name().value(), "node");

    // Test node with arguments
    let args_doc = kdl! {
        node 42 "hello" true
    };
    assert_eq!(args_doc.nodes().len(), 1);
    let node = &args_doc.nodes()[0];
    assert_eq!(node.name().value(), "node");
    assert_eq!(node.entries().len(), 3);

    // Test node with properties
    let props_doc = kdl! {
        node key="value" count=42
    };
    assert_eq!(props_doc.nodes().len(), 1);
    let node = &props_doc.nodes()[0];
    assert_eq!(node.name().value(), "node");
    assert_eq!(node.entries().len(), 2);

    // Test mixed arguments and properties
    let mixed_doc = kdl! {
        node 42 key="value" "string"
    };
    assert_eq!(mixed_doc.nodes().len(), 1);
    let node = &mixed_doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    // Test nested children
    let nested_doc = kdl! {
        parent {
            child1 "value"
            child2 count=10
        }
    };
    assert_eq!(nested_doc.nodes().len(), 1);
    let parent = &nested_doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 2);
    assert_eq!(children.nodes()[0].name().value(), "child1");
    assert_eq!(children.nodes()[1].name().value(), "child2");

    // Test multiple root nodes
    let multi_doc = kdl! {
        node1 "value1"
        node2 count=42
        node3 {
            child "nested"
        }
    };
    assert_eq!(multi_doc.nodes().len(), 3);
    assert_eq!(multi_doc.nodes()[0].name().value(), "node1");
    assert_eq!(multi_doc.nodes()[1].name().value(), "node2");
    assert_eq!(multi_doc.nodes()[2].name().value(), "node3");
}

#[test]
fn test_complex_nested_structure() {
    let doc = kdl! {
        config {
            name "my-app"
            version "1.0.0"
            debug true
            server port=8080 host="localhost" {
                database {
                    url "postgresql://localhost/mydb"
                    pool_size 10
                    ssl true
                }
                cache {
                    cache_type "redis"
                    ttl 3600
                }
            }
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let config = &doc.nodes()[0];
    assert_eq!(config.name().value(), "config");

    let config_children = config.children().unwrap();
    assert_eq!(config_children.nodes().len(), 4); // name, version, debug, server

    // Find the server node
    let server = config_children
        .nodes()
        .iter()
        .find(|n| n.name().value() == "server")
        .unwrap();

    let server_children = server.children().unwrap();
    assert_eq!(server_children.nodes().len(), 2); // database, cache
}

#[test]
fn test_value_types() {
    let doc = kdl! {
        types {
            integers 42 -10 0
            floats 3.14 -2.5 0.0
            booleans true false
            strings "hello" "world"
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let types_node = &doc.nodes()[0];
    let children = types_node.children().unwrap();
    assert_eq!(children.nodes().len(), 4);
}
