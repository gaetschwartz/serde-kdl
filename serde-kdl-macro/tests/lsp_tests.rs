use serde_kdl_macro::kdl;

fn main() {
    kdl! {
        node1 prop1=42
        node2 prop2=100
    };
}
