use serde_kdl_macro::kdl;

#[test]
fn test_simple_string_argument() {
    let doc = kdl! {
        node "hello"
    };
    assert_eq!(doc.nodes().len(), 1);
    assert_eq!(doc.nodes()[0].name().value(), "node");
    assert_eq!(doc.nodes()[0].entries().len(), 1);
}

#[test]
fn test_simple_number_argument() {
    let doc = kdl! {
        node 42
    };
    assert_eq!(doc.nodes().len(), 1);
    assert_eq!(doc.nodes()[0].name().value(), "node");
    assert_eq!(doc.nodes()[0].entries().len(), 1);
}

#[test]
fn test_multiple_nodes_with_semicolon() {
    let doc = kdl! {
        node1 "hello";
        node2 42;
    };
    assert_eq!(doc.nodes().len(), 2);
    assert_eq!(doc.nodes()[0].name().value(), "node1");
    assert_eq!(doc.nodes()[1].name().value(), "node2");
}

#[test]
fn test_multiple_arguments() {
    let doc = kdl! {
        node "hello" 42
    };
    assert_eq!(doc.nodes().len(), 1);
    assert_eq!(doc.nodes()[0].name().value(), "node");
    assert_eq!(doc.nodes()[0].entries().len(), 2);
}

#[test]
fn test_keyword_as_identifier() {
    // Test that keywords work in the macro with r# prefix
    let doc = kdl! {
        r#use "serde"
        r#type "String"
        r#async true
    };
    assert_eq!(doc.nodes().len(), 3);
    assert_eq!(doc.nodes()[0].name().value(), "r#use");
    assert_eq!(doc.nodes()[1].name().value(), "r#type");
    assert_eq!(doc.nodes()[2].name().value(), "r#async");

    // Test that regular identifiers still work
    let doc2 = kdl! {
        regular_name "value"
    };
    assert_eq!(doc2.nodes().len(), 1);
    assert_eq!(doc2.nodes()[0].name().value(), "regular_name");
}
