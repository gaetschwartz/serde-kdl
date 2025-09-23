use serde_kdl_macro::kdl;


#[test]
fn test_simple_node() {
    let doc = kdl! {
        simple_node 42
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "simple_node");
    assert_eq!(node.entries().len(), 1);
    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 42);
}

#[test]
fn test_string_argument() {
    let doc = kdl! {
        config "my-app"
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "config");
    assert_eq!(node.entries().len(), 1);
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "my-app");
}

#[test]
fn test_boolean_argument() {
    let doc = kdl! {
        debug true
    };

    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "debug");
    assert_eq!(node.entries()[0].value().as_bool().unwrap(), true);
}

#[test]
fn test_float_argument() {
    let doc = kdl! {
        version 1.5
    };

    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "version");
    assert_eq!(node.entries()[0].value().as_f64().unwrap(), 1.5);
}

#[test]
fn test_property() {
    let doc = kdl! {
        server host="localhost"
    };

    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "server");
    assert_eq!(node.entries().len(), 1);

    let entry = &node.entries()[0];
    assert_eq!(entry.name().unwrap().value(), "host");
    assert_eq!(entry.value().as_string().unwrap(), "localhost");
}

#[test]
fn test_property_with_number() {
    let doc = kdl! {
        server port=8080
    };

    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "server");
    assert_eq!(node.entries().len(), 1);

    let entry = &node.entries()[0];
    assert_eq!(entry.name().unwrap().value(), "port");
    assert_eq!(entry.value().as_i64().unwrap(), 8080);
}

#[test]
fn test_empty_children() {
    let doc = kdl! {
        parent {}
    };

    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "parent");
    assert!(node.children().is_some());
    assert_eq!(node.children().unwrap().nodes().len(), 0);
}

#[test]
fn test_single_child() {
    let doc = kdl! {
        parent {
            child "value"
        }
    };

    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");

    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 1);

    let child = &children.nodes()[0];
    assert_eq!(child.name().value(), "child");
    assert_eq!(child.entries()[0].value().as_string().unwrap(), "value");
}