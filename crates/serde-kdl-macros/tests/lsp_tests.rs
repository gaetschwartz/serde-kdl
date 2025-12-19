use serde_kdl_macros::kdl;

fn main() {
    let _ = kdl! {
        node1 prop1=42
        node2 prop2=100
        node3 prop3=212
    };
}
