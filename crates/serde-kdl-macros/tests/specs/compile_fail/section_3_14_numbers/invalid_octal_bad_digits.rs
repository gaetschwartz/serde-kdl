use serde_kdl_macros::kdl;

fn main() {
    // Invalid: octal with digit >= 8
    let _ = kdl! { node 0o89 };
}
