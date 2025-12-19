use serde_kdl_macros::kdl;

fn main() {
    // Invalid: underscore at start of number
    let _ = kdl! { node _123 };
}
