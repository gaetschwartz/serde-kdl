use serde_kdl_macro::kdl;

#[test]
fn test_simple_node() {
    let doc = kdl! {
        node
    };
    assert_eq!(doc.nodes().len(), 1);
    assert_eq!(doc.nodes()[0].name().value(), "node");
}

#[test]
fn test_node_with_argument() {
    let doc = kdl! {
        node 42
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "node");
    assert_eq!(node.entries().len(), 1);
}

#[test]
fn test_node_with_property() {
    let doc = kdl! {
        node key="value"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "node");
    assert_eq!(node.entries().len(), 1);
}
