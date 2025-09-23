use serde_kdl_macro::kdl;

fn main() {
    // Example demonstrating Section 3.8 Type Annotation support
    let doc = kdl! {
        // Type annotations on node names (3.8.4 examples)
        (published)date "1970-01-01"
        (contributor)person name="Foo McBar"

        // Type annotation on argument
        node (u8)255

        // Type annotation on property value
        node2 prop=(string)"hello"

        // Nested nodes with type annotations
        parent {
            (child-type)child "value"
        }
    };

    println!("Generated KDL document: {:#?}", doc);

    // The macro generates proper KdlDocument with type annotations
    println!("\nDocument has {} nodes", doc.nodes().len());

    // Type annotations are preserved in the generated structure
    for node in doc.nodes() {
        println!(
            "Node '{}' has type annotation: {:?}",
            node.name(),
            node.ty()
        );

        for entry in node.entries() {
            if let Some(ty) = entry.ty() {
                println!("  Entry has type annotation: {}", ty);
            }
        }
    }
}
