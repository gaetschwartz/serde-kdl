use serde_kdl_macro::kdl;

fn main() {
    // Invalid: exponent without digits
    let _ = kdl! { node 1e };
}
