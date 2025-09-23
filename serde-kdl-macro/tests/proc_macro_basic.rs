use serde_kdl_macro::kdl;

#[test]
fn test_proc_macro_empty_document() {
    let doc = kdl! {};
    assert_eq!(doc.nodes().len(), 0);
}

#[test]
fn test_proc_macro_simple_node() {
    let doc = kdl! {
        simple
    };
    assert_eq!(doc.nodes().len(), 1);
    assert_eq!(doc.nodes()[0].name().value(), "simple");
}

#[test]
fn test_proc_macro_node_with_string() {
    let doc = kdl! {
        config "my-app"
    };
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "config");
    assert_eq!(node.entries().len(), 1);
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "my-app");
}

#[test]
fn test_proc_macro_node_with_integer() {
    let doc = kdl! {
        port 8080
    };
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "port");
    assert_eq!(node.entries().len(), 1);
    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 8080);
}

#[test]
fn test_proc_macro_node_with_boolean() {
    let doc = kdl! {
        debug true
    };
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "debug");
    assert_eq!(node.entries().len(), 1);
    assert_eq!(node.entries()[0].value().as_bool().unwrap(), true);
}

#[test]
fn test_proc_macro_node_with_property() {
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
fn test_proc_macro_node_with_children() {
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
