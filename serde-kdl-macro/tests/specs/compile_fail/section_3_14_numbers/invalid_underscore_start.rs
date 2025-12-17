use serde_kdl_macro::kdl;

fn main() {
    // Invalid: underscore at start of number
    let _ = kdl! { node _123 };
}
