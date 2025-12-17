use serde_kdl_macro::kdl;

fn main() {
    // Invalid: binary with digit >= 2
    let _ = kdl! { node 0b102 };
}
