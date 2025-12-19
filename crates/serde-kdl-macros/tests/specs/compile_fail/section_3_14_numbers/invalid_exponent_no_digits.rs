use serde_kdl_macros::kdl;

fn main() {
    // Invalid: exponent without digits
    let _ = kdl! { node 1e };
}
