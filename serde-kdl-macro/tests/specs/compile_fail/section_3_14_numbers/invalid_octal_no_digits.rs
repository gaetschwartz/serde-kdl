use serde_kdl_macro::kdl;

fn main() {
    // Invalid: octal prefix without digits
    let _ = kdl! { node 0o };
}
