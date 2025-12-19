use serde_kdl_macros::kdl;

fn main() {
    // Invalid: binary with digit >= 2
    let _ = kdl! { node 0b102 };
}
